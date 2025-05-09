import { ModelConfig } from "../config";
import { EModelType } from "../data";
import { ModelBase } from "./modelbase";
import { Ollama } from "./ollama";
import { OpenAIModel } from "./openai";

export var MainModel: ModelBase = new Ollama("http://127.0.0.1:11434/api/generate", "qwen2.5-coder:latest", "电脑小助手", "")

export function generateModelFromConfig(config: ModelConfig): ModelBase {
    let model
    switch(config.modelType) {
        case EModelType.OpenAI:
        case EModelType.DeepSeek:
            model = new OpenAIModel(config.baseUrl, config.modelName, config.roleCard, config.apiKey)
            break
        case EModelType.Ollama:
            model = new Ollama(config.baseUrl, config.modelName, config.roleCard, config.apiKey)
            break
    }
    return model
}