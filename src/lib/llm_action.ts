import { invoke } from "@tauri-apps/api/core";
import { ask } from "@tauri-apps/plugin-dialog";
import { writeText } from "@tauri-apps/plugin-clipboard-manager"
import { isPermissionGranted, requestPermission, sendNotification } from "@tauri-apps/plugin-notification";
import { BaseDirectory, writeTextFile } from "@tauri-apps/plugin-fs";

// 执行命令行
export async function execCmd(command: string):Promise<number> {
    return await invoke("exec_cmd", {arg: command}) as number
}

//通过应用的名字打开应用
export async function openAppByShortcut(name: string):Promise<number> {
    return await invoke("open_app_by_shortcut", {name: name}) as number
}

//获取所有应用的名字
export async function getAllAppNames():Promise<string[]> {
    let res: string[] = []
    const arr = await invoke("get_all_app_names") as string[]
    for (const s in arr) {
        res.push(s)
    }
    return res
}
//打开一个标题为 title，内容为 msg 的问话弹窗
export async function openAskDialog(title: string, msg: string) {
    ask(msg, title).then(e=>console.log(e))
}
//将 text 写入剪贴板
export async function writeToClipboard(text:string) {
    writeText(text)
}
//向用户发送一个标题为 title，内容为 msg 的通知
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
//将content写入文件filename.txt中
export async function writeToFile(filename:string, content:string) {
    writeTextFile(`${filename}.txt`, content, {baseDir: BaseDirectory.Home})
}
//进行网页搜索，输入的 question 只能是英文
export async function searchWeb(question:string) {
    invoke("search_duck_duck_go", {question: question}).then(e=>console.log(e))
}

//在项目临时目录写入文本文件
export async function writeTextFileAtProjectRoot(path:string, content:string) {
    if (path.indexOf("..") >= 0) {
        console.warn("不得传入相对上级的路径：", path)
        return
    }
    path = "temp/" + path
    await invoke("write_text_file_at_project_root", {path: path, content: content})
}