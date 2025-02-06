<script lang="ts">
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import Toast from "~/src/components/Toast.vue";
import { ModelConfig, ModelList, RoleCards, loadConfig, saveConfig } from "~/src/config";
import { EModelType } from "~/src/data";
import { emitModelUpdateEvent } from "~/src/events/model_event";
import { moveWindow, Position as WindowPosition } from "@tauri-apps/plugin-positioner";
import { openRoleCardView } from "~/src/lib/window";
import { RoleCardBase } from "~/src/rolecard/rolecardbase";
import { Ollama } from "~/src/model/ollama";
import { generateModelFromConfig } from "~/src/model/global";

export default {
    components: {
        'toast': Toast
    },
    setup() {
		moveWindow(WindowPosition.RightCenter)
    },
    data() {
        return {
            currentPage: "modelList",
            transitionName: "slide-left",
            model: new ModelConfig(""),
            modelTypes: [],
            models: {'a':new ModelConfig("")},
            editModelName: "",
            currentModelName: "",
            rolecards: [RoleCardBase],
            created : false,
            selectableModels: [],
        }
    },
    mounted() {
        loadConfig().then(()=>{
            this.refresh()
        })
    },
    methods: {
        closeWindow() {
            console.log("close")
            WebviewWindow.getCurrent().close()
        },
        onConfigChange() {
        },
        onURLChange() {
            this.selectableModels = []
            const m = generateModelFromConfig(this.model)
            console.log(m)
            if (m instanceof Ollama) {
                m.getModels().then((models)=>{
                    this.selectableModels = models
                    console.log(models)
                }).catch((e)=>console.warn(e))
            }
        },
        switchView(model) {
            this.created = false
            console.log(model)
            this.model = model
            this.editModelName = model.name
            this.currentPage = this.currentPage === "modelList" ? "modelDetail" : "modelList"
            if (this.currentPage !== "modelList")
                this.transitionName = "slide-left"
            else
                this.transitionName = "slide-right"
            this.onURLChange()
        },
        saveModel() {
            console.log("保存配置")
            const model = ModelList.getModelConfigByName(this.model.name)
            if (model && this.created) {
                this.$refs.toast.danger("存在同名配置")
                return
            }
            console.log(this.model)
            ModelList.saveModel(this.editModelName, this.model)
            ModelList.validate()
            console.log(ModelList)
            console.log(this.models)
            this.models = ModelList.Models
            console.log(ModelList.Models)
            saveConfig().then(()=>{
                this.refresh()
                this.$refs.toast.show("保存成功")
                emitModelUpdateEvent()
            })
        },
        saveSelect() {
            ModelList.currentConfig = this.currentModelName
            saveConfig().then(()=>{
                this.$refs.toast.show("保存成功")
                emitModelUpdateEvent()
            })
        },
        addModel() {
            this.model = new ModelConfig("模型")
            this.editModelName = this.model.name
            this.$refs.toast.show("添加配置")
            this.created = true
            this.onURLChange()
        },
        refresh() {
            const model = ModelList.getCurrentModelConfig()
            this.model = model
            this.modelTypes = Object.values(EModelType)
            this.models = ModelList.Models
            this.editModelName = ""
            this.currentModelName = ModelList.currentConfig
            this.rolecards = []
            for (let v of RoleCards) {
                this.rolecards.push(v.name)
            }
            this.$forceUpdate(); // 强制刷新
        },
        onPromptChanged() {
            // 
        },
        openRoleCardView() {
            console.log("open role")
            openRoleCardView()
        },
        selectModel(name) {
            const model = ModelList.getModelConfigByName(name)
            if (model) {
                this.currentModelName = name
            }
        }
    }
}
</script>

