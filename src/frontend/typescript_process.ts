import { DeepSeek } from "@/model/deepseek"
import { generate } from "../lib/llm_interface"
import { ModulePrompt } from "../prompt/module_prompt"
import { TsCommandPrompt } from "../prompt/typescript_prompt"
import { invoke } from "@tauri-apps/api/core"
import * as moduleAction from "../lib/llm_action"
import { LLMBase } from "../model/llm_base"

enum EProcessMode
{
    Direct,
    SequenceCommand,
    Exec,
}

export class TypescriptProcess {
    question: string = ""
    model: string = "qwen2.5-coder:latest"
    answer: string = ""
    waiting: boolean = false
    errorInfo: string = ""
    tryTimes: number = 0
    maxTryTimes: number = 3
    deepseek: LLMBase = new DeepSeek()
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
            e = e.replace("\`\`\`javascript", "")
            e = e.replace("\`\`\`", "")
            // e = e.replace(new RegExp("const", 'g'), "let")
            e = e.trim()
            console.log(e)
            try {
                new Function('invoke', 'fetch', e)(invoke, fetch)
            }
            catch (e) {
                let errorInfo = ""
                if (e instanceof Error) {
                    console.log(e.stack)
                    errorInfo = e.stack || ""
                }
                else {
                    console.error(e)
                    errorInfo = e as string
                }
                this.errorInfo = errorInfo
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
                let errorInfo = ""
                if (e instanceof Error) {
                    console.log(e.stack)
                    errorInfo = e.stack || ""
                }
                else {
                    console.error(e)
                    errorInfo = e as string
                }
                this.errorInfo = errorInfo
                this.requestion()
            }
        }
        this.generateCode(this.answer + "报错：" + this.errorInfo + "。请重新给出答案").then(handle)
    }

    async generateCode(ctx:string):Promise<string> {
        let res = await this.generate(this.model, ctx, false, ModulePrompt, 0)
        // this.deepseek.generate(this.answer + "报错：" + this.errorInfo + "。请重新给出答案", 2, TsPrompt).then(handle)
        // this.deepseek.generate(this.question, 1, TsPrompt).then(handle)

        res = res.replace("\`\`\`javascript", "")
        res = res.replace("\`\`\`", "")
        res = res.trim()
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

    async generateModule(question: string) {
        if (this.waiting)
            return
        this.mode = EProcessMode.Exec
        console.log("generateModule", question)
        this.waiting = true
        this.question = question
        const handle = async (e:string)=>{
            this.waiting = false
            this.answer = e
            e = e.replace("\`\`\`javascript", "")
            e = e.replace("\`\`\`", "")
            e = e.trim()
            console.log(e)
            try {
                let funcStr = []
                let funcs = []
                for (let key of Object.keys(moduleAction)) {
                    if (typeof moduleAction[key] === "function") {
                        funcStr.push(key)
                        funcs.push(moduleAction[key])
                    }
                }
                new Function(...funcStr, e)(...funcs)
            }
            catch (e) {
                let errorInfo = ""
                if (e instanceof Error) {
                    console.log(e.stack)
                    errorInfo = e.stack || ""
                }
                else {
                    console.error(e)
                    errorInfo = e as string
                }
                this.errorInfo = errorInfo
                moduleAction.notify("第一次尝试失败", "正在重试.....")
                this.tryTimes = 0
                this.regenerateModule()
            }
        }
        this.generateCode(this.question).then(handle)
        // moduleAction.searchWeb(question)
    }

    async regenerateModule() {
        if (this.waiting)
            return
        if (this.mode != EProcessMode.Exec)
            return
        if (this.tryTimes >= this.maxTryTimes) {
            moduleAction.notify(`执行命令${this.question}失败`, `原因：${this.errorInfo}`)
            console.log("达到最大重试次数，放弃重试")
            return
        }
        this.tryTimes++
        console.log("regenerateModule", this.question)
        const handle = async (e:string)=>{
            this.waiting = false
            this.answer = e
            e = e.replace("\`\`\`javascript", "")
            e = e.replace("\`\`\`", "")
            e = e.trim()
            console.log(e)
            try {
                let funcStr = []
                let funcs = []
                for (let key of Object.keys(moduleAction)) {
                    if (typeof moduleAction[key] === "function") {
                        funcStr.push(key)
                        funcs.push(moduleAction[key])
                    }
                }
                new Function(...funcStr, e)(...funcs)
            }
            catch (e) {
                let errorInfo = ""
                if (e instanceof Error) {
                    console.log(e.stack)
                    errorInfo = e.stack || ""
                }
                else {
                    console.error(e)
                    errorInfo = e as string
                }
                this.errorInfo = errorInfo
                moduleAction.notify(`第${this.tryTimes}次尝试失败`, "正在重试.....")
                this.regenerateModule()
            }
        }
        this.generateCode(`问题${this.question}的回答：${this.answer}报错：${this.errorInfo}。请重新输出正确的代码（不要生成代码以外的东西）`).then(handle)
    }
}