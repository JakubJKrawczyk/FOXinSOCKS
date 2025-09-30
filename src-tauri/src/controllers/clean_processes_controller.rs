use tokio::task;
use crate::models::task_model::{TaskModel, TaskStatus};
use crate::models::task_process_model::TaskProcessModel;
use crate::controllers::file_system_controller;
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;
use walkdir::WalkDir;
use std::fs;
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
    loop {
        let folder_path_path = Path::new(&folder_path);
        if !folder_path_path.exists() || !folder_path_path.is_dir() {
            sleep(Duration::from_secs(interval as u64)).await;
            continue;
        }
        for regex_str in &regex_patterns {
            let re = match Regex::new(regex_str) { Ok(r) => r, Err(_) => continue };
            let mut file_groups: HashMap<String, Vec<(String, std::time::SystemTime)>> = HashMap::new();
            for entry in WalkDir::new(folder_path_path).into_iter().filter_map(|e| e.ok()) {
                if !entry.file_type().is_file(){ continue; }
                let file_name = entry.file_name().to_string_lossy().to_string();
                if re.is_match(&file_name) {
                    if let Ok(metadata) = entry.metadata() { if let Ok(modified) = metadata.modified() {
                        file_groups.entry(file_name.clone()).or_default().push((entry.path().to_string_lossy().to_string(), modified));
                    }}
                }
            }
            for (_name, mut files) in file_groups { // cleanup
                files.sort_by_key(|f| f.1);
                if files.len() > dup_to_leave as usize {
                    let to_delete = files.len() - dup_to_leave as usize;
                    for (path, _) in files.into_iter().take(to_delete) { let _ = fs::remove_file(path); }
                }
            }
        }
        sleep(Duration::from_secs(interval as u64)).await;
    }
}