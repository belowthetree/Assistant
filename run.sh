#!/bin/bash
# 确保路径正确
script_path=$(pwd)
echo "当前脚本目录：$script_path"

# 执行 ts-node 命令
npx ts-node "$script_path/public/buildprompt.js" || {
  echo "执行 ts-node 失败，请检查路径或依赖"
  exit 1
}

# 执行 tauri dev 命令
npm run tauri dev || {
  echo "执行 tauri dev 失败"
  exit 1
}