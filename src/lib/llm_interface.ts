import { execCmd, getAllAppNames, openApp } from "../prompt/function_prompt";
import { FunctionNameSystemPrompt, SystemPrompt } from "../prompt/system_prompt";
import { fetch} from "@tauri-apps/plugin-http"

const FunctionFormat = {
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

const InstructionForamt = {
    "type": "object",
    "properties": {
        "instructions": {
            "type": "array",
            "items": {
                "type": "string"
            }
        }
    }
}

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
                "temperature": 0,
            },
            "format": {
                "type": "object",
                "properties": {
                    "instructions": {
                        "type": "array",
                        "items": {
                            "type": "string"
                            
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

export async function generate(model: string, ctx: string, stream?: boolean, system?: string, format?:string, temperature?:number) {
    console.log(model, ctx)
    let body;
    if (!format) {
        console.log("No format")
        body = {
            "model": model,
            "prompt": ctx,
            "stream": stream || false,
            "options": {
                "temperature": 0,
            },
            "system": system || "",
        }
    } else {
        body = {
            "model": model,
            "prompt": ctx,
            "stream": stream || false,
            "options": {
                "temperature": temperature || 0,
            },
            format : format,
            "system": system || "",
        }
    }
    let v = await fetch("http://127.0.0.1:11434/api/generate", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(body)
    })

    console.log(v)

    if (v.status == 200){
        let t = await v.text()
        return JSON.parse(t).response
    }
    let t = await v.text()
    console.log(t)
    return ""
}

export async function chat(model: string, ctx: string, stream?: boolean) {
    console.log(model, ctx)
    let v = await fetch("http://127.0.0.1:11434/api/chat", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            "model": model,
            "messages": [{
                "role": "user",
                "content": ctx,
            }],
            // "system": SystemPrompt,
            "stream": stream || false,
            "options": {
                "temperature": 0,
            },
        })
    })

    console.log(v)

    if (v.status == 200){
        return JSON.parse(await v.text()).message
    }
    else
        return "";
}

export async function generate_instructions(model:string, ctx: string, stream?: boolean):Promise<String> {
    console.log(model, ctx)
    console.log(FunctionNameSystemPrompt)
    let v = await fetch("http://127.0.0.1:11434/api/generate", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            "model": model,
            "prompt": ctx,
            "stream": stream || false,
            "options": {
                "temperature": 0,
            },
            "system": FunctionNameSystemPrompt,
            "format": InstructionForamt
        })
    })

    console.log(v)

    if (v.status == 200){
        let t = await v.text()
        return JSON.parse(t).response
    }
    return ""
}

export async function chat_with_tool(model:string, ctx: string, stream?: boolean):Promise<String> {
    console.log(model, ctx)
    let v = await fetch("http://127.0.0.1:11434/api/chat", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            "model": model,
            "messages": [{
                "role": "user",
                "content": ctx,
            }],
            "stream": stream || false,
            "options": {
                "temperature": 0,
            },
            "system": FunctionNameSystemPrompt,
            "format": InstructionForamt
        })
    })

    console.log(v)

    if (v.status == 200){
        let t = await v.text()
        console.log(t)
        return JSON.parse(t).response
    }
    return ""
}