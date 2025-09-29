import {taskModel} from "../backend/models/taskModel"

interface TasksListProps{
    t: taskModel[];
    selectTask: (e: React.MouseEvent<HTMLTableRowElement, MouseEvent>, task: taskModel) => void;
    addTask: () => void;
    delTask: (id: string) => void;
    runTask: (id: string) => void;
}

export default function TasksList({ t, selectTask, addTask, delTask, runTask }: TasksListProps){
       /**
     *
     */
    
       return (
        <>
        <div className="tasks-list">
            <table>
              <tbody className="tasks-table-items">
                {t.map((task, index) => (
                  <tr key={index} className="task-item task-selectable" onClick={(e) => selectTask(e, task)}>
                    <td>
                      <label> {task.title}</label>
                    </td>
                    <td className="task-item-status">{task.status}</td>
                    <td className="task-item-run" onClick={() => runTask(task.id)}>▶</td>
                    <td className="task-item-del" onClick={() => delTask(task.id)}>X</td>
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