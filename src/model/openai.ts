import { LLMBase } from './llm_base';

export class OpenAIModel extends LLMBase {

    constructor(url: string, modelName: string, roleCard: string, api_key: string) {
        super(url, modelName, roleCard, api_key)
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

    async checkApiKeyValid(): Promise<boolean>  {
        return (await this.getModels()).length > 0
    }
}