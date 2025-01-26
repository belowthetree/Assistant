
import { window } from '@tauri-apps/api';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { LogicalSize, Window } from '@tauri-apps/api/window';

export async function createWindow(
    label:string,
    title: string,
    width: number,
    height: number,
    center: boolean,
    resizable: boolean,
    offset: LogicalSize = new LogicalSize(0, 0),
    minWidth?: number,
    minHeight?: number,
):Promise<WebviewWindow> {
    let win = await WebviewWindow.getByLabel(label)
    if (win) {
        console.log(`已存在 ${label}`)
        if (!win.isVisible())
            win.show()
        win.setFocus()
        return win
    }
    console.log("创建" + `${label}`)
    const monitor = await window.currentMonitor()
    const size = monitor.size
    win = new WebviewWindow(label, {
        title,
        url: `/${label}`,
        width,
        height,
        resizable,
        decorations: false,
        center,
        visible: true,
        x: size.width - width + offset.width,
        y: (size.height - height) / 2 + offset.height,
        maximizable: false,
        minHeight: minHeight,
        minWidth: minWidth,
    })

    // let win = await Window.getByLabel(label)
    // if (!win) {
    //     win = new Window(label)
    //     console.log("创建")
    // }
    // else
    //     win.show()

    // 监听窗口创建成功事件
    win.once('tauri://created', () => {
        console.log('创建成功');
    });

    // 监听窗口创建失败事件
    win.once('tauri://error', (e) => {
        console.error('创建失败', e);
    });
    return win
}

export async function openSettingWindow():Promise<WebviewWindow> {
    return createWindow("setting","设置", 500, 600, false, false)
}

export async function openTalkView():Promise<WebviewWindow> {
    return createWindow('talk', "对话", 500, 500, false, false)
}

export async function openRoleCardView() {
    return createWindow('rolecard', "角色卡", 500, 600, false, true, undefined, undefined, 450)
}