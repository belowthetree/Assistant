
export class Theme {
    content: string

    constructor(content: string) {
        this.content = content
    }

    getSystemPrompt(): string {
        return this.content
    }
}