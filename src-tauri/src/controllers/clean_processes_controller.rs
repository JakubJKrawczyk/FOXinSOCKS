use tokio::task;
use crate::models::task_model::{TaskModel, TaskStatus};
use crate::models::task_process_model::TaskProcessModel;
use crate::controllers::file_system_controller;
use crate::utills::logger;
use std::path::Path;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::time::SystemTime;
use tokio::time::{sleep, Duration};

pub struct CleanProcessController{
    pub tasks_processes: Vec<TaskProcessModel>,
    pub tasks : Vec<TaskModel>
}

impl CleanProcessController{
    pub fn new() -> Self {
        let _ = file_system_controller::init();
        let tasks = file_system_controller::get_tasks();
        CleanProcessController { tasks, tasks_processes: Vec::new() }
    }

    pub fn get_task(&self, id: &str) -> Option<&TaskModel>{
        self.tasks.iter().find(|task| task.id == id)
    }

    pub fn update_task(&mut self, new_task: &TaskModel) -> Result<String, String>{
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == new_task.id){
            *task = new_task.clone();
            Ok(String::from("Task zaktualizowany"))
        }else{ Err(String::from("Nie znaleziono taska")) }
    }

    pub fn delete_task(&mut self, task_id: &str) -> Result<String, String>{
        if let Some(pos) = self.tasks.iter().position(|task| task.id == task_id){
            self.tasks.remove(pos);
            Ok(String::from("Udało się usunąć taska"))
        }else{ Err(String::from("Nie znaleziono taska")) }
    }

    pub fn add_task(&mut self, task: &TaskModel) -> Result<TaskModel, String>{
        if !self.tasks.iter().any(|t| t.id == task.id){
            self.tasks.push(task.clone());
            Ok(task.clone())
        }else{ Err(String::from("W systemie istnieje już task o takim Id")) }
    }

    pub fn create_process(&mut self, task_model: &mut TaskModel) -> Result<(), String>{
        if !self.tasks_processes.iter().any(|tp| tp.task_id == task_model.id){

            let new_entry = TaskProcessModel::new(
                task::spawn(cleaning_command(
                    task_model.regex_patterns.clone(),
                    task_model.folder_path.clone(),
                    task_model.auto_run_interval,
                    task_model.number_of_dup_to_keep
                )),
                task_model.id.clone()
            );
            self.tasks_processes.push(new_entry);
            task_model.status = TaskStatus::Sheduled;
            self.update_task(task_model).ok();
            Ok(())
        }else{ Err(String::from("Task znajduje się już w procesach")) }
    }

    pub fn stop_process(&mut self, task_id: &str) -> Result<(), String>{
        if let Some(task_process) = self.tasks_processes.iter().find(|task| task.task_id == task_id){
            if !task_process.process.is_finished() { task_process.process.abort(); }
            self.tasks_processes.retain(|tp| tp.task_id != task_id);
            Ok(())
        }else{ Err(String::from("Błąd podczas zatrzymywania procesu")) }
    }
}

async fn cleaning_command(regex_patterns: Vec<String>, folder_path: String, interval: u32, dup_to_leave: u8){
    // Interpretacja: interval w minutach (spójnie) – w przypadku błędnej ścieżki też czekamy pełny interwał.
    let sleep_duration = Duration::from_secs(interval as u64 * 60);

    // Prekompilacja regexów (pomijamy błędne)
    let compiled_patterns: Vec<(String, Regex)> = regex_patterns.iter()
        .filter_map(|p| Regex::new(p).ok().map(|r| (p.clone(), r)))
        .collect();

    loop {
        let base_path = Path::new(&folder_path);
        if !base_path.exists() || !base_path.is_dir(){
            logger::process("Ścieżka do folderu jest niepoprawna, folder nie istnieje lub nie jest to folder. Wstrzymanie do następnej próby.");
            sleep(sleep_duration).await;
            continue;
        }

        logger::process(format!("Start czyszczenia – {} wzorców", compiled_patterns.len()));

        // 1. Zbierz unikalne "rodziny" plików (jak w GetUniqueFilesNames z C#).
        let mut families: HashSet<String> = HashSet::new();
        let dir_iter = match fs::read_dir(base_path){ Ok(it) => it, Err(_) => { sleep(sleep_duration).await; continue; } };

        // Zebrane nazwy plików (przyda się ponownie)
        let mut entries_cache: Vec<(String, SystemTime)> = Vec::new();

        for entry_res in dir_iter {
            if let Ok(entry) = entry_res {
                if let Ok(ft) = entry.file_type() { if !ft.is_file() { continue; } }
                let path_buf = entry.path();
                let file_name = match path_buf.file_name().and_then(|n| n.to_str()) { Some(n) => n.to_string(), None => continue };
                // Precache czasy (creation -> fallback modified)
                let ctime = entry.metadata().ok()
                    .and_then(|md| md.created().or_else(|_| md.modified()).ok())
                    .unwrap_or(SystemTime::UNIX_EPOCH);
                entries_cache.push((file_name.clone(), ctime));

                for (pattern_str, re) in &compiled_patterns {
                    if let Some(m) = re.find(&file_name) { // dopasowanie w samej nazwie
                        let mut family_key = if pattern_str == "^.*_\\d{2}\\." {
                            let matched = &file_name[m.start()..m.end()];
                            if matched.len() > 3 { matched[..matched.len()-3].to_string() } else { matched.to_string() }
                        } else {
                            // pełna nazwa pliku jako rodzina (odwzorowanie pierwotnej logiki C#)
                            file_name.clone()
                        };
                        if family_key.is_empty() { family_key = file_name.clone(); }
                        families.insert(family_key);
                    }
                }
            }
        }

        logger::process(format!("Zidentyfikowano {} rodzin plików", families.len()));

        // 2. Dla każdej rodziny zbierz pasujące pliki (contains) i usuń starsze zostawiając dup_to_leave najnowszych.
        for family in families.into_iter() {
            // Zbierz pliki pasujące (contains)
            let mut family_files: Vec<(String, SystemTime)> = entries_cache.iter()
                .filter(|(name, _)| name.contains(&family))
                .map(|(n, t)| (n.clone(), *t))
                .collect();

            if family_files.len() as u8 <= dup_to_leave { continue; }

            // Sortuj od najnowszych do najstarszych (jak C# po reverse)
            family_files.sort_by_key(|(_, t)| *t); // rosnąco (najstarsze pierwsze)
            family_files.reverse(); // teraz najnowsze pierwsze

            let to_delete = family_files.len().saturating_sub(dup_to_leave as usize);
            if to_delete == 0 { continue; }
            logger::process(format!("[{}] Plików: {}, do usunięcia: {}", family, family_files.len(), to_delete));

            for (idx, _) in family_files.iter().enumerate() {
                if idx >= to_delete { break; }
            }
            // Po odwróceniu najnowsze są na początku – chcemy zostawić dup_to_leave pierwszych, więc kasujemy od indeksu dup_to_leave wzwyż.
            for (file_name, _) in family_files.iter().skip(dup_to_leave as usize) {
                let full_path = base_path.join(file_name);
                logger::process(format!("[{}] Usuwanie pliku {}", family, full_path.to_string_lossy()));
                let _ = fs::remove_file(full_path);
            }
        }

        logger::process("Czyszczenie zakończone – oczekiwanie do następnego interwału.");
        sleep(sleep_duration).await;
    }
}