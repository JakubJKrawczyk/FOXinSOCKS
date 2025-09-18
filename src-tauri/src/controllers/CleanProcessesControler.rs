use std::ops::Index;
use std::process::id;

use crate::models::task_model::TaskModel;
use crate::models::task_process_model::TaskProcessModel;
use crate::controllers::FileSystemController;


pub struct clean_process_controller{
    pub tasks_processes: Vec<TaskProcessModel>,
    pub tasks : Vec<TaskModel>
}

impl clean_process_controller{
    pub fn new() -> Self {
        let init_filesystem = FileSystemController::file_system_controller::init();
        let tasks = FileSystemController::file_system_controller::get_tasks();

        clean_process_controller {
            tasks: tasks,
            tasks_processes: Vec::new()
        }
    }

    pub fn get_task(&self, id: String) -> Option<&TaskModel>
    {
        self.tasks.iter().find(|task| task.id == id)
    }

    pub fn update_task(&mut self, new_task: TaskModel) -> Result<String, String>{
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == new_task.id){
            *task = new_task;
            Ok(String::from("Task zaktualizowany"))
        }else{
            Err(String::from("Nie znaleziono taska"))
        }
    }

    pub fn delete_task(&mut self, task_id: String) -> Result<String, String>{
        if let Some(pos) = self.tasks.iter().position(|task| task.id == task_id){
            self.tasks.remove(pos);
            Ok(String::from("Udało się usunąć taska"))
        }else{
            Err(String::from("Nie znaleziono taska"))
        }
    }

    pub fn add_task(&mut self, task: TaskModel) -> Result<TaskModel, String>{
        if self.tasks.iter().any(|t| t.id == task.id) == false{

            self.tasks.push(task.clone());
            Ok(task)
        }else{
            Err(String::from("W systemie istnieje już task o takim Id"))
        }
    }
}