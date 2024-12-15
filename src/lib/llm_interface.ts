import axios from "axios";
import { SystemPrompt } from "../prompt/system_prompt";

export async function inputCommand(model: string, desc: string) {
    console.log(desc)
    let v = await axios.post("http://127.0.0.1:11434/api/generate", {
        "model": model,
        "prompt": desc,
        "system": SystemPrompt,
        "stream": false,
    })
    console.log(v)
    if (v.status == 200)
        return v.data.response
    else
        return "";
}