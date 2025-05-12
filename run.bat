@echo off
REM 确保路径正确
echo 当前脚本目录：%~dp0

REM 执行 ts-node 命令
call npx ts-node %~dp0public\buildprompt.js || (
  echo 执行 ts-node 失败，请检查路径或依赖
  exit /b
)

npm install
npm run build

REM 执行 tauri dev 命令
call npx tauri dev || (
  echo 执行 tauri dev 失败
  exit /b
)