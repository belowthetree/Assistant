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
        // "ignore": "exec_cmd",
        "name": "执行控制台命令",
        "parameters": {
            "type": "object",
            "properties": {
                "cmd": {
                    "type": "string",
                    "description": "The command to execute, e.g. 'ls -la'"
                }
            },
            "required": ["cmd"]
        }
    }
}


export const openApp = {
    "name": "open_app_by_shortcut",
    "Desc":
    {
        // "ignore": "open_app",
        "name": "通过名字打开应用",
        "parameters": {
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "The name of the application to open"
                }
            },
            "required": ["name"]
        }
    }
}

export const getAllAppNames = {
    "name": "get_all_app_names",
    "Desc":
    {
        // "ignore": "get_all_apps_name",
        "name": "获取所有应用名字",
        "parameters": {
            "type": "object",
            "properties": {},
            "required": []
        }
    }
}

export const Instructions =
[
    execCmd,
    openApp,
    getAllAppNames,
]