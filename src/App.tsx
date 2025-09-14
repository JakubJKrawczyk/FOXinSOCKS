import "./style/App.css";
import taskModel from "./backend/models/taskModel";
import { Component, useState  } from "react";
import TasksList from "./components/TasksList";


function App() {

    // VARIABLES

    const [selectedItem, setSelectedItem] = useState<taskModel | null>(null);
    const [tasks, setTasks] = useState<taskModel[]>([]);
    var tasksList = document.getElementsByClassName("tasks-list")[0] as HTMLTableElement
  // END OF VARIABLES

  // EVENTS
  document.onclick = (e) => {
    if(
      (e.target as HTMLElement).className.includes("task") === false
    ) {
      console.log("clicked outside");
      setSelectedItem(null);
    }
  };


  // FUNCTIONS

  

  function updateTask(e: React.FormEvent<HTMLFormElement>) {
    console.log("Updating task...");
    
    if(selectedItem) {
      const formData = new FormData(e.currentTarget);
      selectedItem.title = formData.get("task-title") as string;
      selectedItem.description = formData.get("task-description") as string;
      selectedItem.status = formData.get("task-status") as "sheduled" | "in-progress" | "idle" | "done";
      selectedItem.auto_run = formData.get("task-auto-run") === "on";
      selectedItem.auto_run_interval = Number(formData.get("task-auto-run-interval"));
      selectedItem.number_of_dup_to_keep = Number(formData.get("task-number-of-dups"));
      selectedItem.regex_patterns = (formData.get("task-regex-patterns") as string).split("\n").map(pattern => pattern.trim()).filter(pattern => pattern.length > 0);
      selectedItem.folder_paths = (formData.get("task-folder-paths") as string).split("\n").map(path => path.trim()).filter(path => path.length > 0);
    }
    const newTasks = tasks.map((task) => 
      task.id == selectedItem?.id ? { ...task, task } : task 
    );
    setTasks(newTasks);

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
          updated.status = value as "sheduled" | "in-progress" | "idle" | "done";
          break;
        case "task-auto-run":
          updated.auto_run = value.toString() === "true";
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
        case "task-folder-paths":
          updated.folder_paths = value.split("\n").map(path => path.trim()).filter(path => path.length > 0);
          break;
        default:
          break;
      }
      return updated;
    });
  }

  function addTask(){
    console.log("Adding new task")
    setTasks([...tasks, new taskModel()])
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
          
          <TasksList t={tasks} setSelectedItem={setSelectedItem} addTask={addTask} />
          
          {/* Task description */}
          {selectedItem && (
            <form className="task-description" datatype="taskModel" onSubmit={(e) => updateTask(e as React.FormEvent<HTMLFormElement>)}>
              <div className="task-title-container">
                <input className="task-title-input" type="text" value={selectedItem.title} name="task-title" onChange={handleChange}/>
              </div>
              <div className="task-description-container">
                <label className="task-description-label">Description:</label>
                <textarea className="task-description-input" value={selectedItem.description} name="task-description" onChange={handleChange}/>
              </div>
              <div className="task-status-container">
                <select className="task-status-input" value={selectedItem.status} name="task-status" onChange={handleChange}>
                  <option value="idle">Idle</option>
                  <option value="in-progress">In Progress</option>
                  <option value="done">Done</option>
                </select>
              </div>
              <div className="task-auto-run-container">
                <input className="task-auto-run-input" type="checkbox" checked={selectedItem.auto_run} name="task-auto-run" onChange={handleChange}/>
                <label className="task-auto-run-label"> Auto Run</label>
              </div>
              <div className="task-interval-container">
                <input className="task-interval-input" type="number" value={selectedItem.auto_run_interval} name="task-auto-run-interval" onChange={handleChange}/>
                <label className="task-interval-label"> Minutes Interval</label>
              </div>
              <div className="task-number-of-dups-container">
                <input className="task-number-of-dups-input" type="number" value={selectedItem.number_of_dup_to_keep} name="task-number-of-dups" onChange={handleChange}/>
                <label className="task-number-of-dups-label"> Number of Duplicates to Keep</label>
              </div>
              <div className="task-regex-patterns-container">
                <label className="task-regex-patterns-label">Regex Patterns:</label>
                <textarea className="task-regex-patterns-input" value={selectedItem.regex_patterns.join("\n")} name="task-regex-patterns" onChange={handleChange}/>
              </div>
              <div className="task-folder-paths-container">
                <label className="task-folder-paths-label">Folder Paths:</label>
                <textarea className="task-folder-paths-input" value={selectedItem.folder_paths.join("\n")} name="task-folder-paths" onChange={handleChange}/>
              </div>
              <div className="task-save-button-container">
                <button className="task-save-button" type="submit">Save</button>
              </div>
              
            </form>
          )}
          </div>
      </div>
    </main>
  );
}

export default App;
