import { SystemPrompt } from "../prompt/system_prompt";
import { fetch} from "@tauri-apps/plugin-http"

export async function inputCommand(model: string, desc: string) {
    console.log(model, desc)
    let v = await fetch("http://127.0.0.1:11434/api/generate", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            "model": model,
            "prompt": desc,
            "system": SystemPrompt,
            "stream": false,
            "options": {
                "temperature": 2,
            },
            "format": {
                "type": "object",
                "properties": {
                    "function":{
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "name": {
                                    "type": "string",
                                },
                                "parameter": {
                                    "type": "array",
                                    "items": {
                                        "type": "object",
                                        "properties": {
                                            "name": {
                                                "type": "string",
                                            },
                                            "value": {
                                                "type": "string",
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }),
    })
    console.log(v)
    if (v.status == 200){
        return JSON.parse(await v.text()).response
    }
    else
        return "";
}