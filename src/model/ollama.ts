import { LLMBase } from "@/model/llm_base";
import { EModelType } from "@/data"

export class Ollama extends LLMBase {
    url="http://127.0.0.1:11434/api/generate"
    constructor(url: string, model: string, api_key: string = "") {
        super(url, model, api_key)
        this.modelType = EModelType.Ollama
        console.log(this.modelName)
    }

    async chat(content: string, temperature?: number, system?: string): Promise<string> {
        this.url = this.url.replace("generate", "chat")
        return super.chat(content, temperature, system)
    }

    async generate(content: string, temperature?: number, system?: string): Promise<string> {
        this.url = this.url.replace("chat", "generate")
        return super.generate(content, temperature, system)
    }
}