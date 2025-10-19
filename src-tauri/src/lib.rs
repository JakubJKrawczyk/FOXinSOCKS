pub mod controllers;
mod data;
pub mod models;
mod commands;
pub mod utills; // logger & inne utils
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
fn get_log_path() -> String {
    crate::utills::logger::current_log_file_path()
}

// Zwraca pełną zawartość aktualnego pliku logów.
// Jeśli plik nie istnieje (np. jeszcze nic nie zalogowano) zwraca pusty string.
#[tauri::command]
fn get_all_logs() -> String {
    let path = crate::utills::logger::current_log_file_path();
    match std::fs::read_to_string(&path) {
        Ok(content) => content,
        Err(_) => String::new(),
    }
}

// Zwraca ogon (ostatnie 'lines' linii) logu, czytając maksymalnie ~64KB z końca pliku
#[tauri::command]
fn get_log_tail(lines: usize) -> String {
    use std::io::{Read, Seek, SeekFrom};
    let path = crate::utills::logger::current_log_file_path();
    let file = match std::fs::File::open(&path) { Ok(f) => f, Err(_) => return String::new() };
    let size = file.metadata().map(|m| m.len()).unwrap_or(0);
    let window: u64 = 64 * 1024; // 64KB okno do odczytania z końca
    let start = if size > window { size - window } else { 0 };
    let mut f = file;
    if start > 0 { let _ = f.seek(SeekFrom::Start(start)); }
    let mut buf = String::new();
    let _ = f.read_to_string(&mut buf);
    let lines_vec: Vec<&str> = buf.lines().collect();
    let len = lines_vec.len();
    let start_line = if lines >= len { 0 } else { len - lines };
    lines_vec[start_line..].join("\n")
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
    .invoke_handler(tauri::generate_handler![get_log_path, get_all_logs, get_log_tail, init, get_tasks, get_task, add_task, del_task, run_task, stop_task, update_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
