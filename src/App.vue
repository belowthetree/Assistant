<script>
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { chat_with_tool, inputCommand, generate_instructions, generate } from "./lib/llm_interface";
import { ask, confirm } from "@tauri-apps/plugin-dialog";
import { generateModelFromConfig } from "./model/global";
import { getCurrentWindow, LogicalPosition } from '@tauri-apps/api/window';
import { LogicalSize, Position } from "@tauri-apps/api/dpi";
import { Setting } from "@element-plus/icons-vue";
import { openKnowledgeWindow, openSettingWindow } from "./lib/window";
import Bubbles from "./components/Bubbles.vue";
import { ModulePrompt } from "./prompt/module_prompt";
import { emitModelResultEvent, listenModelUpdateEvent } from "./events/model_event";
import { webviewWindow } from "@tauri-apps/api";
import { addBubble } from "./view/Talk/api";
import { moveWindow, Position as WindowPosition } from "@tauri-apps/plugin-positioner";
import { loadConfig, ModelList } from "./config";
import { listenTalkViewQueryEvent } from "./events/window_event";
import { ServerConfigInfo } from "./frontend/MCPServer";
import { Talk } from "./life/talk/talk"

var talk = null
var modeSelect = "Exec"
var commands = []

var isDragging = false
var startX = 0
var startY = 0

export default {
	components: {
		Bubbles,
	},
	setup() {
		moveWindow(WindowPosition.RightCenter)
	},
	data() {
		return {
			inputText: '',
			maxHeight: 500,
			minHeight: 70,
			userInput: "",
			controlDown: false,
			reply: "",
		}
	},
	methods: {
		async clickSetting() {
			const icon = document.getElementById("settingIcon")
			// 触发动画
			icon.classList.add('clicked');
			setTimeout(() => {
				icon.classList.remove('clicked');
			}, 300); // 动画持续时间
			// this.$refs.bubbles.addBubble()
			openSettingWindow()
		},
		clickKnowledge() {
			const icon = document.getElementById("voiceIcon")
			// 触发动画
			icon.classList.add('clicked');
			setTimeout(() => {
				icon.classList.remove('clicked');
			}, 300); // 动画持续时间
			openKnowledgeWindow()
		},
		onInput() {
			const Window = getCurrentWindow()
			const input = document.getElementById("userInput")
			input.style.height = 'auto'
			const height = Math.min(input.scrollHeight, this.maxHeight)
			input.style.height = `${height}px`;
			if (input.scrollHeight > this.maxHeight) {
				input.style.overflowY = 'scroll'
			}
			Window.setSize(new LogicalSize(300, height + 35)).catch(e=>console.log(e))
		},
		async commitCommand() {
			try {
				console.log("输入命令", this.userInput)
				talk.userSay(this.userInput)
			}
			catch(e) {
				console.log(e)
			}
		},
		onKeyDown(event) {
			if (event.key == "Control")
				this.controlDown = true
			else if (!this.controlDown && event.key == "Enter") {
				this.commitCommand()
				event.preventDefault()
			}
		},
		onKeyUp(event) {
			if (event.key == "Control")
				this.controlDown = false
		},
	},
	mounted() {
		loadConfig().then(()=>{
			// 生成聊天对象
			const model = generateModelFromConfig(ModelList.getCurrentModelConfig())
			talk = new Talk(model, "")
		})
		listenModelUpdateEvent(()=>{
			loadConfig().then(()=>{
				// 更新模型
				const model = generateModelFromConfig(ModelList.getCurrentModelConfig())
				talk.setModel(model)
			})
			console.log(ModelList)
		})
		this.onInput()
		const mainWindow = webviewWindow.getCurrentWebviewWindow()
		// 监听窗口显示事件
		mainWindow.once('tauri://focus', () => {
			// 窗口显示时，聚焦输入框
			const input = document.getElementById("userInput")
			console.log("focus")
			input.focus()
			// 为元素添加拖拽事件
			document.getElementById('maincontainer').addEventListener('mousedown', async (e) => {
				if (e.button === 0) { // 左键点击
					console.log("drag")
					const mainWindow = webviewWindow.getCurrentWebviewWindow()
					await mainWindow.startDragging();
				}
			});
		});
	}
}
//创建 vue 页面基本模板并复制到剪贴板

