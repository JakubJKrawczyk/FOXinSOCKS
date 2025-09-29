use crate::controllers::clean_processes_controller::CleanProcessController;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use crate::models::task_model::TaskModel;
use crate::utills::logger; // logger
pub static CONTROLLER: OnceLock<Mutex<CleanProcessController>> = OnceLock::new();
pub static INITIALIZED: AtomicBool = AtomicBool::new(false);


#[tauri::command]
pub async fn init() -> Result<(), String> {
    if INITIALIZED.load(Ordering::SeqCst) {
    logger::warning("Próba ponownej inicjalizacji backendu (zignorowano)");
        return Err(String::from("Rust jest już zainicjalizowany"));
    }
    logger::log("Inicjalizacja backendu");
    match CONTROLLER.set(Mutex::new(CleanProcessController::new())) {
        Ok(_) => {
            INITIALIZED.store(true, Ordering::SeqCst);
            logger::log("Backend zainicjalizowany poprawnie");
            Ok(())
        },
        Err(_) => {
            logger::error("Nie udało się ustawić kontrolera (OnceLock)");
            Err(String::from("Błąd inicjalizacji"))
        }
    }
}

#[tauri::command]
pub async fn get_tasks() -> Result<Vec<TaskModel>, String>{
    if INITIALIZED.load(Ordering::SeqCst) {
    logger::log("Pobieranie listy tasków");
        let tasks = CONTROLLER.get().unwrap().lock().unwrap().tasks.clone();
        // Log pełnej listy jako JSON (jedna linia) + liczność
        match serde_json::to_string(&tasks) {
            Ok(json) => logger::log(format!("Lista tasków ({}): {}", tasks.len(), json)),
            Err(e) => logger::warning(format!("Nie udało się zserializować listy tasków: {}", e)),
        }
        Ok(tasks)

    }else{
    logger::warning("Próba pobrania tasków bez inicjalizacji");
        Err(String::from("Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
}

#[tauri::command]
pub async fn get_task(task_id: String) -> Result<TaskModel, String>{
    if INITIALIZED.load(Ordering::SeqCst) {
    logger::log(format!("Pobieranie taska id={}", task_id));
        let task_clone = {
            let guard = CONTROLLER.get().unwrap().lock().unwrap();
            guard.tasks.iter().find(|task| task.id == task_id).cloned()
        };
        if let Some(t) = task_clone { Ok(t) } else {
            logger::warning(format!("Nie znaleziono taska id={}", task_id));
            Err(format!("Nie znaleziono zadania dla id {}", task_id))
        }

    }else{
    logger::warning("Próba pobrania taska bez inicjalizacji");
        Err(String::from("Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
}

 #[tauri::command]
 pub async fn add_task() -> Result<TaskModel, String>{
    if INITIALIZED.load(Ordering::SeqCst) {
        let task_to_add = TaskModel::new_default();
    logger::log(format!("Dodawanie nowego taska id={} title={}", task_to_add.id, task_to_add.title));
        let res = CONTROLLER.get().unwrap().lock().unwrap().add_task(&task_to_add);
        match res {
            Ok(_) => { logger::log("Dodano task"); Ok(task_to_add) },
            Err(e) => { logger::error(format!("Błąd dodawania taska: {}", e)); Err(format!("Nie udało się dodać nowego taska! {}", e)) }
        }
    }else{
    logger::warning("Próba dodania taska bez inicjalizacji");
        Err(String::from("Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
 }

 #[tauri::command]
 pub async fn del_task(task_id: String) -> Result<(), String>{
    if INITIALIZED.load(Ordering::SeqCst) {
    logger::log(format!("Usuwanie taska id={}", task_id));
        let res = CONTROLLER.get().unwrap().lock().unwrap().delete_task(&task_id);
        match res {
            Ok(_) => { logger::log("Usunięto task"); Ok(()) },
            Err(e) => { logger::error(format!("Błąd usuwania taska: {}", e)); Err(format!("Nie udało się usunąć taska: {}", e)) }
        }
    }else{
    logger::warning("Próba usunięcia taska bez inicjalizacji");
        Err(String::from("Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
 }

 #[tauri::command]
 pub async fn run_task(task_id: String) -> Result<(), String> {
    if INITIALIZED.load(Ordering::SeqCst) {
    logger::log(format!("Uruchamianie taska id={}", task_id));
        let ctrl = CONTROLLER.get().unwrap().lock().unwrap();
        let task = ctrl.get_task(&task_id).cloned();
        drop(ctrl);
        if let Some(task_ref) = task {
            let mut c = CONTROLLER.get().unwrap().lock().unwrap();
            match c.create_process(&task_ref) {
                Ok(_) => { logger::log("Proces dla taska uruchomiony"); Ok(()) },
                Err(e) => { logger::error(format!("Błąd tworzenia procesu: {}", e)); Err(String::from("Błąd tworzenia procesu dla taska")) }
            }
        } else {
            logger::warning(format!("Nie znaleziono taska id={} przy uruchamianiu", task_id));
            Err(format!("Nie znaleziono zadania dla id {}", task_id))
        }
    } else {
    logger::warning("Próba uruchomienia taska bez inicjalizacji");
        Err(String::from("Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
 }

  #[tauri::command]
 pub async fn stop_task(task_id: String) -> Result<(), String> {
    if INITIALIZED.load(Ordering::SeqCst) {
    logger::log(format!("Zatrzymywanie taska id={}", task_id));
        let ctrl = CONTROLLER.get().unwrap().lock().unwrap();
        let task_exists = ctrl.get_task(&task_id).is_some();
        drop(ctrl);
        if task_exists {
            let c = CONTROLLER.get().unwrap().lock().unwrap();
            match c.stop_process(&task_id) {
                Ok(_) => { logger::log("Proces taska zatrzymany"); Ok(()) },
                Err(e) => { logger::error(format!("Błąd zatrzymywania procesu: {}", e)); Err(String::from("Błąd zatrzymywania procesu dla taska")) }
            }
        } else {
            logger::warning(format!("Nie znaleziono taska id={} przy zatrzymywaniu", task_id));
            Err(format!("Nie znaleziono zadania dla id {}", task_id))
        }
    } else {
    logger::warning("Próba zatrzymania taska bez inicjalizacji");
        Err(String::from("Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
 }

 #[tauri::command]
 pub async fn update_task(task: TaskModel) -> Result<(), String>{
    if INITIALIZED.load(Ordering::SeqCst) {
    logger::log(format!("Aktualizacja taska id={}", task.id));
        let ctrl = CONTROLLER.get().unwrap().lock().unwrap();
        let backend_task = ctrl.get_task(&task.id).cloned();
        drop(ctrl);
        if let Some(old_task) = backend_task {
            // zatrzymaj proces (jeśli działa)
            if let Some(c) = CONTROLLER.get() {
                let stop_res = c.lock().unwrap().stop_process(&old_task.id);
                if let Err(e) = stop_res { logger::warning(format!("Nie udało się zatrzymać procesu przed aktualizacją: {}", e)); }
            }
            if let Some(c) = CONTROLLER.get() {
                let update_res = c.lock().unwrap().update_task(&task);
                match update_res {
                    Ok(_) => { logger::log("Task zaktualizowany"); Ok(()) },
                    Err(e) => { logger::error(format!("Błąd aktualizacji taska: {}", e)); Err(String::from("Błąd przy aktualizacji taska")) }
                }
            } else {
                logger::error("Brak kontrolera podczas aktualizacji");
                Err(String::from("Błąd wewnętrzny kontrolera"))
            }
        } else {
            logger::warning(format!("Nie znaleziono taska id={} do aktualizacji", task.id));
            Err(format!("Nie znaleziono zadania dla id {}", task.id))
        }
    } else {
    logger::warning("Próba aktualizacji taska bez inicjalizacji");
        Err(String::from("Rust nie został zainicjalizowany! użyj metody init żeby to zmienić"))
    }
 }