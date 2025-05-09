import { invoke } from "@tauri-apps/api/core";
import { Server, Servers, Tool } from "../data/mcp";

export async function getServers(): Promise<Server[]> {
    return [...Servers.values()]
}

export async function getTools(serverName: string): Promise<Tool[]> {
    const server = Servers.get(serverName)
    if (server) {
        if (!server.updated) {
            const tools = await invoke("get_tools", {name: serverName}) as Tool[]
            return Promise.resolve(tools)
        }
    }
    return Promise.reject([])
}