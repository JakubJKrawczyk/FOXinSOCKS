import "./style/App.css";
import taskModel from "./backend/models/taskModel";
import { useState  } from "react";

function App() {

  // VARIABLES
  const [selectedItem, setSelectedItem] = useState<taskModel | null>(null);

  const tasks : taskModel[] = [
    new taskModel(1, "Clean temp files", "Cleans all temp files from your system", true, 101, "sheduled"),
    new taskModel(2, "Backup Documents", "Backs up all documents to external drive", false, 102, "idle"),
    new taskModel(3, "Update Software", "Updates all installed software to the latest version", true, 103, "in-progress"),
    new taskModel(4, "Scan for Viruses", "Scans the system for viruses and malware", false, 104, "done"),
    new taskModel(5, "Optimize Performance", "Optimizes system performance by cleaning up unnecessary files", true, 105, "idle"),
  ];

  // EVENTS
  document.onclick = (e) => {
    if(
      (e.target as HTMLElement).className.includes("task-item") === false &&
     (e.target as HTMLElement).className.includes("task-description") === false
    ) {
      console.log("clicked outside");
      setSelectedItem(null);
    }
  };

 

  // FUNCTIONS

  function selectTask(index: number, e: React.MouseEvent<HTMLTableRowElement, MouseEvent>, task: taskModel) {
  
    console.log("Selected task:", task);
    var element = e.currentTarget as HTMLTableRowElement;
    let listElements = document.getElementsByClassName("task-item");

    for (let i = 0; i < listElements.length; i++) {
      listElements[i].setAttribute("className", "task-item");
    }
    
    if(element.className !== "task-item-add-new task-item") {
      element.setAttribute("className", "task-item selected-task-item");
      setSelectedItem(task);
    }
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
          <div className="tasks-list">
            <table>
              <tbody className="tasks-table-items">
                {tasks.map((task, index) => (
                  <tr key={index} className="task-item" onClick={(e) => selectTask(index, e, task)}>
                    <td>
                      <label> {task.title}</label>
                    </td>
                    <td className="task-item-status">{task.status}</td>
                  </tr>
                ))}
                <tr>
                  <td className="task-item-add-new task-item">
                    Add new...
                  </td>
                </tr>
              </tbody>
            </table>
          </div>

          {/* Task description */}
          {selectedItem && (
            <div className="task-description">
              <div>
                <input className="task-title-input" type="text" defaultValue={selectedItem.title} />
              </div>
              <div>
                <textarea className="task-description-input" defaultValue={selectedItem.description} />
              </div>
              <div>
                <select className="task-status-input" defaultValue={selectedItem.status}>
                  <option value="idle">Idle</option>
                  <option value="in-progress">In Progress</option>
                  <option value="done">Done</option>
                </select>
              </div>
            </div>
          )}
          </div>
      </div>
    </main>
  );
}

export default App;
