
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { Window } from '@tauri-apps/api/window';

export async function createWindow(
    label:string,
    title: string,
    width: number,
    height: number,
    center: boolean,
):Promise<Window> {
    console.log("创建" + `/${label}`)
    const win = new WebviewWindow(label, {
        title,
        url: `/${label}`,
        width,
        height,
        resizable: true,
        decorations: false,
        center,
        visible: true
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