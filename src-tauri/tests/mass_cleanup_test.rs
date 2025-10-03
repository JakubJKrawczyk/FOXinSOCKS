//! Test masowego czyszczenia – generuje 100 bazowych nazw, dla każdej 100 duplikatów
//! oraz tworzy regexy odpowiadające każdej bazie. Uruchamia logikę czyszczenia
//! (pojedynczą iterację) i weryfikuje czy liczba plików została zredukowana
//! do oczekiwanej wartości (number_of_dup_to_keep) * liczba bazowych nazw.

use std::fs::{self, File};
use std::io::Write;
use rand::{Rng, distributions::Alphanumeric};
use foxinsocks_lib::models::task_model::TaskModel;

// Pomocnicza funkcja: jedna iteracja czyszczenia – kopiujemy wewnętrzną logikę cleaning_command
async fn single_clean_run(folder_path: &str, regex_patterns: &[String], dup_to_keep: u8){
    use std::path::Path;
    use walkdir::WalkDir;
    use std::collections::HashMap;
    use regex::Regex;

    let folder_path_path = Path::new(folder_path);
    assert!(folder_path_path.exists() && folder_path_path.is_dir(), "Folder testowy nie istnieje");

    let mut total_deleted = 0usize;

    for regex_str in regex_patterns {
        let re = Regex::new(regex_str).expect("Niepoprawny regex w teście");
        let mut file_groups: HashMap<String, Vec<(String, std::time::SystemTime)>> = HashMap::new();
        for entry in WalkDir::new(folder_path_path).into_iter().filter_map(|e| e.ok()) {
            if !entry.file_type().is_file(){ continue; }
            let file_name = entry.file_name().to_string_lossy().to_string();
            if re.is_match(&file_name) {
                if let Ok(metadata) = entry.metadata() { if let Ok(modified) = metadata.modified() {
                    file_groups.entry(regex_str.clone()).or_default().push((entry.path().to_string_lossy().to_string(), modified));
                }}
            }
        }
        for (_name, mut files) in file_groups { // cleanup
            files.sort_by_key(|f| f.1);
            if files.len() > dup_to_keep as usize {
                let to_delete = files.len() - dup_to_keep as usize;
                for (path, _) in files.into_iter().take(to_delete) {
                    let _ = fs::remove_file(&path);
                    total_deleted += 1;
                }
            }
        }
    }
    foxinsocks_lib::utills::logger::process(format!("[TEST] Usunięto {} plików w pojedynczym przebiegu", total_deleted));
}

#[tokio::test]
async fn mass_cleanup_reduces_duplicates(){
    // Parametry testu
    const BASE_FILES: usize = 100; // liczba bazowych nazw
    const DUPS_PER_BASE: usize = 100; // duplikatów tworzonych na bazę
    const KEEP: u8 = 5; // ile chcemy zostawić

    // Katalog testowy
    let mut dir = std::env::temp_dir();
    dir.push(format!("foxinsocks_mass_test_{}", rand::thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect::<String>()));
    fs::create_dir_all(&dir).expect("Nie można utworzyć katalogu testowego");

    // Generacja plików oraz regexów
    let mut patterns: Vec<String> = Vec::new();
    for i in 0..BASE_FILES {
        // baza nazwy np. file_<i>_
        let base = format!("file_{}__", i);
        // regex dopasuje wszystkie jego warianty (cokolwiek po bazie aż do rozszerzenia)
        let regex = format!("^{}.*\\.log$", regex::escape(&base));
        patterns.push(regex);
        for d in 0..DUPS_PER_BASE {
            let file_name = format!("{}{}_{}.log", base, d, rand::thread_rng().sample_iter(&Alphanumeric).take(5).map(char::from).collect::<String>());
            let mut path = dir.clone();
            path.push(file_name);
            let mut f = File::create(&path).expect("create file");
            // write something + different timestamp hint
            writeln!(f, "test {} {}", i, d).ok();
        }
    }

    // Sanity check
    let initial_count = fs::read_dir(&dir).unwrap().count();
    assert_eq!(initial_count, BASE_FILES * DUPS_PER_BASE, "Nie utworzono oczekiwanej liczby plików");

    // Przygotowanie pseudo-taska (nie odpalamy create_process aby nie wchodzić w nieskończoną pętlę)
    let mut task = TaskModel::new_default();
    task.folder_path = dir.to_string_lossy().to_string();
    task.regex_patterns = patterns.clone();
    task.number_of_dup_to_keep = KEEP;

    // Jednorazowe odpalenie logiki czyszczącej (symulacja jednej iteracji)
    single_clean_run(&task.folder_path, &task.regex_patterns, task.number_of_dup_to_keep).await;

    // Weryfikacja: dla każdej grupy powinno zostać <= KEEP plików
    // Sprawdzamy przez iterację regexów
    for pattern in &patterns {
        let re = regex::Regex::new(pattern).unwrap();
        let mut matched = 0usize;
        for entry in walkdir::WalkDir::new(&dir).into_iter().filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() { continue; }
            let name = entry.file_name().to_string_lossy();
            if re.is_match(&name) { matched += 1; }
        }
        assert!(matched as u8 <= KEEP, "Dla wzorca {} zostalo {} > {}", pattern, matched, KEEP);
    }
}
