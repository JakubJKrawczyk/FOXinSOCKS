use crate::controllers::CleanProcessesControler::clean_process_controller;
use std::os::windows::process;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use crate::models::task_model::TaskModel;
pub static controller: OnceLock<Mutex<clean_process_controller>> = OnceLock::new();
pub static initialized: AtomicBool = AtomicBool::new(false);


#[tauri::command]
pub async fn init() -> Result<(), String> {
    if initialized.load(Ordering::SeqCst) {
        let _ = controller.set(Mutex::new(clean_process_controller::new()));
        initialized.store(true, Ordering::SeqCst);
        Ok(())
    } else {
        Err(String::from("Rust jest już zainicjalizowany"))
    }
}

#[tauri::command]
pub async fn get_tasks() -> Result<Vec<TaskModel>, String>{
    if initialized.load(Ordering::SeqCst) {
        Ok(controller.get().unwrap().lock().unwrap().tasks.clone())

    }else{
        Err(String::from("Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
}

#[tauri::command]
pub async fn get_task(task_id: String) -> Result<TaskModel, String>{
    if initialized.load(Ordering::SeqCst) {
        Ok(controller.get().unwrap().lock().unwrap().tasks.iter().find(|task| task.id == task_id).unwrap().clone())

    }else{
        Err(String::from("Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
}

 #[tauri::command]
 pub async fn add_task(task_to_add: TaskModel) -> Result<(), String>{
    if initialized.load(Ordering::SeqCst) {
        let res = controller.get().unwrap().lock().unwrap().add_task(task_to_add);

        if res.is_err(){
            return Err(String::from(format!("Nie udało się dodać nowego taska! {}", res.err().unwrap())));
        }
        Ok(())
    }else{
        Err(String::from("Rust Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
 }

 #[tauri::command]
 pub async fn del_task(task_id: String) -> Result<(), String>{
    if initialized.load(Ordering::SeqCst) {
        let res = controller.get().unwrap().lock().unwrap().delete_task(task_id);
        if res.is_err() {
            return Err(String::from(format!("Nie udało się usunąć taska: {}", res.err().unwrap())));
        }
        Ok(())
    }else{
        Err(String::from("Rust Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
 }

 #[tauri::command]
 pub async fn run_task(task_id: String) -> Result<(), String> {
    if initialized.load(Ordering::SeqCst) {
        let ctrl = controller.get().unwrap().lock().unwrap();

        let task = ctrl.get_task(task_id.clone());
        
        if task.is_none() == false {
            let mut c = controller.get().unwrap().lock().unwrap();
            let proc = c.create_process(task.unwrap().clone());

            if proc.is_ok() {
                Ok(())
            }else{
                Err(String::from("Błąd tworzenia procesu dla taska"))
            }
        } else {
            Err(format!("Nie znaleziono zadania dla id {}", task_id))
        }
    } else {
        Err(String::from("Rust Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
 }

  #[tauri::command]
 pub async fn stop_task(task_id: String) -> Result<(), String> {
    if initialized.load(Ordering::SeqCst) {
        let ctrl = controller.get().unwrap().lock().unwrap();

        let task = ctrl.get_task(task_id.clone());
        
        if task.is_none() == false {
            let c = controller.get().unwrap().lock().unwrap();
            let proc = c.stop_process(task_id);

            if proc.is_ok() {
                Ok(())
            }else{
                Err(String::from("Błąd tworzenia procesu dla taska"))
            }
        } else {
            Err(format!("Nie znaleziono zadania dla id {}", task_id))
        }
    } else {
        Err(String::from("Rust Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
 }