</script>

<template>
	<main class="maincontainer drag-area" id="maincontainer">
		<div class="row macos-background">
			<!-- <Bubbles ref="bubbles" style="color: black;">fff</Bubbles> -->
			<textarea @input="onInput" @keydown="onKeyDown" @keyup="onKeyUp" id="userInput" class="no-drag" v-model="userInput" placeholder="输入指令"></textarea>
			<button class="right_bottom" @click="clickSetting">
				<i class="iconBtn fa-solid fa-cog hover_color fa-5" id="settingIcon" :style="{color: 'black'}" ></i>
			</button>
			<button class="right_bottom" @click="clickKnowledge" style="right: 45px;">
				<i class="iconBtn fa fa-microphone hover_color fa-5 " id="voiceIcon" :style="{color: 'black'}"></i>
			</button>
		</div>
	</main>
</template>

<style>
.right_bottom {
	position: absolute;
	right: 15px;
	bottom: 25px;
	color: #0f0f0f00;
	outline: none;
	border: none;
	width: 1rem;
	background-color: #0f0f0f00;
}
.hover_color {
	position: absolute;
	width: 100%;
	height: 100%;
	top: 50%;
	left: 50%;
	transform: translate(-50%, -50%);
	transition: all 0.1s ease;
}
.hover_color:hover {
	transform: translate(-50%, -50%) scale(1.1);
	transition: all 0.1s ease;
}
.hover_color.clicked {
	transform: translate(-50%, -50%) scale(1);
	transition: all 0.1s ease;
}

i {
	font-size: 0.8rem;
}

.drag-area {
	 /* 允许拖拽 */
	-webkit-app-region: drag;
	app-region: drag;
	/* cursor: grab; */
}
.no-drag {
	-webkit-app-region: no-drag;
	app-region: no-drag;
}
.maincontainer {
	margin: 0;
	display: flex;
	flex-direction: column;
	justify-content: center;
	text-align: center;
	width: 100%;
	height: 100%;
	background-color: #8c070700;
}

:root {
	font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
	font-size: 26px;
	line-height: 24px;
	font-weight: 400;

	color: #0f0f0f00;
	background-color: #f6f6f600;

	font-synthesis: none;
	text-rendering: optimizeLegibility;
	-webkit-font-smoothing: antialiased;
	-moz-osx-font-smoothing: grayscale;
	-webkit-text-size-adjust: 100%;
	background: transparent;
	border: none;
	outline: none;
}

.row {
	margin: 0;
	display: flex;
	flex-direction: column;
	padding: 0;
	height: 100%;
	width: 100%;
}

#userInput {
	outline: none;
	height: 100%;
	border-radius: 5px;
	resize: none;
	border: none;
	text-align: left;
	padding: 10px;
	padding-bottom: 0;
	margin: 5px;
	margin-bottom: 40px;
	background: linear-gradient(135deg, #f0f0f0, #ffffff); /* 浅灰色到白色的渐变 */
	font-size: 21px;
	min-height: 100px;
}

.macos-background {
  height: 100vh; /* 使背景占满整个视口高度 */
  background: linear-gradient(135deg, #f0f0f0, #ffffff); /* 浅灰色到白色的渐变 */
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  color: #333; /* 深灰色文字 */
  text-align: center;
}

@media (prefers-color-scheme: dark) {
	:root {
		color: #f6f6f600;
		background-color: #2f2f2f00;
		background: transparent;
		border-width: 0;
	}

	a:hover {
		color: #24c8db;
	}
}
</style>

<script>
</script>
