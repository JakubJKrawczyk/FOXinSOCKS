import {v4 as uuidv4} from 'uuid';
enum TaskStatus {
    Sheduled = "sheduled",
    InProgress = "in-progress",
    Idle = "idle",
    Done = "done"
}

class taskModel {
    id: string;
    title: string;
    description: string;
    auto_run: boolean;
    auto_run_interval: number; // in minutes
    task_process_id: number;
    status: TaskStatus;
    regex_patterns: string[];
    folder_path: string;
    number_of_dup_to_keep: number;


    constructor(
        taskId: string = uuidv4(),
        taskTitle: string = "task",
        taskDescription: string = "",
        taskAutoRun: boolean = false,
        taskProcessId: number = 0,
        taskStatus: TaskStatus = TaskStatus.Idle,
        auto_run_interval: number = 60,
        regex_patterns: string[] = [],
        folder_path: string = "",
        number_of_dup_to_keep: number = 2
    ) {
        this.id = taskId;
        this.title = taskTitle;
        this.description = taskDescription;
        this.auto_run = taskAutoRun;
        this.task_process_id = taskProcessId;
        this.status = taskStatus;
        this.auto_run_interval = auto_run_interval;
        this.regex_patterns = regex_patterns;
        this.folder_path = folder_path;
        this.number_of_dup_to_keep = number_of_dup_to_keep;
    }
}

export {taskModel, TaskStatus};