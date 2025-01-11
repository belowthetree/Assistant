import { appConfigDir, BaseDirectory } from "@tauri-apps/api/path"
import { exists, mkdir, readTextFile, writeTextFile } from "@tauri-apps/plugin-fs"
import { LLMBase } from "./model/llm_base"
import { Ollama } from "./model/ollama"
import { EModelType } from "./data"
import { DeepSeek } from "./model/deepseek"
import { listenModelUpdateEvent } from "./events/model_event"

export interface IBaseConfig {
    baseUrl: string
    modelType: EModelType
    apiKey: string
    modelName: string
}

class BaseConfigBase implements IBaseConfig {
    baseUrl: string = "127.0.0.1:11434"
    modelType: EModelType = EModelType.Ollama
    apiKey: string = ""
    modelName: string = "qwen2.5-coder:7b"
}

export var BaseConfig: BaseConfigBase = new BaseConfigBase()
export var MainModel: LLMBase = new Ollama("http://127.0.0.1:11434/api/generate", "qwen2.5-coder:7b")

listenModelUpdateEvent((event)=>{
    console.log(event.payload)
    BaseConfig = event.payload.config
    saveConfig()
})

export async function loadConfig() {
    const dir = await appConfigDir()
    console.log("load config" + dir)
    readTextFile("config.json", {baseDir: BaseDirectory.AppConfig}).then((cfg)=>{
        BaseConfig = JSON.parse(cfg) as BaseConfigBase
        console.log(BaseConfig)
        refreshModel()
    }).catch((e)=>{
        BaseConfig = new BaseConfigBase()
        console.warn(e)
        refreshModel()
    })
}

export async function saveConfig() {
    if (MainModel.modelType != BaseConfig.modelType) {
        refreshModel()
    }
    else {
        MainModel.url = BaseConfig.baseUrl
        MainModel.modelName = BaseConfig.modelName
    }
    const js = JSON.stringify(BaseConfig)
    console.log(js)
    const exis = await exists("", {baseDir: BaseDirectory.AppConfig})
    if (!exis) {
        console.log("创建配置文件夹")
        await mkdir("", {baseDir: BaseDirectory.AppConfig, recursive: true})
    }
    writeTextFile("config.json", js, {baseDir: BaseDirectory.AppConfig})
}

function refreshModel() {
    console.log('refresh')
    switch(BaseConfig.modelType) {
        case EModelType.Ollama:
            MainModel = new Ollama(BaseConfig.baseUrl, BaseConfig.modelName, BaseConfig.apiKey)
            break
        case EModelType.DeepSeek:
            MainModel = new DeepSeek(BaseConfig.baseUrl, BaseConfig.modelName, BaseConfig.apiKey)
            break
    }
    console.log(MainModel)
}