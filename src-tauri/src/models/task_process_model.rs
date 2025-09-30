use tokio::task;
use serde::{Serialize};

#[derive(Serialize)]
pub struct TaskProcessModel{
    #[serde(skip_serializing, skip_deserializing)]
    pub process: task::JoinHandle<()>,
    pub task_id: String
}

impl TaskProcessModel{
    pub fn new(process: task::JoinHandle<()>, id: String) -> TaskProcessModel{
        TaskProcessModel { process, task_id: id }
    }
}