import "./style/App.css";
import {taskModel, TaskStatus} from "./backend/models/taskModel";
import { useEffect, useState  } from "react";
import TasksList from "./components/TasksList";
import TasksController from "./backend/utilities/tasksController";
import LogDrawer from "./components/LogDrawer";

function App() {

    const [refreshRunning, setrefreshRunning] = useState<boolean>(false);

    if(!refreshRunning){
      runTaskWithInterval(refreshTasks, 5000);
      setrefreshRunning(true);
    }

    // VARIABLES

    const [selectedItem, setSelectedItem] = useState<taskModel | null>(null);
  const [tasks, setTasks] = useState<taskModel[]>([]);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  const [errorTimerId, setErrorTimerId] = useState<number | null>(null);
  const [showLogDrawer, setShowLogDrawer] = useState<boolean>(false);
  const [logText, setLogText] = useState<string>("");
  const [logAutoScroll, setLogAutoScroll] = useState<boolean>(true);

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

  // On tasks change: highlight last
  useEffect(() => {
    const listElements = document.getElementsByClassName("task-selectable");
    if(listElements.length === 0) return;
    const element = listElements[listElements.length - 1] as HTMLElement;
    selectFrontItem(element)
  },[tasks]);

  // helper to show error toast
  function showError(msg: string){
    console.error(msg);
    setErrorMsg(msg);
  }

  // auto hide error after 5s
  useEffect(() => {
    if(errorMsg){
      if(errorTimerId){ clearTimeout(errorTimerId); }
      const id = window.setTimeout(() => setErrorMsg(null), 5000);
      setErrorTimerId(id);
    }
  }, [errorMsg]);

  // global listeners for unexpected errors
  useEffect(() => {
    const onWindowError = (e: ErrorEvent) => { showError(e.message || "Nieznany błąd (window error)"); };
    const onUnhandled = (e: PromiseRejectionEvent) => { showError(e.reason?.toString?.() || "Unhandled promise rejection"); };
    window.addEventListener('error', onWindowError);
    window.addEventListener('unhandledrejection', onUnhandled);
    // przechwycenie console.error (dowolny błąd z konsoli też pokaże toast)
    const originalConsoleError = console.error;
    console.error = (...args: any[]) => {
      try { showError(args.map(a => (typeof a === 'string' ? a : (a?.message || a?.toString?.() || ''))).join(' ')); } catch(_) {}
      originalConsoleError(...args);
    };
    // skrót klawiszowy Ctrl+Shift+E -> testowy toast
    const keyHandler = (e: KeyboardEvent) => {
      if(e.ctrlKey && e.shiftKey && (e.key === 'E' || e.key === 'e')){
        showError('Testowy błąd (CTRL+SHIFT+E)');
      }
    };
    window.addEventListener('keydown', keyHandler);
    return () => {
      window.removeEventListener('error', onWindowError);
      window.removeEventListener('unhandledrejection', onUnhandled);
      window.removeEventListener('keydown', keyHandler);
      console.error = originalConsoleError;
    };
  }, []);

  // Initial backend init + load tasks + log path
  useEffect(() => {
    (async () => {
      const initRes = await controller.init();
      if(!initRes.ok){ showError("Init backend error: " + initRes.error); }
      const tasksRes = await controller.getTasks();
      if(tasksRes.ok && 'data' in tasksRes){ setTasks(tasksRes.data); } else { showError("Pobieranie tasków nieudane"); }
      const logRes = await controller.getLogPath();
      if(logRes.ok && 'data' in logRes){ console.log("Ścieżka pliku logu:", logRes.data); }
    })();
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // FUNCTIONS


  function runTaskWithInterval(task: () => Promise<void>, interval: number): void {
    let isRunning = false;

    setInterval(async () => {
      console.log("Odświeżanie listy tasków.")

        if (isRunning) {
            console.log("Task is already running. Skipping this interval.");
            return;
        }

        isRunning = true;
        try {
            await task(); // Run the async task
        } catch (error) {
            console.error("Error occurred during task execution:", error);
        } finally {
            isRunning = false; // Reset the flag after task completion
        }
    }, interval);
}

  async function refreshTasks(){
    const res = await controller.getTasks();
    if(res.ok && 'data' in res){ setTasks(res.data); }
    else { showError('Nie udało się pobrać tasków po operacji'); }
  }

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

    

  async function updateTask() {
    if(!selectedItem) return;
    console.log("Updating task...", selectedItem.id);
    selectedItem.regex_patterns = selectedItem.regex_patterns.filter(pattern => pattern.length > 0);
    selectedItem.folder_path = selectedItem.folder_path.trim();
    const res = await controller.updateTask(selectedItem);
    if(!res.ok){ showError('Błąd aktualizacji backend: ' + res.error); }
    await refreshTasks();
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
          updated.regex_patterns = value.split("\n");
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

  async function newTask(){
    console.log("Adding new task");
    const res = await controller.addTask();
    if(!res.ok){ showError('Błąd dodawania na backend: ' + res.error); return; }
    // Użyj obiektu z backendu aby ID było spójne
    const backendTask = 'data' in res ? res.data : null;
    await refreshTasks();
    if(backendTask){
      setSelectedItem(backendTask as taskModel);
    }
  }

  async function delTask(id: string){
    console.log("Deleting task with id: " + id);
    const res = await controller.delTask(id);
  if(!res.ok){ showError('Błąd usuwania: ' + res.error); }
    await refreshTasks();
    if(selectedItem && selectedItem.id === id){ setSelectedItem(null); }
  }

  async function runTask(id: string){
    console.log("Running task with id: " + id);
    const res = await controller.runTask(id);
  if(!res.ok){ showError('Błąd uruchamiania: ' + res.error); }
    else console.log("Task uruchomiony: " + id);
    await refreshTasks();
  }

  async function stopTask(id: string){
    console.log("Stopping task with id: " + id);
    const res = await controller.stopTask(id);
  if(!res.ok){ showError('Błąd zatrzymywania: ' + res.error); }
    else console.log("Task zatrzymany: " + id);
    await refreshTasks();
  }

  // Pobieranie logów (cała zawartość pliku) – wywoływane przy otwieraniu oraz w interwale.
  async function fetchLogs(){
    // pobieramy tylko ostatnie 400 linii dla oszczędności pamięci
    const res = await controller.getLogTail(400);
    if(res.ok && 'data' in res){
      setLogText(res.data);
    }
  }

  // Auto-refresh logów kiedy szuflada otwarta (co 0.5 sekundy)
  useEffect(() => {
    if(!showLogDrawer) return;
    // pierwsze pobranie
    fetchLogs();
    const interval = window.setInterval(fetchLogs, 500); // co 0.5 sekundy
    return () => window.clearInterval(interval);
  }, [showLogDrawer]);

  // Po zamknięciu szuflady zwolnij pamięć
  useEffect(() => {
    if(!showLogDrawer){
      setLogText("");
    }
  }, [showLogDrawer]);
  
  // RENDER
  return (
    <main>
      {/* Main container */}
      <div className="container-main">
        {/* Top Container */}
        <div className="container-top" > 
          <h1 className="title">Fox in Socks</h1>
          <p className="subtitle">A simple cleaner at your service</p>
          <div style={{ display: 'flex', gap: '10px' }}>
            <button type="button" onClick={() => setShowLogDrawer(s => !s)}>
              {showLogDrawer ? 'Ukryj logi' : 'Pokaż logi'}
            </button>
            {showLogDrawer && (
              <button type="button" onClick={() => fetchLogs()}>Odśwież logi</button>
            )}
          </div>
        </div>
        {/* Bottom Container */}
        <div className="container-bottom"> 
          {/* List of tasks */}

          <TasksList t={tasks} selectTask={selectTask} addTask={newTask} delTask={delTask} runTask={runTask} stopTask={stopTask}/>

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
      {errorMsg && (
        <div className="error-toast" role="alert" aria-live="assertive" onClick={() => setErrorMsg(null)}>
          <span className="error-toast-msg">{errorMsg}</span>
          <button className="error-toast-close" type="button" onClick={(e) => { e.stopPropagation(); setErrorMsg(null); }}>×</button>
        </div>
      )}
  <LogDrawer open={showLogDrawer} logs={logText} onClose={() => setShowLogDrawer(false)} autoScroll={logAutoScroll} setAutoScroll={setLogAutoScroll} />
    </main>
  );
}

export default App;
