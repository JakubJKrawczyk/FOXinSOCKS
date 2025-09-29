import {taskModel, TaskStatus} from "../backend/models/taskModel"

interface TasksListProps{
  t: taskModel[];
  selectTask: (e: React.MouseEvent<HTMLTableRowElement, MouseEvent>, task: taskModel) => void;
  addTask: () => void;
  delTask: (id: string) => void;
  runTask: (id: string) => void;
  pauseTask?: (id: string) => void; // opcjonalnie – jeśli nie podane, przycisk Pause nie działa
}

export default function TasksList({ t, selectTask, addTask, delTask, runTask, pauseTask }: TasksListProps){
       /**
     *
     */
    
       return (
        <>
        <div className="tasks-list">
            <table>
              <tbody className="tasks-table-items">
                {t.map((task, index) => {
                  const isRunning = task.status === TaskStatus.InProgress || task.status === TaskStatus.Sheduled;
                  const runPauseLabel = isRunning ? "⏸" : "▶";
                  const runPauseTitle = isRunning ? "Pause" : "Run";
                  const onRunPauseClick = (e: React.MouseEvent) => {
                    e.stopPropagation();
                    if (isRunning) {
                      pauseTask && pauseTask(task.id);
                    } else {
                      runTask(task.id);
                    }
                  };
                  return (
                    <tr key={index} className="task-item task-selectable" onClick={(e) => selectTask(e, task)}>
                      <td>
                        <label> {task.title}</label>
                      </td>
                      <td className="task-item-status">{task.status}</td>
                      <td className="task-item-run" title={runPauseTitle} onClick={onRunPauseClick}>{runPauseLabel}</td>
                      <td className="task-item-del" onClick={(e) => { e.stopPropagation(); delTask(task.id); }}>X</td>
                    </tr>
                  );
                })}
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