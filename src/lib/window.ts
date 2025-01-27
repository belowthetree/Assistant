
import { WebviewOptions } from '@tauri-apps/api/webview';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { WindowOptions } from '@tauri-apps/api/window';

export async function createWindow(
    label:string,
    WindowOption?:Omit<WebviewOptions, 'x' | 'y' | 'width' | 'height'> & WindowOptions,
    fucus: boolean = true,
):Promise<WebviewWindow> {
    let win = await WebviewWindow.getByLabel(label)
    if (win) {
        console.log(`已存在 ${label}`)
        if (!win.isVisible())
            win.show()
        if (fucus)
            win.setFocus()
        return win
    }
    console.log("创建" + `${label}`)
    WindowOption.url = `/${label}`,
    win = new WebviewWindow(label, WindowOption)

    // let win = await Window.getByLabel(label)
    // if (!win) {
    //     win = new Window(label)
    //     console.log("创建")
    // }
    // else
    //     win.show()

    return new Promise((resolve, reject)=>{
        // 监听窗口创建成功事件
        win.once('tauri://created', () => {
            console.log('创建成功');
            resolve(win)
        });

        // 监听窗口创建失败事件
        win.once('tauri://error', (e) => {
            console.error('创建失败', e);
            reject(win)
        });
    })
}

export async function openSettingWindow():Promise<WebviewWindow> {
    // const monitor = await window.currentMonitor()
    // const size = monitor.size
    // win = new WebviewWindow(label, {
    //     title,
    //     url: `/${label}`,
    //     width,
    //     height,
    //     resizable,
    //     decorations: false,
    //     center,
    //     visible: true,
    //     x: size.width - width + offset.width,
    //     y: (size.height - height) / 2 + offset.height,
    return createWindow("setting", {title: "设置", width: 500, height: 600, resizable: false, decorations: false})
}

export async function openTalkView():Promise<WebviewWindow> {
    return createWindow('talk', {title: "对话", width: 500, height: 500, resizable: false, decorations: false, transparent: true, shadow: false, visible: true}, false)
}

export async function openRoleCardView() {
    return createWindow('rolecard', {title: "角色卡", width: 500, height: 600, resizable: false, decorations: false, minHeight: 450})
}