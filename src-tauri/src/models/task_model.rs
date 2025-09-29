use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize, Clone)]
pub enum TaskStatus{
    #[serde(rename="sheduled")] // zachowanie dotychczasowego stringa (pisownia w danych)
    Sheduled,
    #[serde(rename="in-progress")] // dostosowane do frontendu
    InProgress,
    #[default]
    #[serde(rename="idle")]
    Idle
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TaskModel{
    pub id: String,
    pub title: String,
    pub description: String,
    pub auto_run: bool,
    pub auto_run_interval: u32,
    pub task_process_id: u32,
    pub status: TaskStatus,
    pub regex_patterns: Vec<String>,
    pub folder_path: String,
    pub number_of_dup_to_keep: u8,
}

impl TaskModel{
    /// Utwórz nowy TaskModel z domyślnymi wartościami i auto-generowanym ID.
    /// Domyślne wartości:
    ///  title: "New Task"
    ///  description: ""
    ///  auto_run: false
    ///  auto_run_interval: 60 (minut)
    ///  task_process_id: 0
    ///  status: TaskStatus::Idle
    ///  regex_patterns: []
    ///  folder_path: ""
    ///  number_of_dup_to_keep: 2
    pub fn new_default() -> TaskModel {
        TaskModel {
            id: Uuid::new_v4().to_string(),
            title: "New Task".to_string(),
            description: String::new(),
            auto_run: false,
            auto_run_interval: 60,
            task_process_id: 0,
            status: TaskStatus::Idle,
            regex_patterns: Vec::new(),
            folder_path: String::new(),
            number_of_dup_to_keep: 2,
        }
    }
}

impl Default for TaskModel {
    fn default() -> Self { TaskModel::new_default() }
}