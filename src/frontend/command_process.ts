import { invoke } from "@tauri-apps/api/core"
import { generate, generate_instructions } from "../lib/llm_interface"
import { Instructions } from "../prompt/function_prompt"

export class CommandProcess {
    question: string = ""
    model: string = "qwen2.5-coder:latest"

    commands: string[] = []
    valid: boolean = false
    commandIndex: number = 0

    commandToFunc: Map<string, string> = new Map<string, string>()
    lastResult: String = ""

    constructor(model?: string, question?: string) {
        this.model = model || "qwen2.5-coder:latest"
        if (question)
            this.inputQuestion(question)
    }

    inputQuestion(question:string): void {
        console.log("inputQuestion", question)
        this.valid = false
        this.question = question
        this.commandIndex = 0
        this.commandToFunc.clear()
        generate_instructions(this.model, question, false).then((res)=>{
            this.valid = true
            const js = JSON.parse(res.toString())
            console.log(js)
            if (js.instructions) {
                this.commands = js.instructions
                for (let cmd of this.commands.values()) {
                    for (let ins of Instructions.values()) {
                        if (ins.Desc.ability == cmd) {
                            console.log(cmd, ins.name)
                            this.commandToFunc.set(cmd, ins.name)
                        }
                    }
                }
            } else {
                this.commands = []
            }
        })
    }

    async invokeCommand(): Promise<String> {
        if (this.commandIndex >= this.commands.length) {
            return this.commandIndex.toString()
        }
        const cmd = this.commands[this.commandIndex]
        const name = this.commandToFunc.get(cmd)
        this.commandIndex++
        console.log(name, this.lastResult)
        if (name) {
            this.lastResult = await invoke(name, {arg : this.lastResult})
            return this.lastResult
        } else {
            var res: string
            if (this.commandIndex >= 2 && this.lastResult.length > 0) {
                const lastCmd = this.commands[this.commandIndex - 2]
                res = await generate(this.model, "\"" + this.lastResult + "\"这是命令\"" + lastCmd + "\"的结果，请对这些结果执行命令\"" + cmd + "\"", false)
            } else {
                res = await generate(this.model, "请执行命令\"" + cmd + "\"", false)
            }
            const js = JSON.parse(res)
            this.lastResult = js.result || ""
            return this.lastResult
        }
    }
}