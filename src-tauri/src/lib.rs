mod controllers;
mod data;
mod models;
mod commands;
mod utills; // logger & inne utils
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

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
        .invoke_handler(tauri::generate_handler![greet, get_log_path, init, get_tasks, get_task, add_task, del_task, run_task, stop_task, update_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
