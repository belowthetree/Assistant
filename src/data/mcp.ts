export interface ServerConfig {
    name: string,
    command: string,
    args?: string,
    env?: Map<string, string>
    // 是否内部服务，默认 false
    isInternal?: boolean
    enable: boolean
}

export interface Server {
    // 是否连接，每次连接更新 tools、resources
    connect: boolean
    tools: Tool[]
    resources: Resource[]
    config: ServerConfig
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