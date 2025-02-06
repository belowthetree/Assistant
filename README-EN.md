# Assistant

## Links
- [中文 README](README.md)
- [English README](README-EN.md)

A frontend for an AI assistant that provides intelligent assistant services through prompt engineering and API integration with a specified large model service.
> Default model used: ollama qwen2.5-coder:latest

## Dependencies
Navigate to the project root directory and ensure you have npm and Rust installed. For details, refer to: https://tauri.app/zh-cn/start/prerequisites/ (Chinese) / https://tauri.app/v1/guides/getting-started/prerequisites (English)
- **Tauri CLI**
`npm install -g @tauri-apps/cli`
- **npm Dependencies**
`npm install`
- **Tauri Dependencies**
`npm run tauri init`
- **Run**
`npm run tauri dev`

## Features
1. Assistant pop-up window, currently supports command-based dialogue, default as a command assistant, currently tested available commands:
    * Say hello
    * Generate a happy birthday webpage and open it

2. Set custom models and prompts (under construction)
3. Support for more complex prompt engineering (to be done)
4. Fine-tuning for actual scenarios (to be done)