<template>
    <main class="drag-area maincontainer">
        <toast ref="toast" class="toast"></toast>
        <transition :name="transitionName">
            <div :key="currentPage" class="panel">
                <div v-if="currentPage === 'modelList'" class="inputcontainer">
                    <div style="display: flex;flex-direction: row;margin: 15px;margin-top: 0;">
                        <el-button type="info" @click="saveSelect" style="width: 40%;margin-left: auto;margin-right: auto;">保存</el-button>
                        <el-button type="info" @click="openRoleCardView" style="width: 40%;margin-left: auto;margin-right: auto;">设置角色卡</el-button>
                    </div>
                    <div :class="{'modelcard lightShadow no-drag': true, modelcard_selected: currentModelName===key}" v-for="(v, key) in models">
                        <button style="text-align: left;" class="cardbutton" @click="selectModel(v.name)">
                            <label>昵称：{{ key }}</label><br/>
                            <label>地址：{{ v.baseUrl }}</label><br/>
                            <label>类型：{{ v.modelType }}</label><br/>
                            <label>名字：{{ v.modelName }}</label><br/>
                            <label>角色：{{ v.roleCard }}</label><br/>
                        </button>
                        <button style="margin-left: auto;" id="rightbutton" @click="switchView(v)">
                            <el-icon ><ArrowRightBold /></el-icon>
                        </button>
                    </div>
                </div>
                <div class="inputcontainer no-drag" v-else>
                    <div class="" style="margin-top: 15px;">
                        <button @click="saveModel">保存</button>
                        <button @click="addModel">新增</button>
                    </div>
                    <button id="leftbutton" @click="switchView">
                        <el-icon ><ArrowLeftBold /></el-icon>
                    </button>
                    <label>名字</label>
                    <input class="input" @input="onConfigChange" v-model="this.model.name"/>
                    <label>API Key</label>
                    <input class="input" @input="onConfigChange" type="password" v-model="this.model.apiKey"/>
                    <label>模型地址</label>
                    <input class="input" @input="onURLChange" v-model="this.model.baseUrl"/>
                    <label>模型名称</label>
                    <input v-if="this.selectableModels.length <= 0" class="input" @input="onConfigChange" v-model="this.model.modelName"/>
                    <el-select v-else class="lightShadow" @change="onConfigChange" v-model="this.model.modelName" placeholder="Select" size="large" style="width: 240px;margin-left: auto;margin-right: auto;">
                        <el-option v-for="item in selectableModels" :key="item" :label="item" :value="item" style="border: none;outline: none;"/>
                    </el-select>
                    <label>模型类型</label>
                    <el-select class="lightShadow" @change="onConfigChange" v-model="this.model.modelType" placeholder="Select" size="large" style="width: 240px;margin-left: auto;margin-right: auto;">
                        <el-option v-for="item in modelTypes" :key="item" :label="item" :value="item" style="border: none;outline: none;"/>
                    </el-select>
                    <el-select class="lightShadow" @change="onConfigChange" v-model="this.model.roleCard" placeholder="Select" size="large" style="width: 240px;margin-left: auto;margin-right: auto;">
                        <el-option v-for="item in rolecards" :key="item" :label="item" :value="item" style="border: none;outline: none;"/>
                    </el-select>
                </div>
            </div>
        </transition>
    </main>
</template>

<style>
/**弹窗 tips */
.toast {
    position: absolute;
}
/**模型选择 */
.modelcard {
    background: linear-gradient(135deg, #f0f0f0, #ffffff);
    width: 400px;
    height: 120px;
    padding: 15px;
    padding-left: 25px;
    margin-left: auto;
    margin-right: auto;
    margin-top: 15px;
    outline: none;
    border: none;
    border-radius: 10px;
    display: flex;
}
.modelcard_selected {
    background: linear-gradient(135deg, #c9dcff, #ffffff);
}
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
  transform: translateX(150%);
}
.slide-left-leave-to {
  transform: translateX(-150%);
}

/* 从右到左的动画 */
.slide-right-enter-from {
  transform: translateX(-150%);
}
.slide-right-leave-to {
  transform: translateX(150%);
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
    background: radial-gradient(closest-side, rgba(211, 211, 211, 0.35), rgba(255, 255, 255, 0)); /* 中心向四周渐变透明 */
}
#rightbutton {
    background-color: #00000000;
    outline: none;
    border: none;
    height: 70px;
    transform: translate(0, -50%);
    position: relative;
    right: 0%;
    top:50%;
}
#rightbutton:hover {
    background: radial-gradient(closest-side, rgba(211, 211, 211, 0.35), rgba(255, 255, 255, 0)); /* 中心向四周渐变透明 */
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
    box-shadow: 0 5px 6px rgba(0, 0, 0, 0.1);
    outline: none;
    border: none;
    margin-left: auto;
    margin-right: auto;
    margin-bottom: 15px;
    width: 240px;
}
.lightShadow {
    box-shadow: 0 5px 6px rgba(0, 0, 0, 0.1);
}
/**输入面板 */
.inputcontainer {
    background: #ffffff00;
    width: 100%;
    height: 500px;
    text-align: center;
    margin: auto;
    display: flex;
    flex-direction: column;
    overflow-y: scroll;
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

.cardbutton {
    border: none;
    outline: none;
    background: transparent;
    width: 100%;
}
.cardbutton:hover {
    cursor: pointer;
}
.cardbutton:hover * {
    cursor: pointer;
}

#rolecardbutton {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
    outline: none;
    border-top: none;
    border-bottom: none;
    border-right: none;
    border-left: 1px solid #00000022;
    background-color: rgba(245, 245, 247, 0.8);
}
#rolecardbutton:hover {
    background-color: rgba(214, 214, 214, 0.8);
}
#rolecardbutton:active {
    background-color: rgba(214, 214, 214, 0.8);
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
</style>