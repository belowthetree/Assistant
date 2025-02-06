export class RoleCardBase {
    // 名字
    name: string = "默认"
    // 身份介绍
    roleDesc: string = ""
    // 是否保留上下文
    saveContext: boolean = true
    // 上下文长度
    contextLength: number = 1024
    // 是否启用人工提示链
    useManualPromptChain: boolean = false
    // 人工提示链
    promptChain: string[] = []
    // 系统提示词
    systemPrompt: string = ""

    getPrompt(index:number):string | null {
        if (!this.useManualPromptChain)
            return null
        return this.promptChain[index]
    }
}