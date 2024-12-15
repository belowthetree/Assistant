import { invoke } from "@tauri-apps/api/core";

export function openApp(name: string) {
    invoke("openApp", {name : name}).then((e)=>{
        console.log(e)
    })
}