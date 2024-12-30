import { DeepSeek } from "../lib/deepseek"
import { generate } from "../lib/llm_interface"
import { TsCommandPrompt, TsPrompt } from "../prompt/typescript_prompt"
import { invoke } from "@tauri-apps/api/core"
import { fetch} from "@tauri-apps/plugin-http"

enum EProcessMode
{
    Direct,
    SequenceCommand,
}

export class TypescriptProcess {
    question: string = ""
    model: string = "qwen2.5-coder:latest"
    answer: string = ""
    waiting: boolean = false
    errorInfo: string = ""
    tryTimes: number = 0
    maxTryTimes: number = 3
    deepseek: DeepSeek = new DeepSeek()
    index: number = 0
    mode: EProcessMode = EProcessMode.Direct

    constructor(model?: string, question?: string, maxTryTimes?: number) {
        this.waiting = false
        this.model = model || "qwen2.5-coder:latest"
        this.maxTryTimes = maxTryTimes || 3
        if (question)
            this.inputQuestion(question)
    }

    inputQuestion(question:string): void {
        if (this.waiting)
            return
        this.mode = EProcessMode.Direct
        console.log("inputQuestion", question)
        this.waiting = true
        this.question = question
        const handle = (e:string)=>{
            this.waiting = false
            this.answer = e
            e = e.replace("\`\`\`typescript", "")
            e = e.replace("\`\`\`", "")
            // e = e.replace(new RegExp("const", 'g'), "let")
            e = e.trim()
            console.log(e)
            try {
                new Function('invoke', 'fetch', e)(invoke, fetch)
            }
            catch (e) {
                console.error(e)
                this.errorInfo = e as string
                this.requestion()
            }
        }
        this.generateCode(this.question).then(handle)
    }

    // 改错
    requestion() {
        if (this.waiting)
            return
        this.waiting = true
        this.tryTimes++
        if (this.tryTimes > this.maxTryTimes)
            return
        const handle = (e:string)=>{
            this.waiting = false
            this.answer = e
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
        }
        this.generateCode(this.answer + "报错：" + this.errorInfo + "。请重新给出答案").then(handle)
    }

    async generateCode(ctx:string):Promise<string> {
        let res = await this.generate(this.model, ctx, false, TsPrompt, 0)
        // this.deepseek.generate(this.answer + "报错：" + this.errorInfo + "。请重新给出答案", 2, TsPrompt).then(handle)
        // this.deepseek.generate(this.question, 1, TsPrompt).then(handle)

        res = res.replace("\`\`\`typescript", "")
        res = res.replace("\`\`\`", "")
        res = res.trim()
        console.log(res)
        return res
    }

    async generateCommands(question:string) {
        if (this.waiting)
            return
        this.index = 0
        this.waiting = true
        this.mode = EProcessMode.SequenceCommand
        this.question = question
        // const res = await this.deepseek.generate(this.question, 1, TsCommandPrompt)
        let res = await this.generate(this.model, this.question, false, TsCommandPrompt, 1)
        res = res.replace("\`\`\`json", "")
        res = res.replace("\`\`\`", "")
        res = res.trim()
        console.log(res)
        this.answer = res
        this.waiting = false
    }

    async nextCommand() {
        if (this.waiting || this.mode != EProcessMode.SequenceCommand)
            return
        this.waiting = true
        const js = JSON.parse(this.answer)
        if (this.index >= js.length)
            return
        console.log("当前：" + js[this.index])
        this.generateCode(`问题是：${this.question}，所有步骤为${this.answer} 请执行:${js[this.index]}`)
        this.index++
        this.waiting = false
    }

    generate(model: string, ctx: string, stream:boolean, system?: string, temperature?: number):Promise<string> {
        if (model == "deepseek") {
            return this.deepseek.generate(ctx, temperature || 1, system)
        }
        else
            return generate(model, ctx, stream, system, undefined, temperature)
    }
}