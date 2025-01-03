import { invoke } from "@tauri-apps/api/core";
import { ask } from "@tauri-apps/plugin-dialog";
import { writeText } from "@tauri-apps/plugin-clipboard-manager"
import { isPermissionGranted, requestPermission, sendNotification } from "@tauri-apps/plugin-notification";
import { BaseDirectory, writeTextFile } from "@tauri-apps/plugin-fs";

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

export async function notify(title: string, msg: string) {
    let permissionGranted = await isPermissionGranted();

    // 如果没有，我们需要请求它
    if (!permissionGranted) {
    const permission = await requestPermission();
        permissionGranted = permission === 'granted';
    }

    // 一旦获得许可，我们就可以发送通知
    if (permissionGranted) {
        sendNotification({ title: title, body: msg });
    }
}

export async function writeToFile(content:string) {
    writeTextFile("temp.txt", content, {baseDir: BaseDirectory.Home})
}

export async function searchWeb(question:string) {
    invoke("search_duck_duck_go", {question: question}).then(e=>console.log(e))
}