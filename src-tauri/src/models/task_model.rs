use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub enum TaskStatus{
    sheduled,
    inProgress,
    #[default]
    idle
}

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct TaskModel{
    pub id: String,
    pub title: String,
    pub description: String,
    pub auto_run: bool,
    pub auto_run_interval: u32,
    pub task_process_id: u32,
    pub status: TaskStatus,
    pub regex_patterns: Vec<String>,
    pub folder_paths: Vec<String>,
    pub number_of_dup_to_keep: u8,
}

impl TaskModel{
    pub fn new(
        id: String,
        title: String,
        description: String,
        auto_run: bool,
        auto_run_interval: u32,
        task_process_id: u32,
        status: TaskStatus,
        regex_patterns: Vec<String>,
        folder_paths: Vec<String>,
        number_of_dup_to_keep: u8
    ) -> TaskModel{
        TaskModel {
            id: id,
            title: title,
            description: description,
            auto_run: auto_run,
            auto_run_interval: auto_run_interval,
            task_process_id: task_process_id,
            status: status,
            regex_patterns: regex_patterns,
            folder_paths: folder_paths,
            number_of_dup_to_keep: number_of_dup_to_keep
        }
    }
}