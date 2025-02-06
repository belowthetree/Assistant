const template = {
    "type": "function",
    "function": {
        "name": "get_current_weather",
        "description": "Get the current weather for a location",
        "parameters": {
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The location to get the weather for, e.g. San Francisco, CA"
                },
                "format": {
                    "type": "string",
                    "description": "The format to return the weather in, e.g. 'celsius' or 'fahrenheit'",
                    "enum": ["celsius", "fahrenheit"]
                }
            },
            "required": ["location", "format"]
        }
    }
}

export const execCmd = {
    "name": "execCmd",
    "Desc":
    {
        "ability": "执行控制台命令",
        "parameters": {
            "type": "object",
            "properties": {
                "arg": {
                    "type": "string",
                    "description": "The command to execute, e.g. 'ls -la'"
                }
            },
            "required": ["arg"]
        }
    }
}


export const openApp = {
    "name": "open_app_by_shortcut",
    "Desc":
    {
        "ability": "通过名字打开应用",
        "parameters": {
            "type": "object",
            "properties": {
                "arg": {
                    "type": "string",
                    "description": "The name of the application to open"
                }
            },
            "required": ["arg"]
        }
    }
}

export const getAllAppNames = {
    "name": "get_all_app_names",
    "Desc":
    {
        "ability": "获取所有应用名字",
        "parameters": {
            "type": "object",
            "properties": {},
            "required": []
        }
    }
}

export const askWindow = 
{
    "name": "ask_window",
    "Desc":
    {
        "ability": "打开一个提问窗口通知用户",
        "parameters": {
            "type": "object",
            "properties": {
                "msg": {
                    "type": "string",
                    "description": "The message to display in the window"
                },
                "title": {
                    "type": "string",
                    "description": "The title of the window"
                }
            },
            "required": ["app", "msg", "title"]
        }
    }
}

export const Instructions =
[
    execCmd,
    openApp,
    getAllAppNames,
    askWindow,
]