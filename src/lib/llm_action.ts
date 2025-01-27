import { invoke } from "@tauri-apps/api/core";
import { ask } from "@tauri-apps/plugin-dialog";
import { writeText } from "@tauri-apps/plugin-clipboard-manager"
import { isPermissionGranted, requestPermission, sendNotification } from "@tauri-apps/plugin-notification";
import { BaseDirectory, writeTextFile } from "@tauri-apps/plugin-fs";
import { addBubble } from "../view/Talk/api";

// 执行命令行
export async function execCmd(command: string):Promise<number> {
    return await invoke("exec_cmd", {command: command}) as number
}

/**打开系统中已经注册的快捷方式应用
 * @param name 快捷方式名字
 * @return 错误码，如果为 0 代表打开正常
*/
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
//与用户对话，内容为 content，与用户交互优先使用这个函数
export function chat(content: string) {
    addBubble(content)
}
//发送一个标题为 title，内容为 msg 的系统级通知，此函数仅作系统级别通知
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
/**
 * 在临时目录写入相对路径为 path 的文本文件
 * @param path 临时目录下的相对路径
 * @param content 写入的文本内容
 * @returns 结果
 */
export async function writeTextFileAtProjectTemp(path:string, content:string):Promise<string> {
    if (path.indexOf("..") >= 0) {
        console.warn("不得传入相对上级的路径：", path)
        return
    }
    path = "temp/" + path
    await invoke("write_text_file_at_project_root", {path: path, content: content})
}
/**
 * 读取临时目录下的文本文件
 * @param path 临时目录下的相对路径
 * @returns 文本文件内容
 */
export async function readTextFileAtProjectTemp(path:string): Promise<string> {
    if (path.indexOf("..") >= 0) {
        console.warn("不得传入相对上级的路径：", path)
        return
    }
    path = "temp/" + path
    return await invoke("read_text_file_at_project_root", {path: path})
}
//获取临时目录绝对路径
export async function getProjectTempPath():Promise<string> {
    const path = await invoke("get_project_root_path") + "/temp/"
    console.log("绝对路径：", path)
    return path
}
//用 AI 进行模糊判断
export async function fuzzyMatch(list:string[], target: string) {
    
}
