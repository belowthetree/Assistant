import { ModelMessage } from "./model/modelbase"

export enum EModelType {
    Ollama = "Ollama",
    DeepSeek = "DeepSeek",
    OpenAI = "OpenAI",
}

export enum ECmdMode {
    CommandSequence = "CommandSequence",
    TypescriptCode = "TypescriptCode",
    Exec = "Exec",
}

export enum EChatMode {
    Chat,
    Generate,
}

export interface ModelData {
    model_type: EModelType
    api_key: String
    messages: ModelMessage[]
    url: String,
    model_name: String,
    temperature: number,
}