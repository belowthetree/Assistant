import { DeepSeek } from './deepseek';

export class OpenAIModel extends DeepSeek {

    constructor(url: string, modelName: string, roleCard: string, api_key: string) {
        super(url, modelName, roleCard, api_key)
    }

    async checkApiKeyValid(): Promise<boolean>  {
        return (await this.getModels()).length > 0
    }
}