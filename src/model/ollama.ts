import { ModelBase, ModelInputParam, ModelResponse } from "~/src/model/modelbase";
import { EModelType } from "../data/model";

export class Ollama extends ModelBase {
    constructor(url: string, model: string, roleCard: string, api_key: string = "") {
        if (url.endsWith("/")) {
            url = url.slice(0, url.length)
        }
        super(url, model, roleCard, api_key)
        this.modelType = EModelType.Ollama
        console.log(this.modelName)
    }

    get_chat_url(): string {
        return this.url + "/api/chat"
    }

    get_generate_url(): string {
        return this.url + "/api/generate"
    }

    async generate(param: ModelInputParam): Promise<ModelResponse> {
        this.url = this.url.replace("chat", "generate")
        return super.generate(param)
    }

    async getModels():Promise<string[]> {
        const response = await fetch(this.url + "/api/tags")
        console.log(response)
        if (response.status == 200) {
            const txt = await response.text()
            const js = JSON.parse(txt)
            const models: string[] = []
            for (let m of js.models) {
                models.push(m.name)
            }
            return Promise.resolve(models)
        }
        else {
            console.warn("ollama api/tags 请求失败")
            return Promise.reject([])
        }
    }
}