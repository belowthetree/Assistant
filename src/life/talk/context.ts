import { ModelMessage, ModelResponse } from "../../model/modelbase"

export enum ERole {
    User = "user",
    Assistant = "assistant",
    System = "system",
}

export interface TalkContentInterface {
    role: ERole
    content: string
    reasoning_content: string
    tool_calls: [
        {
            id: string,
            type: string,
            function: {
                name: string,
                arguments: string
            }
        }
    ]
}
export class TalkContent implements TalkContentInterface {
    role: ERole
    content: string = ""
    reasoning_content: string
    tool_calls: [
        {
            id: string,
            type: string,
            function: {
                name: string,
                arguments: string
            }
        }
    ]

    constructor(ct: TalkContentInterface) {
        this.role = ct.role
        this.content = ct.content
        this.reasoning_content = ct.reasoning_content
        this.tool_calls = ct.tool_calls
    }

    append(content: string) {
        if (this.content.length > 0) {
            this.content += "\n" + content
        }
        else {
            this.content = content
        }
    }
}

export class TalkContext {
    content: TalkContent[] = []

    addContent(ct: TalkContent) {
        this.content.push(ct)
    }

    addUser(res: ModelResponse) {
        this.content.push(new TalkContent({
            role: ERole.User,
            content: res.content,
            reasoning_content: res.reasoning_content,
            tool_calls: res.tool_calls,
        }))
    }

    addAssistant(res: ModelResponse) {
        this.content.push(new TalkContent({
            role: ERole.Assistant,
            content: res.content,
            reasoning_content: res.reasoning_content,
            tool_calls: res.tool_calls,
        }))
    }

    addSystem(res: ModelResponse) {
        this.content.push(new TalkContent({
            role: ERole.System,
            content: res.content,
            reasoning_content: res.reasoning_content,
            tool_calls: res.tool_calls,
        }))
    }

    tostring(): string {
        let res = ""
        for (const ct of this.content) {
            res += ct + "\n"
        }
        res.trim()
        return res
    }

    tojson(): ModelMessage[] {
        let messages = []
        for (const ct of this.content) {
            messages.push({
                "role": ct.role.toString().trim(),
                "content": ct.content.trim()
            })
        }
        return messages
    }
}