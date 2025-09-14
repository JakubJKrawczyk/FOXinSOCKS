import taskModel from "../backend/models/taskModel"

interface TasksListProps{
    t: taskModel[];
    selectTask: (e: React.MouseEvent<HTMLTableRowElement, MouseEvent>, task: taskModel) => void;
    addTask: () => void;
}

export default function TasksList({ t, selectTask, addTask }: TasksListProps){
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