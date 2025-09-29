use tokio::task;

pub struct TaskProcessModel{
    pub process: task::JoinHandle<()>,
    pub task_id: String
}

impl TaskProcessModel{
    pub fn new(process: task::JoinHandle<()>, id: String) -> TaskProcessModel{
        TaskProcessModel { process, task_id: id }
    }
}