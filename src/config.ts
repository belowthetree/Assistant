import { appConfigDir, BaseDirectory } from "@tauri-apps/api/path"
import { exists, mkdir, readTextFile, writeTextFile } from "@tauri-apps/plugin-fs"
import { listenModelUpdateEvent } from "./events/model_event"
import { RoleCardBase } from "./rolecard/rolecardbase"
import { ModulePrompt } from "./prompt/module_prompt"
import { EModelType } from "./data/model"

export class ModelConfig {
    name: string = ""
    baseUrl: string = "http://127.0.0.1:11434"
    modelType: EModelType = EModelType.Ollama
    apiKey: string = ""
    modelName: string = "qwen2.5-coder:7b"
    temperature: number = 0.6
    stream: boolean = false
    roleCard: string

    constructor(name: string) {
        this.name = name
        this.roleCard = "电脑小助手"
    }

    fromjson(json: any) {
        this.name = json.name
        this.baseUrl = json.baseUrl
        this.modelType = json.modelType as EModelType
        this.apiKey = json.apiKey
        this.modelName = json.modelName
        this.roleCard = json.roleCard
    }
}

export class ModelListConfig {
    currentConfig: string
    Models: {[key: string]: ModelConfig} = {}

    constructor() {
        const model = new ModelConfig("默认配置")
        this.currentConfig = model.name
        this.Models[model.name] = model
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

export async function loadConfig():Promise<void> {
    await loadBaseConfig()
    await loadRoleCards()
    return Promise.resolve()
}

export function getRoleCard(name: string): RoleCardBase {
    for (const role of RoleCards) {
        if (role.name === name) {
            return role
        }
    }
    return RoleCards[0]
}

export async function saveConfig():Promise<void> {
    // 保存模型列表
    const js = JSON.stringify(ModelList, undefined, "\t")
    const exis = await exists("", {baseDir: BaseDirectory.AppConfig})
    if (!exis) {
        console.log("创建配置文件夹")
        await mkdir("", {baseDir: BaseDirectory.AppConfig, recursive: true})
    }
    await writeTextFile("config.json", js, {baseDir: BaseDirectory.AppConfig})
    // 保存角色卡
    const rolecardJs = JSON.stringify(RoleCards, undefined, "\t")
    await writeTextFile("rolecards.json", rolecardJs, {baseDir: BaseDirectory.AppConfig})
}

// 读取模型列表配置
async function loadBaseConfig():Promise<void> {
    try {
        const cfg = await readTextFile("config.json", {baseDir: BaseDirectory.AppConfig})
        const configs = JSON.parse(cfg)
        const t = new ModelListConfig()
        for (const key in configs.Models) {
            const model = new ModelConfig(key)
            model.fromjson(configs.Models[key])
            t.saveModel(key, model)
        }
        ModelList = t
        ModelList.currentConfig = configs.currentConfig
        if (!ModelList.getCurrentModelConfig){
            console.log("没有配置，默认创建")
            ModelList = new ModelListConfig()
        }
        ModelList.getCurrentModelConfig()
        return Promise.resolve()
    }
    catch(e) {
        ModelList = new ModelListConfig()
        console.warn(e)
    }
}

// 读取角色卡
async function loadRoleCards():Promise<void> {
    try {
        const cfg = await readTextFile("rolecards.json", {baseDir: BaseDirectory.AppConfig})
        RoleCards = JSON.parse(cfg) as RoleCardBase[]
    }
    catch(e) {
        // 加载基础角色卡
        RoleCards = []
        console.warn(e)
    }
    RoleCards = RoleCards.filter((v)=>{return v})
    if (RoleCards.length <= 0) {
        let card = new RoleCardBase()
        card.name = "电脑小助手"
        card.systemPrompt = ModulePrompt
        card.roleDesc = "一个帮助你使用电脑的小助手，你可以跟他对话，或者让他帮助你在电脑上做点事情"
        RoleCards.push(card)
    }
}
