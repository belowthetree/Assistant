<script>
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { chat_with_tool, inputCommand, generate_instructions, generate } from "./lib/llm_interface";
import { ask, confirm } from "@tauri-apps/plugin-dialog";
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
import { listenTalkViewQueryEvent } from "./events/window_event";
import { ServerConfigInfo } from "./frontend/MCPServer";
import { Talk } from "./life/talk/talk"
import Conversation from "./components/Conversation.vue"
import { listen } from '@tauri-apps/api/event';

var talk = null
var modeSelect = "Exec"
var commands = []

var isDragging = false
var startX = 0
var startY = 0

export default {
	components: {
		'conversation': Conversation
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
			disableInput: false,
			modelOutput: "",
			conversationHeight: 0,
			inputMaxHeight: 150,
			modelOutputVisible: false,
			modelReplyUnlisten: undefined,
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
			const inputHeight = Math.min(input.scrollHeight, this.inputMaxHeight)
			input.style.height = `${inputHeight}px`;
			if (input.scrollHeight > this.inputMaxHeight) {
				input.style.overflowY = 'scroll'
			}
			console.log(inputHeight, this.conversationHeight)
			const totalHeight = Math.min(this.maxHeight, inputHeight + this.conversationHeight + 35)
			Window.setSize(new LogicalSize(300, totalHeight)).catch(e => console.log(e))
		},

		updateConversationHeight() {
			if (this.$refs.conversation) {
				const conversationEl = this.$refs.conversation.$el
				this.conversationHeight = conversationEl.scrollHeight + 15
			}
			else {
				this.conversationHeight = 0
			}
			this.modelOutputVisible = this.modelOutput && this.modelOutput.length > 0
		},
		async commitCommand() {
			try {
				console.log("输入命令", this.userInput)
				this.disableInput = true
				invoke("talk", { ctx: this.userInput }).then(e => {
					this.disableInput = false
					console.log(e)
				}).catch(e => {
					this.disableInput = false
					console.warn(e)
				})
			}
			catch (e) {
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
		onModelReply(content) {
			console.log(content)
			this.modelOutput = content
			this.updateConversationHeight()
		}
	},
	mounted() {
		// 监听模型更新
		listen("AssistantReplyEventName", (ev)=>{
			this.onModelReply(ev.payload)
		}).then((unli)=>{
			this.modelReplyUnlisten = unli
		})
		this.updateConversationHeight()
		this.onInput() // 触发窗口高度更新
		const mainWindow = webviewWindow.getCurrentWebviewWindow()
		// 监听窗口显示事件
		mainWindow.once('tauri://focus', () => {
			// 窗口显示时，聚焦输入框
			const input = document.getElementById("userInput")
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

		// 监听 Conversation 内容变化
		this.$watch('modelOutput', () => {
			this.$nextTick(() => {
				this.updateConversationHeight()
			})
		})
	},
	destroyed() {
		if (this.unli) {
			this.unli();
		}
	}
}
//创建 vue 页面基本模板并复制到剪贴板

</script>

<template>
	<main class="maincontainer rounded-lg " id="maincontainer">
		<div class="row macos-background rounded-lg flex flex-col justify-end">
			<!-- <Bubbles ref="bubbles" style="color: black;">fff</Bubbles> -->
			<conversation v-if="modelOutputVisible" ref="conversation" :content="modelOutput"
				class="no-drag rounded-lg"></conversation>
			<hr style="width: 80%;margin: auto; margin-bottom: 15px;margin-top: 15px;" />
			<textarea @input="onInput" :readonly="disableInput" @keydown="onKeyDown" @keyup="onKeyUp" id="userInput"
				class="no-drag rounded-lg " v-model="userInput" placeholder="输入你想说的话然后按下回车"></textarea>
			<button class="right_bottom" @click="clickSetting">
				<i class="iconBtn fa-solid fa-cog hover_color fa-5" id="settingIcon" :style="{ color: 'black' }"></i>
			</button>
			<button class="right_bottom" @click="clickKnowledge" style="right: 45px;">
				<i class="iconBtn fa fa-microphone hover_color fa-5 " id="voiceIcon" :style="{ color: 'black' }"></i>
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
	margin-bottom: 40px;
	background: transparent;
	font-size: 21px;
	min-height: 100px;
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
