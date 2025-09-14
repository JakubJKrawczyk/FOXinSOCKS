import taskModel from "../backend/models/taskModel"

interface TasksListProps{
    t: taskModel[];
    setSelectedItem: React.Dispatch<React.SetStateAction<taskModel | null>>;
    addTask: () => void;
}

export default function TasksList({ t, setSelectedItem, addTask }: TasksListProps){
       /**
     *
     */
    function selectTask(e: React.MouseEvent<HTMLTableRowElement, MouseEvent>, task: taskModel) {
    
        console.log("Selected task:", task);
        var element = e.currentTarget as HTMLTableRowElement;
        let listElements = document.getElementsByClassName("task-item");

        for (let i = 0; i < listElements.length; i++) {
        listElements[i].setAttribute("class", "task-item");
        }
        
        element.setAttribute("class", "task-item selected-task-item");
        setSelectedItem(task)
    }
       return (
        <>
        <div className="tasks-list">
            <table>
              <tbody className="tasks-table-items">
                {t.map((task, index) => (
                  <tr key={index} className="task-item" onClick={(e) => selectTask(e, task)}>
                    <td>
                      <label> {task.title}</label>
                    </td>
                    <td className="task-item-status">{task.status}</td>
                  </tr>
                ))}
                <tr>
                  <td className="task-item-add-new task-item" onClick={() => addTask()}>
                    Add new...
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </>
       )
}