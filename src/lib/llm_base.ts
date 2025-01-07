import { notify } from "./llm_action"

export interface LLMInterface {
    getModelName():string
    setApiKey(key: string)
    checkApiKeyValid(): boolean
    chat(content:string, temperature?:number, system?:string): Promise<string>
    generate(content: string, temperature?:number, system?:string, ctx?:any): Promise<string>
}

export class LLMBase implements LLMInterface {
    api_key: string = ""
    messages: any[] = []
    url: string = ""
    modelName: string = ""

    constructor(url: string, modelName: string) {
        this.url = url
        this.modelName = modelName
    }

    getModelName(): string {return ""}

    checkApiKeyValid(): boolean {return true}

    setApiKey(key: string) {
        this.api_key = "Bearer " + key
        console.log("set api key" + key)
    }

    async chat(content:string, temperature?:number, system?:string): Promise<string> {
        const res = await this.generate(content, temperature, system, this.messages)
        this.messages.push({
            "role": "assistant",
            "content": res,
        })
        return res
    }

    async generate(content: string, temperature?:number, system?:string, ctx?:any): Promise<string> {
        console.log(content)
        if (!this.checkApiKeyValid()) {
            notify("API Key 未设置", `模型${this.getModelName()} api key 未设置`)
        }
        const messages = ctx || []
        messages.push(
            {"role": "system", "content": `${system}`},
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
                "temperature": temperature || 1
            })
        })

        if (response.status == 200) {
            const text = await response.text()
            const js = JSON.parse(text)
            if (js.choices) {
                return js.choices[0].message.content
            }
        }
        return await response.text()
    }
}