import { emitModelResultEvent } from "~/src/events/model_event";
import { openTalkView } from "~/src/lib/window";

export async function addBubble(content:string) {
    openTalkView().then(()=>{
        console.log("Add bubble")
        emitModelResultEvent(content)
    })
}