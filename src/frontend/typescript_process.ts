import { generate } from "../lib/llm_interface"
import { TsPrompt } from "../prompt/typescript_prompt"
import { invoke } from "@tauri-apps/api/core"

``

export class TypescriptProcess {
    question: string = ""
    model: string = "qwen2.5-coder:latest"
    answer: string = ""
    valid: boolean = false
    errorInfo: string = ""
    tryTimes: number = 0
    maxTryTimes: number = 3

    constructor(model?: string, question?: string, maxTryTimes?: number) {
        this.model = model || "qwen2.5-coder:latest"
        this.maxTryTimes = maxTryTimes || 3
        if (question)
            this.inputQuestion(question)
    }

    inputQuestion(question:string): void {
        console.log("inputQuestion", question)
        this.valid = false
        this.question = question
        generate(this.model, this.question, false, TsPrompt).then((e)=>{
            console.log(e)
            this.answer = e
            e = e.replace("\`\`\`typescript", "")
            e = e.replace("\`\`\`", "")
            // e = e.replace(new RegExp("const", 'g'), "let")
            // console.log(e)
            try {
                new Function('invoke', e)(invoke)
            }
            catch (e) {
                console.error(e)
                this.errorInfo = e as string
                this.requestion()
            }
        })
    }

    // 改错
    requestion() {
        this.tryTimes++
        if (this.tryTimes > this.maxTryTimes)
            return
        generate(this.model, this.answer + "报错：" + this.errorInfo + "。请重新给出答案", false, TsPrompt, undefined, 2.8).then((e)=>{
            console.log(e)
            this.answer = e
            e = e.replace("\`\`\`typescript", "")
            e = e.replace("\`\`\`", "")
            // e = e.replace(new RegExp("const", 'g'), "let")
            // console.log(e)
            try {
                new Function('invoke', e)(invoke)
            }
            catch (e) {
                console.error(e)
                this.errorInfo = e as string
                this.requestion()
            }
        })
    }
}