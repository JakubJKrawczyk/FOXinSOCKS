use tokio::task;

use crate::models::task_model::TaskModel;

pub struct TaskProcessModel{
    pub process: task::JoinHandle<String>,
    pub task: TaskModel
}

impl TaskProcessModel{
    pub fn new(process: task::JoinHandle<String>, task: TaskModel) -> TaskProcessModel{


        TaskProcessModel {
            process: process,
            task: task
        }
    }
}