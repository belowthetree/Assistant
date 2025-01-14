import { BaseDirectory } from "@tauri-apps/api/path"
import { exists, mkdir, readTextFile, writeTextFile } from "@tauri-apps/plugin-fs"
import { EModelType } from "./data"
import { listenModelUpdateEvent } from "./events/model_event"
import { RoleCardBase } from "./rolecard/rolecardbase"

export class ModelConfig {
    name: string = ""
    baseUrl: string = "127.0.0.1:11434"
    modelType: EModelType = EModelType.Ollama
    apiKey: string = ""
    modelName: string = "qwen2.5-coder:3b"
    roleCard: RoleCardBase = new RoleCardBase()

    constructor(name: string) {
        this.name = name
    }
}

export class ModelListConfig {
    currentConfig: string
    Models: {[key: string]: ModelConfig} = {}

    constructor() {
        const model = new ModelConfig("默认配置")
        this.currentConfig = model.name
        this.Models[model.name]
    }

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
        this.currentConfig = model.name
        this.Models[model.name] = model
        return model
    }

    getModelConfigByName(name: string): ModelConfig {
        for (const key in this.Models) {
            if (this.Models[key].name === name)
                return this.Models[key]
        }
        return undefined
    }

    addModel(key: string, model: ModelConfig):boolean {
        if (key.length <= 0)
            return false
        if (!this.Models[key]){
            this.Models[key] = model
            return true
        }
        return false
    }

    saveModel(key: string, model: ModelConfig) {
        this.Models[key] = model
    }

    validate() {
        const names = Object.keys(this.Models)
        for (const name of names) {
            if (this.Models[name].name !== name){
                const model = this.Models[name]
                delete this.Models[name]
                this.Models[model.name] = model
            }
        }
    }
}

export var RoleCards: RoleCardBase[] = []
export var ModelList: ModelListConfig = new ModelListConfig()

listenModelUpdateEvent(()=>{
    // loadConfig()
})

export async function loadConfig() {
    await loadBaseConfig()
    await loadRoleCards()
}

export async function saveConfig() {
    // 保存模型列表
    const js = JSON.stringify(ModelList)
    const exis = await exists("", {baseDir: BaseDirectory.AppConfig})
    if (!exis) {
        console.log("创建配置文件夹")
        await mkdir("", {baseDir: BaseDirectory.AppConfig, recursive: true})
    }
    await writeTextFile("config.json", js, {baseDir: BaseDirectory.AppConfig})
    console.log(ModelList)
    // 保存角色卡
    const rolecardJs = JSON.stringify(RoleCards)
    await writeTextFile("rolecards.json", rolecardJs, {baseDir: BaseDirectory.AppConfig})
}

// 读取模型列表配置
function loadBaseConfig() {
    readTextFile("config.json", {baseDir: BaseDirectory.AppConfig}).then((cfg)=>{
        ModelList = JSON.parse(cfg)
        const t = new ModelListConfig()
        for (const key in ModelList) {
            t[key] = ModelList[key]
        }
        ModelList = t
        if (!ModelList.getCurrentModelConfig){
            console.log("没有配置，默认创建")
            ModelList = new ModelListConfig()
        }
        ModelList.getCurrentModelConfig()
        console.log(ModelList)
    }).catch((e)=>{
        ModelList = new ModelListConfig()
        console.warn(e)
    })
}

// 读取角色卡
function loadRoleCards() {
    readTextFile("rolecards.json", {baseDir: BaseDirectory.AppConfig}).then((cfg)=>{
        RoleCards = JSON.parse(cfg) as RoleCardBase[]
        console.log(RoleCards)
    }).catch((e)=>{
        // 加载基础角色卡
        RoleCards = []
        let card = new RoleCardBase()
        card.systemPrompt = 
        RoleCards.push(card)
        console.warn(e)
    })
}
