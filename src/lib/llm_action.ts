import { invoke } from "@tauri-apps/api/core";

export async function execCmd(command: string):Promise<number>  {
    return await invoke("exec_cmd", {arg: command}) as number
}

export async function openAppByShortcut(name: string):Promise<number> {
    return await invoke("open_app_by_shortcut", {name: name}) as number
}

export async function getAllAppNames():Promise<string[]> {
    let res: string[] = []
    const arr = await invoke("get_all_app_names") as string[]
    for (const s in arr) {
        res.push(s)
    }
    return res
}