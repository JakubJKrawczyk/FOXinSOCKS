use tokio::task;

use crate::models::task_model::TaskModel;

pub struct TaskProcessModel{
    pub process: task::JoinHandle<String>,
    pub task_id: String
}

impl TaskProcessModel{
    pub fn new(process: task::JoinHandle<String>, id: String) -> TaskProcessModel{


        TaskProcessModel {
            process: process,
            task_id: id
        }
    }
}