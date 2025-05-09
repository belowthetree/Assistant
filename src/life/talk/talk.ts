import { ModelBase, ModelMessage, ModelResponse } from "../../model/modelbase";
import { TalkContent, TalkContext } from "./context";

export enum ETalkStatus {
    Idle,
    Waiting,
}

export interface Tool {
    id: string,
    type: string,
    function: {
        name: string,
        // json 格式的参数键值对
        arguments: string
    }
}

export class Talk {
    context: TalkContext = new TalkContext()
    system: string
    status: ETalkStatus = ETalkStatus.Idle
    waitingSay: TalkContent
    model: ModelBase

    constructor(model: ModelBase, system: string) {
        setInterval(() => {
            this.tick()
        }, 0.5);
        this.model = model
        // this.context.addContent(new TalkContent({
        //     role: ERole.System,
        //     content: system,
        // }))
        console.log(model)
        this.system = system
    }

    // 内部循环
    tick() {
        if (this.status == ETalkStatus.Idle && this.waitingSay) {
            this.askAssistant()
        }
    }

    // 获取所有对话上下文
    getContextAsString(): string {
        return JSON.stringify(this.context)
    }

    // 用户输入，返回助手回复
    async userSay(content: string): Promise<ModelResponse> {
        this.context.addUser({content, role: "User"})
        try {
            const res = await this.askAssistant()
            this.context.addAssistant(res)
            return Promise.resolve(res)
        }
        catch(e) {
            console.error(e)
            this.context.addAssistant(e)
            return Promise.reject(e)
        }
    }

    // 调用模型，内部函数
    askAssistant(): Promise<ModelResponse> {
        if (this.status == ETalkStatus.Waiting) {
            return
        }
        let messages: ModelMessage[] = [
            {"role": "system", "content": this.system}
        ]
        messages.push(...this.context.tojson())
        this.status = ETalkStatus.Waiting
        return this.model.generate({
            messages,
        })
    }
}