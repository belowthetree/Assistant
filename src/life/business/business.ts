import { Theme } from "./theme";
import { Task } from "./task";

export class Business {
    theme: Theme
    tasks: Task[] = []
    taskIndex: number = 0

    constructor(theme: Theme, tasks: Task[]) {
        this.theme = theme
        this.tasks = tasks
    }

    getCurrentTask(): Task|null {
        return this.tasks[this.taskIndex]
    }
}