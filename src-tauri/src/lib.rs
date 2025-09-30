mod controllers;
mod data;
mod models;
mod commands;
mod utills; // logger & inne utils
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
fn get_log_path() -> String {
    crate::utills::logger::current_log_file_path()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use crate::commands::commands::{init, get_tasks, get_task, add_task, del_task, run_task, stop_task, update_task};
    // Wymuszenie wczesnej inicjalizacji loggera oraz wpis startowy
    crate::utills::logger::log("Aplikacja startuje - inicjalizacja loggera");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // Zapisz taski przy zamknięciu okna (kliknięcie X)
        .on_window_event(|_app_handle, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                crate::utills::logger::log("Zamykanie okna - próba zapisu tasków");
                if let Some(ctrl_mutex) = crate::commands::commands::CONTROLLER.get() {
                    match ctrl_mutex.lock() {
                        Ok(ctrl) => {
                            match crate::controllers::file_system_controller::save_tasks(&ctrl.tasks) {
                                Ok(_) => crate::utills::logger::log("Zapisano taski przed zamknięciem okna"),
                                Err(e) => crate::utills::logger::error(format!("Błąd zapisu tasków przy zamykaniu: {}", e)),
                            }
                        }
                        Err(_) => crate::utills::logger::error("Nie udało się zablokować kontrolera przy zamykaniu okna"),
                    }
                } else {
                    crate::utills::logger::warning("Zamykanie okna - backend nie był zainicjalizowany, brak tasków do zapisu");
                }
            }
        })
        .invoke_handler(tauri::generate_handler![get_log_path, init, get_tasks, get_task, add_task, del_task, run_task, stop_task, update_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
