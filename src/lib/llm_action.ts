import { invoke } from "@tauri-apps/api/core";
import { ask } from "@tauri-apps/plugin-dialog";
import { writeText } from "@tauri-apps/plugin-clipboard-manager"

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

export async function openAskDialog(title: string, msg: string) {
    ask(msg, title).then(e=>console.log(e))
}

export async function writeToClipboard(text:string) {
    writeText(text)
}