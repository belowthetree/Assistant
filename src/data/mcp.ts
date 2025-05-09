
export interface Server {
    // 是否更新，未更新的需要从后端拉取
    updated: boolean
    // 是否内部服务
    isInternal: boolean
    tools: Tool[]
    resources: Resource[]
}

export interface Tool {
    name: string
    description: string
    input_schema?: {
        type: string,
        properties: any
    }
}

export interface Resource {
    uri: string
    mimeType: string
    // 对于文本资源
    text?: string
    // 对于二进制资源（base64 编码）
    blob?: string
}

export var Servers: Map<string, Server> = new Map()