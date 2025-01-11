import { EModelType } from "@/data"
import { notify } from "../lib/llm_action"
import * as moduleAction from "../lib/llm_action"

export interface LLMInterface {
    modelType: EModelType
    getModelName():string
    setApiKey(key: string)
    checkApiKeyValid(): boolean
    chat(content:string, temperature?:number, system?:string): Promise<string>
    generate(content: string, temperature?:number, system?:string, ctx?:any): Promise<string>
}

export class LLMBase implements LLMInterface {
    modelType: EModelType
    api_key: string = ""
    messages: any[] = []
    url: string = ""
    modelName: string = ""

    constructor(url: string, modelName: string, api_key: string = "") {
        this.url = url
        this.modelName = modelName
        this.api_key = api_key
    }

    getModelName(): string {return ""}

    checkApiKeyValid(): boolean {return true}

    setApiKey(key: string) {
        this.api_key = "Bearer " + key
        console.log("set api key" + key)
    }

    async chat(content:string, temperature?:number, system?:string): Promise<string> {
        console.log("chat", content)
        if (!this.checkApiKeyValid()) {
            notify("API Key 未设置", `模型${this.getModelName()} api key 未设置`)
        }
        const messages = this.messages || []
        messages.push(
            {"role": "system", "content": `${system || ""}`},
            {"role": "user", "content": `${content}`}
        )
        const response = await fetch(this.url, {
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
            })
        })

        if (response.status == 200) {
            const text = await response.text()
            const js = JSON.parse(text)
            if (js.message) {
                const res = js.message.content
                this.messages.push({
                    "role": "assistant",
                    "content": res,
                })
                return res
            }
            else
                return text
        }
        return await response.text()
    }

    async generate(content: string, temperature?:number, system?:string): Promise<string> {
        if (!this.checkApiKeyValid()) {
            notify("API Key 未设置", `模型${this.getModelName()} api key 未设置`)
        }
        const response = await fetch(this.url, {
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
            })
        })

        if (response.status == 200) {
            const text = await response.text()
            const js = JSON.parse(text)
            if (js.message) {
                return js.message.content
            }
            else
                return text
        }
        return await response.text()
    }

    execute_typescript(content) {
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