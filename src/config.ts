import { BaseDirectory } from "@tauri-apps/api/path"
import { exists, mkdir, readTextFile, writeTextFile } from "@tauri-apps/plugin-fs"
import { LLMBase } from "./model/llm_base"
import { Ollama } from "./model/ollama"
import { EModelType } from "./data"
import { DeepSeek } from "./model/deepseek"
import { listenModelUpdateEvent } from "./events/model_event"
import { RoleCardBase } from "./rolecard/rolecardbase"

class ModelConfig {
    name: string = ""
    baseUrl: string = "127.0.0.1:11434"
    modelType: EModelType = EModelType.Ollama
    apiKey: string = ""
    modelName: string = "qwen2.5-coder:7b"
    roleCards: RoleCardBase = new RoleCardBase()

    constructor(name: string) {
        this.name = name
    }
}

export class ModelListConfig {
    currentConfig: string
    Models: {[key: string]: ModelConfig} = {}

    getCurrentModelConfig():ModelConfig {
        if (this.Models[this.currentConfig])
            return this.Models[this.currentConfig]
        const keys = Object.keys(this.Models)
        if (keys.length > 0)
        {
            this.currentConfig = this.Models[keys[0]].name
            return this.Models[keys[0]]
        }
        const model = new ModelConfig("默认配置")
        this.Models[model.name]
        return model
    }
}

export var RoleCards: RoleCardBase[] = []
export var CurrentConfig: ModelConfig = null
var ModelList: ModelListConfig = new ModelListConfig()
export var MainModel: LLMBase = new Ollama("http://127.0.0.1:11434/api/generate", "qwen2.5-coder:7b")

listenModelUpdateEvent(()=>{
    loadConfig()
})

export async function loadConfig() {
    loadBaseConfig()
    loadRoleCards()
}

export async function saveConfig() {
    if (MainModel.modelType != CurrentConfig.modelType) {
        refreshModel()
    }
    else {
        MainModel.url = CurrentConfig.baseUrl
        MainModel.modelName = CurrentConfig.modelName
    }
    const js = JSON.stringify(ModelConfig)
    console.log(ModelConfig)
    const exis = await exists("", {baseDir: BaseDirectory.AppConfig})
    if (!exis) {
        console.log("创建配置文件夹")
        await mkdir("", {baseDir: BaseDirectory.AppConfig, recursive: true})
    }
    writeTextFile("config.json", js, {baseDir: BaseDirectory.AppConfig})
    const rolecardJs = JSON.stringify(RoleCards)
    writeTextFile("rolecards.json", rolecardJs, {baseDir: BaseDirectory.AppConfig})
    console.log(ModelConfig)
}

// 读取当前使用的基础配置
function loadBaseConfig() {
    readTextFile("config.json", {baseDir: BaseDirectory.AppConfig}).then((cfg)=>{
        ModelConfig = JSON.parse(cfg) as ModelConfig
        console.log(ModelConfig)
        refreshModel()
    }).catch((e)=>{
        ModelConfig = new ModelConfig()
        console.warn(e)
        refreshModel()
    })
}

// 读取角色卡
function loadRoleCards() {
    const func = ()=>{
        
    }
    readTextFile("rolecards.json", {baseDir: BaseDirectory.AppConfig}).then((cfg)=>{
        RoleCards = JSON.parse(cfg) as RoleCardBase[]
        console.log(RoleCards)
    }).catch((e)=>{
        // 加载基础角色卡
        RoleCards = []
        console.warn(e)
    })
}

function refreshModel() {
    console.log('refresh')
    switch(ModelConfig.modelType) {
        case EModelType.Ollama:
            MainModel = new Ollama(ModelConfig.baseUrl, ModelConfig.modelName, ModelConfig.apiKey)
            break
        case EModelType.DeepSeek:
            MainModel = new DeepSeek(ModelConfig.baseUrl, ModelConfig.modelName, ModelConfig.apiKey)
            break
    }
    console.log(MainModel)
}