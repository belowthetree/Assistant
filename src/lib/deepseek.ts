import { fetch} from "@tauri-apps/plugin-http"
import { resolveResource } from "@tauri-apps/api/path"
import { readTextFileAtProjectRoot } from "./llm_action"

export class DeepSeek {
    url="https://api.deepseek.com/chat/completions"
    api_key: string = ""
    messages: any[] = []

    setApiKey(key: string) {
        this.api_key = "Bearer " + key
        console.log("set api key", key)
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
        if (this.api_key.length <= 0) {
            const res = await readTextFileAtProjectRoot("resources/api_key")
            this.api_key = "Bearer " + res
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
                "model": "deepseek-chat",
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