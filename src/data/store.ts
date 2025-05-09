import { BaseDirectory, exists, mkdir, readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
import { Servers } from "./mcp";
import { appConfigDir } from "@tauri-apps/api/path";

const ServerConfigFile = "server.json"

export async function saveServers() {
    const exis = await exists("", {baseDir: BaseDirectory.AppConfig})
    if (!exis) {
        console.log("创建配置文件夹: ", appConfigDir())
        await mkdir("", {baseDir: BaseDirectory.AppConfig, recursive: true})
    }
    const obj = Object.fromEntries(Servers.entries())
    const ctx = JSON.stringify(obj)
    await writeTextFile(ServerConfigFile, ctx, {baseDir: BaseDirectory.AppConfig})
}

export async function loadServers() {
    const ctx = await readTextFile(ServerConfigFile, {baseDir: BaseDirectory.AppConfig})
    const obj = JSON.parse(ctx)
    Servers.clear()
    for (const key in obj) {
        if (obj.hasOwnProperty(key)) {
            Servers.set(key, obj[key])
        }
    }
}