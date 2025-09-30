use std::io::Write;
use std::path::PathBuf;
use std::fs::{File, create_dir_all};
use crate::data::data::constants;
use crate::models::task_model::TaskModel;

fn saves_dir() -> PathBuf {
    // katalog obok exe + /saves
    let base = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let dir = base.join("saves");
    let _ = create_dir_all(&dir);
    dir
}

fn tasks_file_path() -> PathBuf { saves_dir().join(constants::TASKS_FILE_NAME) }

// Inicjalizacja pliku z zadaniami w katalogu saves
pub fn init() -> Result<String, String>{
    let file_path = tasks_file_path();
    if file_path.exists(){
        Ok(String::from("Znaleziono plik z taskami"))
    }else{
        let mut file = File::create(&file_path).map_err(|e| e.to_string())?;
        file.write_all(b"[]").map_err(|e| e.to_string())?;
        file.flush().map_err(|e| e.to_string())?;
        Err(String::from("Utworzono nowy plik z taskami."))
    }
}

pub fn get_tasks() -> Vec<TaskModel>{
    let file_path = tasks_file_path();
    if file_path.exists() {
        let file_content = std::fs::read_to_string(&file_path).unwrap_or_else(|_| "[]".into());
        serde_json::from_str(&file_content).unwrap_or_default()
    }else{ Vec::new() }
}

pub fn save_tasks(tasks: &[TaskModel]) -> Result<(), String>{
    let file_path = tasks_file_path();
    let json = serde_json::to_string_pretty(&tasks).map_err(|e| e.to_string())?;
    std::fs::write(file_path, json).map_err(|e| e.to_string())
}