
export enum EModelType {
    Ollama = "Ollama",
    Deepseek = "Deepseek",
    OpenAI = "OpenAI",
}

export interface ModelMessage {
    role: string
    content: string
}

export class ModelData {
    model_type: EModelType
    api_key: String
    messages: ModelMessage[]
    url: String
    model_name: String
    temperature: number

    constructor() {
        this.model_type = EModelType.Deepseek
        this.api_key = ""
        this.messages = []
        this.url = "https://api.deepseek.com/v1"
        this.model_name = ""
        this.temperature = 0.6
    }
}