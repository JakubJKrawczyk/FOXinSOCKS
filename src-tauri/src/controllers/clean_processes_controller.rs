use crate::controllers::file_system_controller;
use crate::models::task_model::{TaskModel, TaskStatus};
use crate::models::task_process_model::TaskProcessModel;
use crate::utills::logger;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use tokio::task;
use tokio::time::{sleep, Duration};

pub struct CleanProcessController {
    pub tasks_processes: Vec<TaskProcessModel>,
    pub tasks: Vec<TaskModel>,
}

impl CleanProcessController {
    pub fn new() -> Self {
        let _ = file_system_controller::init();
        let tasks = file_system_controller::get_tasks();
        CleanProcessController {
            tasks,
            tasks_processes: Vec::new(),
        }
    }

    pub fn get_task(&self, id: &str) -> Option<&TaskModel> {
        self.tasks.iter().find(|task| task.id == id)
    }

    pub fn update_task(&mut self, new_task: &TaskModel) -> Result<String, String> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == new_task.id) {
            *task = new_task.clone();
            Ok(String::from("Task zaktualizowany"))
        } else {
            Err(String::from("Nie znaleziono taska"))
        }
    }

    pub fn delete_task(&mut self, task_id: &str) -> Result<String, String> {
        if let Some(pos) = self.tasks.iter().position(|task| task.id == task_id) {
            self.tasks.remove(pos);
            Ok(String::from("Udało się usunąć taska"))
        } else {
            Err(String::from("Nie znaleziono taska"))
        }
    }

    pub fn add_task(&mut self, task: &TaskModel) -> Result<TaskModel, String> {
        if !self.tasks.iter().any(|t| t.id == task.id) {
            self.tasks.push(task.clone());
            Ok(task.clone())
        } else {
            Err(String::from("W systemie istnieje już task o takim Id"))
        }
    }

    pub fn create_process(&mut self, task_model: &mut TaskModel) -> Result<(), String> {
        if !self
            .tasks_processes
            .iter()
            .any(|tp| tp.task_id == task_model.id)
        {
            let new_entry = TaskProcessModel::new(
                task::spawn(cleaning_command(
                    task_model.regex_patterns.clone(),
                    task_model.folder_path.clone(),
                    task_model.auto_run_interval,
                    task_model.number_of_dup_to_keep,
                )),
                task_model.id.clone(),
            );
            self.tasks_processes.push(new_entry);
            task_model.status = TaskStatus::Sheduled;
            self.update_task(task_model).ok();
            Ok(())
        } else {
            Err(String::from("Task znajduje się już w procesach"))
        }
    }

    pub fn stop_process(&mut self, task_id: &str) -> Result<(), String> {
        if let Some(task_process) = self
            .tasks_processes
            .iter()
            .find(|task| task.task_id == task_id)
        {
            if !task_process.process.is_finished() {
                task_process.process.abort();
            }
            self.tasks_processes.retain(|tp| tp.task_id != task_id);
            Ok(())
        } else {
            Err(String::from("Błąd podczas zatrzymywania procesu"))
        }
    }
}

async fn cleaning_command(
    regex_patterns: Vec<String>,
    folder_path: String,
    interval: u32,
    dup_to_leave: u8,
) {
    let sleep_duration = Duration::from_secs(interval as u64 * 60);
    loop {
        let base_path = Path::new(&folder_path);
        if !base_path.exists() || !base_path.is_dir() {
            logger::process(
                "Ścieżka do folderu niepoprawna – wstrzymanie do następnego interwału.",
            );
            sleep(sleep_duration).await;
            continue;
        }
        // 1. Bufor wszystkich plików (nazwa + mtime)
        #[derive(Clone)]
        struct FileInfo {
            name: String,
            path: String,
            modified: SystemTime,
        }
        let mut all: Vec<FileInfo> = Vec::new();
        for e in fs::read_dir(base_path).unwrap_or_else(|_| fs::read_dir(base_path).unwrap()) {
            if let Ok(entry) = e {
                if let Ok(ft) = entry.file_type() {
                    if !ft.is_file() {
                        continue;
                    }
                }
                let name = match entry.file_name().to_str() {
                    Some(s) => s.to_string(),
                    None => continue,
                };
                let meta = entry.metadata().ok();
                let mtime = meta
                    .and_then(|m| m.modified().ok())
                    .unwrap_or(SystemTime::UNIX_EPOCH);
                all.push(FileInfo {
                    name: name.clone(),
                    path: entry.path().to_string_lossy().to_string(),
                    modified: mtime,
                });
            }
        }
        logger::process(format!("[CLEAN] Zebrano {} plików", all.len()));

        // 2. Kompilacja regexów
        let compiled: Vec<(String, Regex)> = regex_patterns
            .iter()
            .filter_map(|p| match Regex::new(p) {
                Ok(r) => Some((p.clone(), r)),
                Err(e) => {
                    logger::warning(format!("Błędny regex '{}': {}", p, e));
                    None
                }
            })
            .collect();
        logger::process(format!("[CLEAN] Aktywnych regexów: {}", compiled.len()));

        // 3. Dopasowania -> klucze
        let mut keys: HashSet<String> = HashSet::new();
        for (pat, re) in &compiled {
            let mut matched_count = 0usize;
            let mut new_keys = 0usize;
            for f in &all {
                if let Some(caps) = re.captures(&f.name) {
                    matched_count += 1;
                    let key = if caps.len() > 1 {
                        caps.get(1).unwrap().as_str().to_string()
                    } else {
                        re.find(&f.name)
                            .map(|m| m.as_str().to_string())
                            .unwrap_or_else(|| f.name.clone())
                    };
                    if key.len() >= 2 && keys.insert(key.clone()) {
                        new_keys += 1;
                    }
                }
            }
            logger::process(format!(
                "Regex '{}' dopasował {} plików (nowe klucze: {})",
                pat, matched_count, new_keys
            ));
        }
        logger::process(format!("[CLEAN] Łącznie unikalnych kluczy: {}", keys.len()));

        // 4. Dla każdego klucza policz pliki zawierające substring, usuń nadmiar
        let mut total_deleted = 0usize;
        for key in &keys {
            let mut set: Vec<&FileInfo> = all.iter().filter(|f| f.name.contains(key)).collect();
            if set.len() <= dup_to_leave as usize {
                continue;
            }
            // sort descending (najnowsze na początku)
            set.sort_by_key(|f| f.modified);
            set.reverse();
            let keep = dup_to_leave as usize;
            let to_remove = &set[keep..];
            logger::process(format!(
                "Klucz '{}' -> plików={} zachowuję={} usuwam={}",
                key,
                set.len(),
                keep,
                to_remove.len()
            ));
            for fi in to_remove {
                match fs::remove_file(&fi.path) {
                    Ok(_) => {
                        total_deleted += 1;
                        logger::process(format!("[{}] Usunięto {}", key, fi.path));
                    }
                    Err(e) => {
                        logger::warning(format!("[{}] Błąd usuwania {}: {}", key, fi.path, e))
                    }
                }
            }
        }
        logger::process(format!(
            "[CLEAN] Podsumowanie: plików={} kluczy={} usunięto={} pozostawiono={}",
            all.len(),
            keys.len(),
            total_deleted,
            all.len().saturating_sub(total_deleted)
        ));
        logger::process("Czyszczenie zakończone – oczekiwanie do następnego interwału.");
        sleep(sleep_duration).await;
    }
}
