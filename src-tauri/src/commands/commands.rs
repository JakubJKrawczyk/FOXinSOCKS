use crate::controllers::CleanProcessesControler::CleanProcessController;
use std::os::windows::process;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use crate::models::task_model::TaskModel;
pub static controller: OnceLock<Mutex<CleanProcessController>> = OnceLock::new();
pub static initialized: AtomicBool = AtomicBool::new(false);


#[tauri::command]
pub async fn init() -> Result<(), String> {
    if initialized.load(Ordering::SeqCst) {
        let _ = controller.set(Mutex::new(CleanProcessController::new()));
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
        let res = controller.get().unwrap().lock().unwrap().add_task(&task_to_add);

        if res.is_err(){
            return Err(String::from(format!("Nie udało się dodać nowego taska! {}", res.err().unwrap())));
        }
        Ok(())
    }else{
        Err(String::from("Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
 }

 #[tauri::command]
 pub async fn del_task(task_id: String) -> Result<(), String>{
    if initialized.load(Ordering::SeqCst) {
        let res = controller.get().unwrap().lock().unwrap().delete_task(&task_id);
        if res.is_err() {
            return Err(String::from(format!("Nie udało się usunąć taska: {}", res.err().unwrap())));
        }
        Ok(())
    }else{
        Err(String::from("Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
 }

 #[tauri::command]
 pub async fn run_task(task_id: String) -> Result<(), String> {
    if initialized.load(Ordering::SeqCst) {
        let ctrl = controller.get().unwrap().lock().unwrap();

        let task = ctrl.get_task(&task_id);
        
        if task.is_none() == false {
            let mut c = controller.get().unwrap().lock().unwrap();
            let proc = c.create_process(&task.unwrap());

            if proc.is_ok() {
                Ok(())
            }else{
                Err(String::from("Błąd tworzenia procesu dla taska"))
            }
        } else {
            Err(format!("Nie znaleziono zadania dla id {}", task_id))
        }
    } else {
        Err(String::from("Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
 }

  #[tauri::command]
 pub async fn stop_task(task_id: String) -> Result<(), String> {
    if initialized.load(Ordering::SeqCst) {
        let ctrl = controller.get().unwrap().lock().unwrap();

        let task = ctrl.get_task(&task_id);
        
        if task.is_none() == false {
            let c = controller.get().unwrap().lock().unwrap();
            let proc = c.stop_process(&task_id);

            if proc.is_ok() {
                Ok(())
            }else{
                Err(String::from("Błąd zatrzymywania procesu dla taska"))
            }
        } else {
            Err(format!("Nie znaleziono zadania dla id {}", task_id))
        }
    } else {
        Err(String::from("Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
 }

 #[tauri::command]
 pub async fn update_task(task: TaskModel) -> Result<(), String>{
    if initialized.load(Ordering::SeqCst) {
        let ctrl = controller.get().unwrap().lock().unwrap();

        let backend_task = ctrl.get_task(&task.id);

        if backend_task.is_none() == false {
            let c = controller.get();
            if c.is_none() == false {
                 let proc = c.unwrap().lock().unwrap().stop_process(&backend_task.unwrap().id);

                if proc.is_ok() {
                    return Ok(())
                }else{
                    return Err(String::from("Błąd zatrzymywania procesu dla taska"));
                }
            }

            let res = c.unwrap().lock().unwrap().update_task(&task);

            if res.is_err(){
                return Err(String::from("Błąd przy aktualizacji taska"))
            }
                
            Ok(())
           
        } else {
            Err(format!("Nie znaleziono zadania dla id {}", backend_task.unwrap().id))
        }
    } else {
        Err(String::from("Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
 }