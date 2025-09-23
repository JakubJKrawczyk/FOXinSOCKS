import taskModel from "../models/taskModel";

declare global {
    interface Window {
        __TAURI__: {
            invoke: (cmd: string, args?: any) => Promise<any>;
        };
    }
}

type Success<T> = { ok: true; data: T };
type SuccessVoid = { ok: true };
type Failure = { ok: false; error: string };

class TasksController {
    // Uniwersalny wrapper
    private async call<T = void>(cmd: string, args?: any): Promise<Success<T> | SuccessVoid | Failure> {
        try {
            const res = await window.__TAURI__.invoke(cmd, args);
            if (res === undefined) return { ok: true };
            return { ok: true, data: res as T };
        } catch (e: any) {
            return { ok: false, error: (e?.toString?.() || "Unknown error") };
        }
    }

    async init() {
        return this.call("init");
    }

    async getTasks() {
        return this.call<taskModel[]>("get_tasks");
    }

    async getTask(task_id: string) {
        return this.call<taskModel>("get_task", { task_id });
    }

    async addTask(task_to_add: taskModel) {
        return this.call("add_task", { task_to_add });
    }

    async delTask(task_id: string) {
        return this.call("del_task", { task_id });
    }

    async runTask(task_id: string) {
        return this.call("run_task", { task_id });
    }

    async stopTask(task_id: string) {
        return this.call("stop_task", { task_id });
    }
}

export default TasksController;