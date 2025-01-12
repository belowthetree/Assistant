<script lang="ts">
import { Close } from "@element-plus/icons-vue"
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { BaseConfig, loadConfig } from "~/src/config";
import { EModelType } from "~/src/data";
import { emitModelUpdateEvent } from "~/src/events/model_event";

export default {
    setup() {
        console.log("Init")
    },
    data() {
        return {
            currentPage: "modelList",
            transitionName: "slide-left",
            baseUrl: '',
            modelName: '',
            modelType: EModelType.Ollama,
            apiKey: '',
            modelTypes: Object.values(EModelType),
            roleCards: [],
        }
    },
    mounted() {
        loadConfig().then(()=>{
            console.log(BaseConfig)
            this.baseUrl = BaseConfig.baseUrl
            this.modelType = BaseConfig.modelType
            this.modelName = BaseConfig.modelName
            this.apiKey = BaseConfig.apiKey
            this.roleCards = BaseConfig.roleCards
        })
    },
    methods: {
        closeWindow() {
            console.log("close")
            WebviewWindow.getCurrent().close()
        },
        onConfigChange() {
            console.log("保存配置")
            BaseConfig.baseUrl = this.baseUrl
            BaseConfig.apiKey = this.apiKey
            BaseConfig.modelName = this.modelName
            BaseConfig.modelType = this.modelType
            emitModelUpdateEvent()
        },
        switchView() {
            this.currentPage = this.currentPage === "modelList" ? "modelDetail" : "modelList"
            if (this.currentPage !== "modelList")
                this.transitionName = "slide-left"
            else
                this.transitionName = "slide-right"
        }
    }
}
</script>

<template>
    <main class="drag-area macos-background">
        <div class="header">
            <div class="header-left"></div>
            <div class="header-center">
                <span class="app-name">设置</span>
            </div>
            <div class="header-right no-drag" style="">
                <button id="closebutton" @click="closeWindow">
                    <div style="margin: auto;">
                        <Close style="width: 15px;height: 15px;margin: auto;"/>
                    </div>
                </button>
            </div>
        </div>
        <transition :name="transitionName">
            <div :key="currentPage" class="panel">
                <div v-if="currentPage === 'modelList'" class="inputcontainer">
                    <button id="rightbutton" @click="switchView">
                        <el-icon ><ArrowRightBold /></el-icon>
                    </button>
                    <p>ttt</p>
                </div>
                <div class="inputcontainer no-drag" v-else>
                    <button id="leftbutton" @click="switchView">
                        <el-icon ><ArrowLeftBold /></el-icon>
                    </button>
                    <label>API Key</label>
                    <input class="input" @input="onConfigChange" type="password" v-model="apiKey" style="width: 240px"/>
                    <label>模型网址</label>
                    <input class="input" @input="onConfigChange" v-model="baseUrl" style="width: 240px"/>
                    <label>模型名称</label>
                    <input class="input" @input="onConfigChange" v-model="modelName" style="width: 240px"/>
                    <label>模型类型</label>
                    <el-select class="lightShadow" @change="onConfigChange" v-model="modelType" placeholder="Select" size="large" style="width: 240px;margin-left: auto;margin-right: auto;">
                        <el-option v-for="item in modelTypes" :key="item" :label="item" :value="item" style="border: none;outline: none;"/>
                    </el-select>
                    <!-- <el-select class="lightShadow" @change="onConfigChange" v-model="modelType" placeholder="Select" size="large" style="width: 240px;margin-left: auto;margin-right: auto;">
                        <el-option v-for="item in modelTypes" :key="item" :label="item" :value="item" style="border: none;outline: none;"/>
                    </el-select> -->
                </div>
            </div>
        </transition>
    </main>
</template>

<style>
/**面板过度 */
.panel {
    position: absolute;
    transition: transform 0.5s;
    left: 50%;
    top: 35px;
    width: 100%;
    transform: translate(-50%, 0);
}
.slide-left-enter-from {
  transform: translateX(100%);
}
.slide-left-leave-to {
  transform: translateX(-100%);
}

/* 从右到左的动画 */
.slide-right-enter-from {
  transform: translateX(-100%);
}
.slide-right-leave-to {
  transform: translateX(100%);
}
.slide-left-enter-active,
.slide-left-leave-active,
.slide-right-enter-active,
.slide-right-leave-active {
  transition: transform 0.5s;
}
#leftbutton {
    background-color: #00000000;
    outline: none;
    border: none;
    height: 200px;
    transform: translate(0, -50%);
    position: absolute;
    left: 5px;
    top:50%;
}
#leftbutton:hover {
    background: radial-gradient(closest-side, rgba(189, 189, 189, 0.35), rgba(255, 255, 255, 0)); /* 中心向四周渐变透明 */
}
#rightbutton {
    background-color: #00000000;
    outline: none;
    border: none;
    height: 200px;
    transform: translate(0, -50%);
    position: absolute;
    right: 5px;
    top:50%;
}
#rightbutton:hover {
    background: radial-gradient(closest-side, rgba(189, 189, 189, 0.35), rgba(255, 255, 255, 0)); /* 中心向四周渐变透明 */
}
/**面板过度结束 */
/**输入框 */
.input {
    width: 100%;
    padding: 10px;
    font-size: 16px;
    border-radius: 8px;
    outline: none;
    background-color: white;
    box-shadow: 0 2px 3px rgba(0, 0, 0, 0.1);
    outline: none;
    border: none;
    margin-left: auto;
    margin-right: auto;
    margin-bottom: 15px;
}
.lightShadow {
    box-shadow: 0 2px 3px rgba(0, 0, 0, 0.1);
}
/**输入面板 */
.inputcontainer {
    background: #ffffff00;
    padding: 20px;
    width: 100%;
    height: 500px;
    text-align: center;
    margin: auto;
    display: flex;
    flex-direction: column;
}
/**页头 */
.header {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    height: 35px;
    background-color: rgba(245, 245, 247, 0.8);
    /* 浅色背景 */
    backdrop-filter: blur(10px);
    /* 毛玻璃效果 */
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    /* 底部边框 */
}

#closebutton {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
    outline: none;
    border-top: none;
    border-bottom: none;
    border-right: none;
    border-left: 1px solid #00000022;
    background: linear-gradient(135deg, #f0f0f0, #ffffff);
}
#closebutton:hover {
    background: linear-gradient(135deg, #ff0000de, #ff0000de);
}
#closebutton:active {
    background: linear-gradient(135deg, #ad0000de, #ad0000de);
}

.header-left,
.header-center,
.header-right {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    align-content: center;
}

.header-left,
.header-right {
    width: 40px;
    height: 100%;
    border: none;
}
.header-center {
    width: 100%;
}
/**页头结束 */
.drag-area {
    /* 允许拖拽 */
    -webkit-app-region: drag;
    /* cursor: grab; */
}

.no-drag {
    -webkit-app-region: no-drag;
}

main {
    background-color: rgba(173, 216, 230, 0.623);
    height: 100%;
    width: 100%;
    margin: 0;
    display: flex;
    flex-direction: column;
}

.macos-background {
    height: 100vh;
    /* 使背景占满整个视口高度 */
    background: linear-gradient(135deg, #f0f0f0, #ffffff);
    /* 浅灰色到白色的渐变 */
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
    color: #333;
    /* 深灰色文字 */
    text-align: center;
}
</style>