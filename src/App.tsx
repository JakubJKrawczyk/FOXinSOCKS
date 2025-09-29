import "./style/App.css";
import {taskModel, TaskStatus} from "./backend/models/taskModel";
import { useEffect, useState  } from "react";
import TasksList from "./components/TasksList";
import TasksController from "./backend/utilities/tasksController";

function App() {

    // VARIABLES

    const [selectedItem, setSelectedItem] = useState<taskModel | null>(null);
    const [tasks, setTasks] = useState<taskModel[]>([]);

    const controller = new TasksController();
  // END OF VARIABLES

  // EVENTS
  document.onclick = (e) => {
    if(
      (e.target as HTMLElement).className.includes("task") === false
    ) {
      console.log("clicked outside");
      setSelectedItem(null);
      selectFrontItem(null);
    }
  };

  useEffect(() => {
      let listElements = document.getElementsByClassName("task-selectable");
      console.log(listElements.length)
      var element = listElements[listElements.length - 1] as HTMLElement;
      selectFrontItem(element)
    },[tasks])

  // FUNCTIONS

  function selectFrontItem(element: HTMLElement | null){

    let listElements = document.getElementsByClassName("task-selectable");

        for (let i = 0; i < listElements.length; i++) {
        listElements[i].setAttribute("class", "task-item task-selectable");
        }
        if(element)
          element.setAttribute("class", "task-item task-selectable selected-task-item");
  }
  
  function selectTask(e: React.MouseEvent<HTMLTableRowElement, MouseEvent>, task: taskModel) {
    
        console.log("Selected task:", task);
        var element = e.currentTarget as HTMLTableRowElement;
        selectFrontItem(element)
        setSelectedItem(task)
    }

    

  function updateTask() {
    console.log("Updating task...");
  
    const newTasks = tasks.map((task) => 
      task.id == selectedItem!.id ? task = selectedItem! : task
    )
    controller.updateTask(selectedItem!);
    setTasks(newTasks)
  };

  function handleChange(e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) {
    const { name, value} = e.target;
    if (!selectedItem) return;

    setSelectedItem(() => {
      let updated = { ...selectedItem };

      switch (name) {
        case "task-title":
          updated.title = value;
          break;
        case "task-description":
          updated.description = value;
          break;
        case "task-status":
          updated.status = value as TaskStatus;
          break;
        case "task-auto-run":
          updated.auto_run = (e.target as HTMLInputElement).checked;
          break;
        case "task-auto-run-interval":
          updated.auto_run_interval = Number(value);
          break;
        case "task-number-of-dups":
          updated.number_of_dup_to_keep = Number(value);
          break;
        case "task-regex-patterns":
          updated.regex_patterns = value.split("\n").map(pattern => pattern.trim()).filter(pattern => pattern.length > 0);
          break;
        case "task-folder-path":
          updated.folder_path = value;
          break;
        default:
          break;
      }
      return updated;
    });
  }

  function addTask(){
    console.log("Adding new task")
    let newTask = new taskModel();
    setTasks([...tasks, newTask])
    setSelectedItem(newTask)
    controller.addTask(newTask);
    
  }

  function delTask(id: string){
    console.log("Deleting task with id: " + id)
    let newTasks = tasks.filter((task) => task.id !== id);
    controller.delTask(id);
    setTasks(newTasks);
  
  }

  function runTask(id: string){
    console.log("Running task with id: " + id)
    controller.runTask(id);
    let newTasks = tasks.map((task) => 
      task.id == id ? {...task, status: TaskStatus.InProgress} : task
    )
    setTasks(newTasks);
  }
  
  // RENDER
  return (
    <main>
      {/* Main container */}
      <div className="container-main">
        {/* Top Container */}
        <div className="container-top" > 
          <h1 className="title">Fox in Socks</h1>
          <p className="subtitle">A simple cleaner at your service</p>
        </div>
        {/* Bottom Container */}
        <div className="container-bottom"> 
          {/* List of tasks */}

          <TasksList t={tasks} selectTask={selectTask} addTask={addTask} delTask={delTask} runTask={runTask}/>

          {/* Task description */}
          {selectedItem && (
            <form className="task-description" datatype="taskModel">
              <div className="task-title-container">
                <input className="task-title-input" type="text" value={selectedItem.title} placeholder="Nazwa taska..." name="task-title" onChange={handleChange}/>
              </div>
              <div className="task-description-container">
                <textarea className="task-description-input" placeholder="Opis..." value={selectedItem.description} name="task-description" onChange={handleChange}/>
              </div>
              <div className="task-regex-patterns-container">
                <textarea className="task-regex-patterns-input" value={selectedItem.regex_patterns.join("\n")} placeholder="Wzory plików" name="task-regex-patterns" onChange={handleChange}/>
              </div>
              <div className="task-folder-path-container">
                <textarea className="task-folder-path-input" value={selectedItem.folder_path} placeholder="Ścieżka folderu" name="task-folder-path" onChange={handleChange}/>
              </div>
            
              <div className="task-auto-run-container">
                <label className="task-auto-run-label"> Auto Run</label>
                <input className="task-auto-run-input" checked={selectedItem.auto_run} type="checkbox" name="task-auto-run" onChange={handleChange}/>
              </div>
              <div className="task-interval-container">
                <label className="task-interval-label"> Minutes Interval: </label>
                <input className="task-interval-input" type="number" value={selectedItem.auto_run_interval} name="task-auto-run-interval" onChange={handleChange}/>
              </div>
              <div className="task-number-of-dups-container">
                <label className="task-number-of-dups-label"> Number of Duplicates to Keep: </label>
                <input className="task-number-of-dups-input" type="number" value={selectedItem.number_of_dup_to_keep} name="task-number-of-dups" onChange={handleChange}/>
              </div>
              <div className="task-save-button-container">
                <button type="button" className="task-save-button" onClick={() => updateTask()}>Zapisz</button>
              </div>
              
            </form>
          )}
          </div>
      </div>
    </main>
  );
}

export default App;
