# Assistant
一款 AI 助手前端，通过提示词工程+接口的形式配合指定地址的大模型服务提供智能助手的服务
> 默认使用 ollama qwen2.5-coder:latest

## 依赖
进入项目根目录，确保你安装了 npm 和 rust，具体参考：https://tauri.app/zh-cn/start/prerequisites/
- **tauri cli**
`npm install -g @tauri-apps/cli`
- **npm 依赖**
`npm install`
- **tauri 依赖**
`npm run tauri init`
- **运行**
`npm run tauri dev`

## 功能
1. 助手小弹窗，目前支持命令式对话，默认作为指令助手，目前测试可用指令：
    * 打个招呼
    * 生成一个生日快乐的网页并打开

2. 设置自定义模型、提示（施工中）
3. 支持更复杂的提示词工程（待做）
4. 针对实际场景进行微调（待做）