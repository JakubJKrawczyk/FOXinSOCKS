use tokio::task;

use crate::models::task_model::TaskModel;
use crate::models::task_process_model::TaskProcessModel;
use crate::controllers::FileSystemController;
use std::{path::Path};
use regex::Regex;
use std::collections::HashMap;
use walkdir::WalkDir;
use std::fs::{self};
use tokio::time::{sleep, Duration};

pub struct clean_process_controller{
    pub tasks_processes: Vec<TaskProcessModel>,
    pub tasks : Vec<TaskModel>
}

impl clean_process_controller{
    pub fn new() -> Self {
        let _ = FileSystemController::file_system_controller::init();
        let tasks = FileSystemController::file_system_controller::get_tasks();

        clean_process_controller {
            tasks: tasks,
            tasks_processes: Vec::new()
        }
    }

    pub fn get_task(&self, id: String) -> Option<&TaskModel>
    {
        self.tasks.iter().find(|task| task.id == id)
    }

    pub fn update_task(&mut self, new_task: TaskModel) -> Result<String, String>{
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == new_task.id){
            *task = new_task;
            Ok(String::from("Task zaktualizowany"))
        }else{
            Err(String::from("Nie znaleziono taska"))
        }
    }

    pub fn delete_task(&mut self, task_id: String) -> Result<String, String>{
        if let Some(pos) = self.tasks.iter().position(|task| task.id == task_id){
            self.tasks.remove(pos);
            Ok(String::from("Udało się usunąć taska"))
        }else{
            Err(String::from("Nie znaleziono taska"))
        }
    }

    pub fn add_task(&mut self, task: TaskModel) -> Result<TaskModel, String>{
        if self.tasks.iter().any(|t| t.id == task.id) == false{

            self.tasks.push(task.clone());
            Ok(task)
        }else{
            Err(String::from("W systemie istnieje już task o takim Id"))
        }
    }

    pub fn create_process(&mut self, task_model: TaskModel) -> Result<(), String>{
        if self.tasks_processes.iter().any(|task_process| task_process.task_id == task_model.id) == false{
            let new_entry = TaskProcessModel::new(
                task::spawn(
                    cleaning_command(
                        task_model.regex_patterns,
                        task_model.folder_paths,
                        task_model.auto_run_interval,
                        task_model.number_of_dup_to_keep
                    )
                ),
                task_model.id
            );
                self.tasks_processes.push(new_entry);
                Ok(())
        }else{
            Err(String::from("Nie udało się utworzyć procesu dla zadania!"))
        }
    }

    pub fn stop_process(&self, task_id: String) -> Result<(), String>{
        if let Some(task_process) = self.tasks_processes.iter().find(|task| task.task_id == task_id){
            if task_process.process.is_finished() == false {
                task_process.process.abort();
            }
            Ok(())
        }else{
            Err(String::from("Błąd podczas zatrzymywania procesu"))
        }
    }
}


pub async fn cleaning_command(regex_patterns: Vec<String>, folder_paths: Vec<String>, interval: u32, dup_to_leave: u8) -> String {

    
    loop {
        for folder in &folder_paths {
            let folder_path = Path::new(folder);

            if !folder_path.exists() || !folder_path.is_dir() {
                continue;
            }

            for regex_str in &regex_patterns {
                if let Ok(re) = Regex::new(regex_str) {
                    // grupowanie plików wg nazwy (np. same base name)
                    let mut file_groups: HashMap<String, Vec<(String, std::time::SystemTime)>> = HashMap::new();

                    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
                        if entry.file_type().is_file() {
                            let file_name = entry.file_name().to_string_lossy().to_string();
                            if re.is_match(&file_name) {
                                if let Ok(metadata) = entry.metadata() {
                                    if let Ok(modified) = metadata.modified() {
                                        file_groups.entry(file_name.clone())
                                            .or_default()
                                            .push((entry.path().to_string_lossy().to_string(), modified));
                                    }
                                }
                            }
                        }
                    }

                    // sprzątanie w każdej grupie
                    for (_name, mut files) in file_groups {
                        // sortujemy po dacie modyfikacji (najnowsze ostatnie)
                        files.sort_by_key(|f| f.1);

                        // jeśli za dużo plików, usuwamy najstarsze
                        if files.len() > dup_to_leave as usize {
                            let to_delete = files.len() - dup_to_leave as usize;
                            for (path, _) in files.into_iter().take(to_delete) {
                                let _ = fs::remove_file(path);
                            }
                        }
                    }
                }
            }
        }

        sleep(Duration::from_secs(interval as u64)).await;
    }


}              