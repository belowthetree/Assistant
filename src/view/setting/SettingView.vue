<script lang="ts">
import "@/style/common.css"
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { moveWindow, Position } from "@tauri-apps/plugin-positioner";
import { openModelSettingWindow, openRoleCardView } from "~/src/lib/window";

export default {
    setup() {
        moveWindow(Position.RightCenter)
    },
    methods: {
        closeWindow() {
            WebviewWindow.getCurrent().close()
        },
        openModelSettingView() {
            openModelSettingWindow()
        },
        openRoleCardSettingView() {
            openRoleCardView()
        }
    }
}
</script>

<template>
    <main class="maincontainer">
        <button class="macos-background lightshadow macosfont cardbutton" @click="openModelSettingView">模型设置</button>
        <button class="macos-background lightshadow macosfont cardbutton" @click="openRoleCardSettingView">角色卡设置</button>
    </main>
</template>

<style>
main {
    background-color: rgba(173, 216, 230, 0.623);
    height: 100%;
    width: 100%;
    margin: 0;
    display: flex;
    flex-direction: column;
}

.maincontainer {
    height: 100vh;
    /* 使背景占满整个视口高度 */
    background: linear-gradient(135deg, #f0f0f0, #ffffff);
    /* 浅灰色到白色的渐变 */
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
    color: #333;
    /* 深灰色文字 */
    text-align: center;
}

.cardbutton {
    height: 50px;
    margin: 15px;
    border-radius: 5px;
    border: none;
    outline: none;
}
.cardbutton:active {
    margin: 20px;
}
</style>