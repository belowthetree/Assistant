
export class Task {
    target: string
    result: string = ""

    constructor(target: string) {
        this.target = target
    }

    isFinish(): boolean {
        return this.result.length > 0
    }
}