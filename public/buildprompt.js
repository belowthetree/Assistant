import { readFileSync, writeFileSync } from "fs"

const content = readFileSync("../src/lib/llm_action.ts").toString()
const ctxArr = content.split("\n")
let funcDesc = ""
for (let i = 0; i < ctxArr.length;++i) {
    const line = ctxArr[i]
    if (line.indexOf("export") >= 0) {
        // 如果存在注释，添加上
        let mark = ""
        for (let k = i - 1;k >= 0; --k) {
            const tempLine = ctxArr[k].trim()
            if (tempLine.indexOf("//") >= 0 || tempLine.indexOf("*") >= 0) {
                mark = ctxArr[k] + "\n" + mark
            }
            else {
                break
            }
        }
        funcDesc += mark + line.replace("export", "").replace("{", "").trim() + "\n"
    }
}

// console.log(funcDesc)

export const ModulePrompt =
`# 角色
你是一个指令执行助手，用户提出要求后你将输出一段 javascript 代码，这段代码将作为字符串被执行。你只会生成代码。

## 约束
-告知用户信息用通知函数
-写入文件先写入项目临时目录
-调用数组元素前先进行判空
-只生成完整 javascript 代码和注释
-小心使用\`、"、' 等字符
-去除自然语言

## 能力
下方是可以直接使用的函数声明，可以直接调用。
${funcDesc}

例子：
输入：
计算 2 的三次方
输出：
notify("2 的三次方", "2 的三次方是 8")
`

writeFileSync("defaultprompt.txt", ModulePrompt)