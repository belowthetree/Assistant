import { emit, EventCallback, listen } from "@tauri-apps/api/event";
import { ask } from "@tauri-apps/plugin-dialog";

const TalkViewQuery = "TalkViewQuery"

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

export async function listenTalkViewQueryEvent(callback:EventCallback<any>) {
    await listen(TalkViewQuery, callback)
}

export function emitTalkViewQueryEvent() {
    emit(TalkViewQuery)
}