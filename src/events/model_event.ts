import { emit, EventCallback, listen } from "@tauri-apps/api/event";
import { BaseConfig } from "../config";

const ModelUpdate = "ModelUpdate"
const ModelResult = "ModelResult"

export async function emitModelUpdateEvent() {
    await emit(ModelUpdate, {config : BaseConfig})
}

export async function listenModelUpdateEvent(callback: EventCallback<any>) {
    await listen(ModelUpdate, callback)
}

export async function emitModelResultEvent(ctx: string) {
    await emit(ModelResult, {ctx: ctx})
}

export async function listenModelResultEvent(callback:EventCallback<any>) {
    await listen(ModelResult, callback)
}