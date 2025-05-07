import { LLMBase, ModelMessage } from "../model/llm_base";
import { ERole, TalkContent, TalkContext } from "./context";

export enum ETalkStatus {
    Idle,
    Waiting,
}

export class Talk {
    context: TalkContext = new TalkContext()
    system: string
    status: ETalkStatus = ETalkStatus.Idle
    waitingSay: TalkContent
    model: LLMBase

    constructor(model: LLMBase, system: string) {
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

    tick() {
        if (this.status == ETalkStatus.Idle && this.waitingSay) {
            this.askAssistant()
        }
    }

    addContent(content:TalkContent) {
        this.context.addContent(content)
    }

    getContextAsString(): string {
        return JSON.stringify(this.context)
    }

    setModel(model: LLMBase) {
        this.model = model
    }

    userSay(content: string) {
        if (this.status == ETalkStatus.Waiting) {
            this.waitingSay.append(content)
        }
        else {
            this.context.addUser(content)
            this.askAssistant()
        }
    }

    askAssistant() {
        if (this.status == ETalkStatus.Waiting) {
            return
        }
        let messages: ModelMessage[] = [
            {"role": "system", "content": this.system}
        ]
        messages.push(...this.context.tojson())
        this.status = ETalkStatus.Waiting
        this.model.generate({
            messages,
        }).then((v)=>{
            console.log(v)
        }).finally(()=>{
            this.status = ETalkStatus.Idle
        })
    }
}