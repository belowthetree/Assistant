import { emit, EventCallback, listen } from "@tauri-apps/api/event";

const ModelUpdate = "ModelUpdate"
const ModelResult = "ModelResult"
const ModelConfigLoaded = "ModelConfigLoaded"

export async function emitModelUpdateEvent() {
    await emit(ModelUpdate)
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

export async function emitModelConfigLoadedEvent(ctx: string) {
    await emit(ModelConfigLoaded, {ctx: ctx})
}

export async function listenModelConfigLoadedEvent(callback:EventCallback<any>) {
    await listen(ModelConfigLoaded, callback)
}