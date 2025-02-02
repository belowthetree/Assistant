import { fetch} from "@tauri-apps/plugin-http"
import { readTextFileAtProjectTemp } from "@/lib/llm_action"
import { LLMBase } from "@/model/llm_base"
import { EModelType } from "@/data"

export class DeepSeek extends LLMBase {
    constructor(url: string = "https://api.deepseek.com/chat/completions", model: string = "deepseek-chat", roleCard: string = "", api: string = "") {
        super(url, model, roleCard, api)
        this.modelType = EModelType.DeepSeek
    }

    async chat(content:string, temperature?:number, system?:string): Promise<string> {
        console.log("deep input", content)
        const res = await this.generate(content, temperature, system, this.messages)
        this.messages.push({
            "role": "assistant",
            "content": res,
        })
        return res
    }

    async generate(content: string, temperature?:number, system?:string, ctx?:any): Promise<string> {
        console.log(content)
        if (this.api_key.length <= 0) {
            console.log("没有 api key")
            const res = await readTextFileAtProjectTemp("resources/api_key")
            this.api_key = "Bearer " + res
        }
        const messages = ctx || []
        if (!system) {
            system = this.roleCard.systemPrompt
        }
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
        console.log(response, this.modelName)

        if (response.status == 200) {
            const text = await response.text()
            const js = JSON.parse(text)
            if (js.choices) {
                return Promise.resolve(js.choices[0].message.content)
            }
        }
        return Promise.reject(response.text())
    }
}