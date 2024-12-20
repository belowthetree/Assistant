import { execCmd, getAllAppNames, openApp } from "./function_prompt"

export const SystemPrompt: string = `
{
    "instructions": [
        "获取所有应用名字",
        "读取文件",
        "读取目录所有文件路径",
        "打开应用",
        "打开文件",
    ]
}
这是命令列表，instructions 内包含了你可以执行的命令。
接下来你需要根据提问选取合适的命令并按照合理的顺序输出。如果没有你想要的命令，你可以自己编造一个命令，但应该以 instructions 中的命令优先。
`

export const Instructions =
`{
    "instructions": [
        "${getAllAppNames.Desc.name}",
        "${execCmd.Desc.name}",
        "${openApp.Desc.name}"
    ]
}`

export const FunctionNameSystemPrompt: string =
`${Instructions}
这是命令列表，instructions 内包含了你可以执行的命令，接下来你需要根据要求选取或编造合适的命令并按照合理的顺序输出，如果没有你想要的命令或者当前命令不足以完成要求，你可以自己编造一个命令，但应该以 instructions 中的命令优先，你需要确保按照命令执行后可以实现提问所需的要求。
比如用户要求“随机打开一个英文应用”，你应该输出：
{
    "instructions": [
        "获取所有应用名字",
        "随机选取一个英文应用",
        "通过名字打开应用"
    ]
}
其中"通过名字打开应用"、"获取所有应用名字" 都是已有的命令可以直接使用，"随机选取一个英文应用" 是新增编造的，因为原始命令没有提供随机选取英文应用的命令，而这个命令可以通过 AI 完成。
如果实在无法通过已有命令或者编造 AI 可以执行命令的方式完成用户要求，你就输出“无法办到”。
`

JSON.stringify({
    "function": [
        {
            "name": "exec_cmd",
            "desc": "执行命令行命令，确保你输入的 cmd 的可以在命令行中执行",
            "parameter": [
                {
                    "name": "cmd",
                    "type": "string",
                }
            ]
        }
    ]
}) + `这是操作函数表，name 字段代表函数名字，desc 字段代表函数作用，parameter 代表函数接收的参数，type 表示参数的类型，name 表示参数名字。接下来你的回答需要从这个函数表中选择合适的函数以 json 格式回答。
你需要回答准确的信息，不要携带任何变量。函数名字不可更改。确保你的输入、使用符合 desc 字段的描述`

const t = {
    "name": {
        "type": "string"
    },
    "parameter": {
        "type": "array",
        "items": {
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                },
                "type": {
                    "type": "string"
                }
            }
        }
    },
    "return": {
        "type": "string"
    }
}