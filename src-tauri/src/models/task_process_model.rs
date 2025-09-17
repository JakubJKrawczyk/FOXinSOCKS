use tokio::task;
mod task_model;
#[derive(Default)]
pub struct TaskProcessModel{
    pub process: &task::Id,
    pub task: &TaskModel
}

impl TaskProcessModel{
    pub fn New(process: &task, task: &TaskModel) -> TaskProcessModel{


        TaskProcessModel {
            process: process,
            task: task
        }
    }
}