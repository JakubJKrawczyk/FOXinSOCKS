import { invoke } from "@tauri-apps/api/core";
import {taskModel} from "../models/taskModel";

type Success<T> = { ok: true; data: T };
type SuccessVoid = { ok: true };
type Failure = { ok: false; error: string };

class TasksController {
    // Uniwersalny wrapper
    private async call<T = void>(cmd: string, args?: any): Promise<Success<T> | SuccessVoid | Failure> {
        try {
            const res = await invoke(cmd, args);
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

    async getTask(taskId: string) {
        return this.call<taskModel>("get_task", { taskId });
    }

    async addTask() {
        return this.call("add_task");
    }

    async delTask(taskId: string) {
        return this.call("del_task", { taskId });
    }

    async runTask(taskId: string) {
        return this.call("run_task", { taskId });
    }

    async stopTask(taskId: string) {
        return this.call("stop_task", { taskId });
    }

    async updateTask(task: taskModel) {
        return this.call("update_task", { task });
    }

    async getLogPath(){
        return this.call<string>("get_log_path");
    }

    async getLogs(){
        return this.call<string>("get_all_logs");
    }

    async getLogTail(lines: number){
        return this.call<string>("get_log_tail", { lines });
    }
}

export default TasksController;