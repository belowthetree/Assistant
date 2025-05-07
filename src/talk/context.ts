import { ModelMessage } from "../model/llm_base"

export enum ERole {
    User = "user",
    Assistant = "assistant",
    System = "system",
}

export interface TalkContentInterface {
    role: ERole
    content: string
}
export class TalkContent implements TalkContentInterface {
    role: ERole
    content: string = ""

    constructor(ct: TalkContentInterface) {
        this.role = ct.role
        this.content = ct.content
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

    addUser(content: string) {
        console.log("user ", content)
        this.content.push(new TalkContent({
            role: ERole.User,
            content,
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