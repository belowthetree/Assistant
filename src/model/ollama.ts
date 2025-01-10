import { LLMBase } from "@/model/llm_base";
import { EModelType } from "@/data"

export class Ollama extends LLMBase {
    url="http://127.0.0.1:11434/api/generate"
    constructor(url: string, model: string, api_key: string = "") {
        super(url, model, api_key)
        this.modelType = EModelType.Ollama
    }
}