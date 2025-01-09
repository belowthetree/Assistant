import { appConfigDir, BaseDirectory } from "@tauri-apps/api/path"
import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs"

export interface IBaseConfig {
    baseUrl: string
    modelType: string
    apiKey: string
}

export enum EModelType {
    Ollama,
    OpenAI,
}
export enum ECmdMode {
    CommandSequence = "CommandSequence",
    TypescriptCode = "TypescriptCode",
    Exec = "Exec",
}

export var BaseConfig: IBaseConfig

export function SetApiKey(key: string) {
    BaseConfig.apiKey = key
}

export function SetModelType(modelType: string) {
    BaseConfig.modelType = modelType
}

export async function loadConfig() {
    const dir = await appConfigDir()
    console.log("load config" + dir)
    readTextFile("config.json", {baseDir: BaseDirectory.AppConfig}).then((cfg)=>{
        BaseConfig = JSON.parse(cfg)
        console.log(BaseConfig)
    }).catch((e)=>{
        console.warn(e)
    })
}

export async function saveConfig() {
    const js = JSON.stringify(BaseConfig)
    console.log(js)
    writeTextFile("config.json", js, {baseDir: BaseDirectory.AppConfig})
}