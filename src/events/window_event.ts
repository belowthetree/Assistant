import { listen } from "@tauri-apps/api/event";
import { ask } from "@tauri-apps/plugin-dialog";

export function registerWindowEvents() {
    listen<{
        msg: string,
        title: string,
    }>("ask_window", (e)=>{
        console.log("ask_window", e.payload)
        ask(e.payload.msg,e.payload.title).then((e)=>
        {
            console.log(e)
        })
    })
}