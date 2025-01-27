import { emitModelResultEvent } from "~/src/events/model_event";
import { listenTalkViewQueryEvent } from "~/src/events/window_event";
import { openTalkView } from "~/src/lib/window";

var reply: string = ""
listenTalkViewQueryEvent(()=>{
    addBubble(reply)
})

export async function addBubble(content:string) {
    reply = content
    openTalkView().then(()=>{
        emitModelResultEvent(content)
    })
}