import { invoke } from "@tauri-apps/api/core"

const content = await invoke("read_text_file_at_project_root", {path: "src/lib/llm_action.ts"}) as string
const ctxArr = content.split("\n")
let funcDesc = ""
for (let i = 0; i < ctxArr.length;++i) {
    const line = ctxArr[i]
    if (line.indexOf("export") >= 0) {
        // 如果存在注释，添加上
        for (let k = i - 1;k >= 0; ++k) {
            if (ctxArr[k].indexOf("//") >= 0) {
                funcDesc += ctxArr[k] + "\n"
            }
            else {
                break
            }
        }
        funcDesc += line.replace("export", "").replace("{", "").trim() + "\n"
    }
}

// console.log(funcDesc)

export const ModulePrompt =
`你是一个指令执行助手，用户提出要求后你将输出一段 javascript 代码，这段代码执行后将完成用户的要求。
下方是你可以直接使用的函数声明，你可以直接调用。请直接输出可以执行并完成用户要求的完整代码，代码将作为动态模块执行。
注意：请不要在声明参数或者变量的时候添加类型声明。比如 let a: number; 这样的声明是错误的。let a; 是对的。
如果需要告知用户信息，请用通知函数；如果你需要写入文件，请优先写入项目临时目录

// 可以直接使用的函数声明
${funcDesc}
`