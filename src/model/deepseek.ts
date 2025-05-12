import { fetch} from "@tauri-apps/plugin-http"
import { ModelBase, ModelInputParam, ModelResponse } from "~/src/model/modelbase"
import { EModelType } from "../data/model"

export class DeepSeek extends ModelBase {
    constructor(url: string = "https://api.deepseek.com/chat/completions", model: string = "deepseek-chat", roleCard: string = "", api: string = "") {
        super(url, model, roleCard, api)
        this.modelType = EModelType.DeepSeek
    }

    async getModels(): Promise<string[]> {
        const response = await fetch(this.url + "/models", {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                "Authorization": this.api_key,
            }
        })
        const res = await response.text()
        console.log(res)
        if (response.status == 200) {
            const js = JSON.parse(res)
            if (js.data && js.data.length > 0) {
                const models = []
                for (const model of js.data) {
                    models.push(model.id)
                }
                return Promise.resolve(models)
            }
        }
        return Promise.reject([])
    }

    async generate(param: ModelInputParam): Promise<ModelResponse> {
        const messages = param.messages
        if (messages.length <= 0) {
            messages.push(
                {"role": "system", "content": `${param.system || ""}`},
                {"role": "user", "content": `${param.content}`}
            )
        }
        const response = await fetch(this.url + "/chat/completions", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "Authorization": this.api_key,
            },
            body: JSON.stringify({
                "model": this.modelName,
                "messages": messages,
                "stream": false,
                "temperature": param.temperature || this.temperature
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