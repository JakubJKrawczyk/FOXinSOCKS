use std::io::Write;
use std::path::Path;
use std::fs::File;
use crate::data::data::constants;
use crate::models::task_model::TaskModel;

// Inicjalizacja pliku z zadaniami
pub fn init() -> Result<String, String>{
    let file_path = Path::new(constants::TASKS_FILE_NAME);
    if file_path.exists(){
        Ok(String::from("Znaleziono plik z taskami"))
    }else{
        let mut file = File::create(file_path).map_err(|e| e.to_string())?;
        file.write_all(b"[]").map_err(|e| e.to_string())?;
        file.flush().map_err(|e| e.to_string())?;
        Err(String::from("Utworzono nowy plik z taskami."))
    }
}

pub fn get_tasks() -> Vec<TaskModel>{
    let file_path = Path::new(constants::TASKS_FILE_NAME);
    if file_path.exists() {
        let file_content = std::fs::read_to_string(file_path).unwrap_or_else(|_| "[]".into());
        serde_json::from_str(&file_content).unwrap_or_default()
    }else{
        Vec::new()
    }
}

pub fn save_tasks(tasks: &[TaskModel]) -> Result<(), String>{
    let file_path = Path::new(constants::TASKS_FILE_NAME);
    let json = serde_json::to_string_pretty(&tasks).map_err(|e| e.to_string())?;
    std::fs::write(file_path, json).map_err(|e| e.to_string())
}