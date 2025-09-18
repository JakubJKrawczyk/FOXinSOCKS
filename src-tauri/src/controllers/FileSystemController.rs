
pub mod file_system_controller{
    use std::io::Write;
    use std::path::Path;
    use std::fs::File;
    use crate::data::data::constants;

    use crate::models::task_model::TaskModel;
    pub fn init() -> Result<String, String>{
        let file_path = Path::new(constants::TASKS_FILE_NAME);
        if file_path.exists(){
            Ok(String::from("Znaleziono plik z taskami"))
        }else{
            let mut file = File::create_new(file_path).unwrap();
            file.write_all(b"[]").unwrap();
            file.flush().unwrap();
            Err(String::from("Utworzono nowy plik z taskami."))
        }
    }

    pub fn get_tasks() -> Vec<TaskModel>{
        let file_path = Path::new(constants::TASKS_FILE_NAME);
        if file_path.exists() {
            let file_content = std::fs::read_to_string(file_path).unwrap();
            let tasks: Vec<TaskModel> = serde_json::from_str(&file_content).unwrap();
            tasks
        }else{
            Vec::new()
        }
    }

    pub fn save_tasks(tasks: Vec<TaskModel>) -> Result<(), String>{
        let file_path = Path::new(constants::TASKS_FILE_NAME);

        let mut stream = File::open(file_path).unwrap();

        let json = serde_json::to_string_pretty(&tasks).unwrap();

        let result = stream.write_all(json.as_bytes());

        if result.is_err(){
            Err(format!("Błąd zapisu struktury do pliku. {}", result.err().unwrap().to_string()))
        }else{
            Ok(())
        }
    }

}