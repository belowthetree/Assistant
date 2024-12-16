export const SystemPrompt: string = JSON.stringify({
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