import { EModelType } from "@/data"
import { notify } from "../lib/llm_action"
import * as moduleAction from "../lib/llm_action"
import { RoleCardBase } from "../rolecard/rolecardbase"
import {fetch} from "@tauri-apps/plugin-http"
import { ModulePrompt } from "../prompt/module_prompt"
import { getRoleCard } from "../config"

export interface LLMInterface {
    modelType: EModelType
    getModelName():string
    setModelName(name: string)
    getModels():Promise<string[]>
    setApiKey(key: string)
    checkApiKeyValid(): Promise<boolean> 
    chat(content:string, temperature?:number, system?:string): Promise<string>
    generate(content: string, temperature?:number, system?:string, ctx?:any): Promise<string>
}

export class LLMBase implements LLMInterface {
    modelType: EModelType
    api_key: string = ""
    messages: any[] = []
    url: string = ""
    modelName: string = ""
    roleCard: RoleCardBase = new RoleCardBase()

    constructor(url: string, modelName: string, roleCard: string, api_key: string) {
        this.roleCard.systemPrompt = ModulePrompt
        this.url = url
        this.modelName = modelName
        this.api_key = "Bearer " + api_key
        this.roleCard = getRoleCard(roleCard)
    }

    getModelName(): string {return ""}

    setModelName(name: string) {
        this.modelName = name
    }

    getModels():Promise<string[]> {
        return Promise.resolve([])
    }

    async checkApiKeyValid(): Promise<boolean> {return Promise.resolve(true)}

    setApiKey(key: string) {
        this.api_key = "Bearer " + key
        console.log("set api key" + key)
    }

    get_chat_url(): string {
        return this.url
    }

    async chat(content:string, temperature?:number, system?:string, tools?:any[]): Promise<string> {
        console.log("chat", content, system)
        if (!this.checkApiKeyValid()) {
            notify("API Key 未设置", `模型${this.getModelName()} api key 未设置`)
        }
        if (!system) {
            system = this.roleCard.systemPrompt
        }
        const messages = this.messages || []
        messages.push(
            {"role": "system", "content": `${system || ""}`},
            {"role": "user", "content": `${content}`}
        )
        const response = await fetch(this.get_chat_url(), {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "Authorization": this.api_key,
            },
            body: JSON.stringify({
                "model": this.modelName,
                "messages": messages,
                "stream": false,
                "options": {
                    "temperature": temperature || 1
                },
                "tools": tools
            })
        })

        console.log(response)
        if (response.status == 200) {
            const text = await response.text()
            const js = JSON.parse(text)
            if (js.message) {
                const res = js.message.content
                this.messages.push({
                    "role": "assistant",
                    "content": res,
                })
                return Promise.resolve(res)
            }
            else
                return Promise.reject(text)
        }
        return Promise.reject(await response.text())
    }

    get_generate_url(): string {
        return this.url
    }

    async generate(content: string, temperature?:number, system?:string, tools?:any[]): Promise<string> {
        if (!this.checkApiKeyValid()) {
            notify("API Key 未设置", `模型${this.getModelName()} api key 未设置`)
        }
        if (!system) {
            system = this.roleCard.systemPrompt
        }
        const response = await fetch(this.get_generate_url(), {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "Authorization": this.api_key,
            },
            body: JSON.stringify({
                "model": this.modelName,
                "system": system || "",
                "prompt": content,
                "stream": false,
                "options": {
                    "temperature": temperature || 1
                },
                tools,
            })
        })

        if (response.status == 200) {
            const text = await response.text()
            const js = JSON.parse(text)
            console.log(js)
            if (js.message) {
                const code = JSON.parse(js.message.content)
                return Promise.resolve(code.code)
            }
            else if (js.response) {
                const code = JSON.parse(js.response)
                return Promise.resolve(code.code)
            }
            else
                return Promise.reject(text)
        }
        return Promise.reject(await response.text())
    }

    execute_typescript(content: string) {
        const arr = content.split("</think>")
        console.log(arr)
        if (arr.length > 1) {
            content = arr[1]
        }
        let code = content.replace("\`\`\`javascript", "")
        code = code.replace("\`\`\`", "")
        code = code.trim()
        console.log(code)
        try {
            let funcStr = []
            let funcs = []
            for (let key of Object.keys(moduleAction)) {
                if (typeof moduleAction[key] === "function") {
                    funcStr.push(key)
                    funcs.push(moduleAction[key])
                }
            }
            code = `const func = async function(){${code}}
            func()`
            new Function(...funcStr, code)(...funcs)
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
            moduleAction.notify("执行失败", errorInfo)
        }
    }
}