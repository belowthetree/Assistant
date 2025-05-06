import { MCPServerConfig, MCPServer } from "./MCPServer";

export interface MCPClientConfig {
    name?: string;
    version?: string;
}

export class MCPClient {
    private servers: Record<string, MCPServer> = {}

    constructor() {
    }

    // 方法将在这里实现
    addServers(configs: MCPServerConfig[]) {
        for (const cfg of configs) {
            const server = new MCPServer(cfg)
            server.connect()
            if (server.isConnected()) {
                this.servers[cfg.name] = server
            }
        }
    }
}