<script lang="ts">
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import Toast from "~/src/components/Toast.vue";
import { RoleCards, loadConfig, saveConfig } from "~/src/config";
import { moveWindow, Position as WindowPosition } from "@tauri-apps/plugin-positioner";
import { RoleCardBase } from "~/src/rolecard/rolecardbase";
import { emitModelUpdateEvent } from "~/src/events/model_event";
import { PromptTemplate } from "~/src/prompt/template";

export default {
    components: {
        'toast': Toast
    },
    setup() {
		moveWindow(WindowPosition.RightCenter)
    },
    data() {
        return {
            currentPage: "roleCardList",
            roleCards: RoleCards,
            transitionName: "slide-left",
            currentRoleName: "",
            roleCard: new RoleCardBase(),
            created: false,
        }
    },
    mounted() {
        loadConfig().then(()=>{
            this.roleCard = RoleCards[0]
            this.currentRoleName = this.roleCard.name
        })
    },
    methods: {
        closeWindow() {
            console.log("close")
            WebviewWindow.getCurrent().close()
        },
        onConfigChange() {

        },
        switchView(model) {
            this.created = false
            this.model = model
            this.currentModelName = model.name
            this.currentPage = this.currentPage === "roleCardList" ? "roleCardDetail" : "roleCardList"
            if (this.currentPage !== "roleCardList")
                this.transitionName = "slide-left"
            else
                this.transitionName = "slide-right"
        },
        saveRoleCard() {
            console.log("保存角色卡配置")
            if (this.created) {
                for (let v of RoleCards) {
                    if (v.name === this.rolecard) {
                        this.$refs.toast.danger("存在同名角色卡")
                        return
                    }
                }
                RoleCards.push(this.roleCard)
            }
            saveConfig().then(()=>{
                this.$refs.toast.show("保存成功")
                emitModelUpdateEvent()
            })
        },
        addRoleCard() {
            this.created = true
            this.roleCard = new RoleCardBase()
            this.roleCard.prompt = PromptTemplate
        },
        refresh() {
            console.log(this.models)
            this.$forceUpdate(); // 强制刷新
        },
        onPromptChanged() {
            // 
        }
    }
}
</script>

<template>
    <main class="drag-area maincontainer">
        <div class="header">
            <div class="header-left"></div>
            <div class="header-center">
                <span class="app-name">角色卡</span>
            </div>
            <div class="header-right no-drag" style="">
                <button id="closebutton" @click="closeWindow">
                    <div style="margin: auto;">
                        <Close style="width: 15px;height: 15px;margin: auto;"/>
                    </div>
                </button>
            </div>
        </div>
        <toast ref="toast" class="toast"></toast>
        <transition :name="transitionName">
            <div :key="currentPage" class="panel">
                <div v-if="currentPage === 'roleCardList'" class="inputcontainer">
                    <div class="rolecard lightShadow no-drag" v-for="(v, _) in roleCards">
                        <div style="text-align: left;">
                            <label>角色名：{{ v.name }}</label><br/><br/>
                            <label style="text-overflow: ellipsis;overflow: hidden;display: -webkit-box;-webkit-box-orient: vertical;-webkit-line-clamp: 2;">简述：{{ v.roleDesc }}</label><br/>
                        </div>
                        <button style="margin-left: auto;" id="rightbutton" @click="switchView(v)">
                            <el-icon ><ArrowRightBold /></el-icon>
                        </button>
                    </div>
                </div>
                <div class="inputcontainer no-drag" v-else>
                    <div class="" style="margin-top: 15px;">
                        <button @click="saveRoleCard">保存</button>
                        <button @click="addRoleCard">新增</button>
                    </div>
                    <button id="leftbutton" @click="switchView">
                        <el-icon ><ArrowLeftBold /></el-icon>
                    </button>
                    <label>名字</label>
                    <input class="input" @input="onConfigChange" v-model="this.roleCard.name"/>
                    <textarea id="systemPromptInput" @input="onPromptChanged" class="no-drag lightShadow" v-model="roleCard.systemPrompt" placeholder="系统提示词"></textarea>
                    <!-- <el-select class="lightShadow" @change="onConfigChange" v-model="this.model.modelType" placeholder="Select" size="large" style="width: 240px;margin-left: auto;margin-right: auto;">
                        <el-option v-for="item in modelTypes" :key="item" :label="item" :value="item" style="border: none;outline: none;"/>
                    </el-select>
                    <textarea id="userInput" @input="onPromptChanged" class="no-drag" v-model="this.model.roleCard.systemPrompt" placeholder="系统提示词"></textarea> -->
                    <!-- <el-select class="lightShadow" @change="onConfigChange" v-model="modelType" placeholder="Select" size="large" style="width: 240px;margin-left: auto;margin-right: auto;">
                        <el-option v-for="item in modelTypes" :key="item" :label="item" :value="item" style="border: none;outline: none;"/>
                    </el-select> -->
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
.rolecard {
    background: linear-gradient(135deg, #f0f0f0, #ffffff);
    width: 400px;
    height: 80px;
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
/**面板过度 */
.panel {
    position: absolute;
    transition: transform 0.5s;
    left: 50%;
    top: 35px;
    width: 100%;
    height: 100%;
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
    box-shadow: 6px 10px 12px 10px rgba(0, 0, 0, 0.1);
}
/**输入面板 */
.inputcontainer {
    background: #ffffff00;
    padding-top: 20px;
    width: 100%;
    height: 90%;
    min-height: 400px;
    text-align: center;
    margin: auto;
    margin-bottom: 0;
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

#systemPromptInput {
    margin: 25px;
    height: 100%;
    margin-top: 15px;
    padding: 5px;
    resize: none;
    outline: none;
    border: none;
    border-radius: 5px;
}
</style>