import { ModelConfig } from "../config";
import { EModelType } from "../data";
import { DeepSeek } from "./deepseek";
import { LLMBase } from "./llm_base";
import { Ollama } from "./ollama";

export var MainModel: LLMBase = new Ollama("http://127.0.0.1:11434/api/generate", "qwen2.5-coder:3b")

export function generateModelFromConfig(config: ModelConfig): LLMBase {
    let model
    switch(config.modelType) {
        case EModelType.OpenAI:
        case EModelType.DeepSeek:
            model = new DeepSeek(config.baseUrl, config.modelName, config.apiKey)
            break
        case EModelType.Ollama:
            model = new Ollama(config.baseUrl, config.modelName, config.apiKey)
            break
    }
    return model
}