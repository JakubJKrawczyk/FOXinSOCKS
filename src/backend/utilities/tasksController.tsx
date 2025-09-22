declare global {
    interface Window {
        __TAURI__: {
            invoke: (cmd: string, args?: any) => Promise<any>;
        };
    }
}

class TasksController {
    async init(): Promise<void> {
        return await window.__TAURI__.invoke("init");
    }

    async getTasks(): Promise<any[]> {
        return await window.__TAURI__.invoke("get_tasks");
    }

    async getTask(task_id: string): Promise<any> {
        return await window.__TAURI__.invoke("get_task", { task_id });
    }

    async addTask(task_to_add: any): Promise<void> {
        return await window.__TAURI__.invoke("add_task", { task_to_add });
    }

    async delTask(task_id: string): Promise<void> {
        return await window.__TAURI__.invoke("del_task", { task_id });
    }

    async runTask(task_id: string): Promise<void> {
        return await window.__TAURI__.invoke("run_task", { task_id });
    }

    async stopTask(task_id: string): Promise<void> {
        return await window.__TAURI__.invoke("stop_task", { task_id });
    }
}

export default TasksController