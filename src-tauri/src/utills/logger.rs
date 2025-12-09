// Prosty logger: log (szary), warning (żółty), error (czerwony).
// Wszystko trafia do pliku Backend_logs_{YYYY-MM-DD}.txt i na konsolę z kolorami.
use chrono::Local;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

const GRAY: &str = "\x1b[90m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
const BLUE: &str = "\x1b[34m"; // PROCESS

enum Level {
    Log,
    Warning,
    Error,
    Process,
}

impl Level {
    fn as_str(&self) -> &'static str {
        match self {
            Level::Log => "LOG",
            Level::Warning => "WARNING",
            Level::Error => "ERROR",
            Level::Process => "PROCESS",
        }
    }
    fn color(&self) -> &'static str {
        match self {
            Level::Log => GRAY,
            Level::Warning => YELLOW,
            Level::Error => RED,
            Level::Process => BLUE,
        }
    }
}

static FILE_HANDLE: OnceLock<Mutex<std::fs::File>> = OnceLock::new();

fn logs_dir() -> PathBuf {
    // katalog obok uruchomionego exe + /logs
    let base = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let dir = base.join("logs");
    let _ = create_dir_all(&dir); // ignorujemy błędy – fallback niżej
    dir
}

fn file_name_for_today() -> String {
    let date = Local::now().format("%Y-%m-%d");
    format!("Backend_logs_{}.txt", date)
}

fn full_log_path() -> PathBuf {
    logs_dir().join(file_name_for_today())
}

pub fn current_log_file_path() -> String {
    full_log_path().to_string_lossy().to_string()
}

fn get_file() -> &'static Mutex<std::fs::File> {
    FILE_HANDLE.get_or_init(|| {
        let path = full_log_path();
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .unwrap_or_else(|_| {
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("Backend_logs_fallback.txt")
                    .expect("Brak fallback pliku")
            });
        Mutex::new(file)
    })
}

fn write_line(level: Level, message: &str) {
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S");
    let line_plain = format!("[{}][{}] {}\n", timestamp, level.as_str(), message);
    if let Ok(mut file) = get_file().lock() {
        let _ = file.write_all(line_plain.as_bytes());
    }
    let colored = format!(
        "{}[{}][{}] {}{}",
        level.color(),
        timestamp,
        level.as_str(),
        message,
        RESET
    );
    match level {
        Level::Error => eprintln!("{}", colored),
        _ => println!("{}", colored),
    }
}

pub fn log<M: AsRef<str>>(message: M) {
    write_line(Level::Log, message.as_ref());
}
pub fn warning<M: AsRef<str>>(message: M) {
    write_line(Level::Warning, message.as_ref());
}
pub fn error<M: AsRef<str>>(message: M) {
    write_line(Level::Error, message.as_ref());
}
pub fn process<M: AsRef<str>>(message: M) {
    write_line(Level::Process, message.as_ref());
}
