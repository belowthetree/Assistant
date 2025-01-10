<script>
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { chat_with_tool, inputCommand, generate_instructions, generate } from "./lib/llm_interface";
import { ask, confirm } from "@tauri-apps/plugin-dialog";
import { TypescriptProcess } from "./frontend/typescript_process";
import { SetApiKey, SetModelType } from "./config";
import { getCurrentWindow, LogicalPosition } from '@tauri-apps/api/window';
import { LogicalSize, Position } from "@tauri-apps/api/dpi";
import { Setting } from "@element-plus/icons-vue";
import { openSettingWindow } from "./lib/window";
import Bubbles from "./components/Bubbles.vue";
import { EModelType, ECmdMode } from "@/data";

var models = EModelType.Deepseek

const response = ref("")
const indexInput = ref("")
const apiKey = ref("")
const model = ref("qwen2.5-coder:7b")
const cmdMode = ECmdMode.Exec
var modeSelect = "Exec"
var commands = []

async function commitCommand() {
	console.log(modeSelect)
	SetModelType(model.value)
	ps.model = model.value
	if (model.value == EModelType.Deepseek) {
		ps.deepseek.setApiKey(apiKey.value)
	}
	switch (modeSelect) {
		case ECmdMode.CommandSequence:
			{
				console.log('sequence')
				ps.generateCommands(cmdInput.value)
			}
			break
		case ECmdMode.TypescriptCode:
			ps.inputQuestion(cmdInput.value)
			break
		case ECmdMode.Exec:
			try {
				ps.generateModule(cmdInput.value)
			}
			catch (e) {
				if (e instanceof Error) {
					console.log(e.stack)
				}
				else {
					console.log(e)
				}
			}
			break
	}
}

var isDragging = false
var startX = 0
var startY = 0

const Window = getCurrentWindow()

function startDrag(ev) {
	console.log("start")
      isDragging = true;
      startX = ev.clientX;
      startY = ev.clientY;
    }
async function onDrag(ev) {
	if (isDragging) {
		console.log(ev)
		const offsetX = ev.clientX - startX;
		const offsetY = ev.clientY - startY;
		const { x, y } = await Window.innerPosition();
		const p = new LogicalPosition(x + offsetX, y + offsetY)
		await Window.setPosition(p);
		startX = ev.clientX;
		startY = ev.clientY;
	}
}
function stopDrag() {
	isDragging = false;
}

export default {
	components: {
		Bubbles,
	},
	setup() {
	},
	data() {
		return {
			inputText: '',
			maxHeight: 500,
			minHeight: 100,
			cmdInput: ""
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
		onInput() {
			console.log("resize")
			const Window = getCurrentWindow()
			const input = document.getElementById("cmdInput")
			input.style.height = 'auto'
			const height = Math.min(input.scrollHeight, this.maxHeight)
			input.style.height = `${height}px`;
			if (input.scrollHeight > this.maxHeight) {
				input.style.overflowY = 'scroll'
			}
			Window.setSize(new LogicalSize(300, height)).catch(e=>console.log(e))
		}
	},
	mounted() {
		this.onInput()
	}
}


</script>

<template>
	<main class="container drag-area" id="container">
		<div class="row macos-background">
			<Bubbles ref="bubbles" style="color: black;">fff</Bubbles>
			<textarea @input="onInput" id="cmdInput" class="no-drag" v-model="cmdInput" placeholder="输入指令"></textarea>
			<button class="right_bottom" @click="clickSetting">
				<Setting class="hover_color" id="settingIcon" :style="{color: 'black'}" />
			</button>
		</div>
	</main>
</template>

<style>
.right_bottom {
	position: absolute;
	right: 5px;
	bottom: 25px;
	color: #0f0f0f00;
	outline: none;
	border: none;
	background-color: #0f0f0f00;
}
.hover_color {
	position: absolute;
	width: 25px;
	height: 25px;
	top: 50%;
	left: 0%;
	transform: translate(-50%, -50%);
	transition: width 0.3s ease;
	transition: height 0.3s ease;
}
.hover_color:hover {
	width: 30px;
	height: 30px;
	transition: width 0.3s ease;
	transition: height 0.3s ease;
}
.hover_color.clicked {
	width: 25px;
	height: 25px;
	transition: width 0.3s ease;
	transition: height 0.3s ease;
}
.drag-area {
	 /* 允许拖拽 */
	-webkit-app-region: drag;
	/* cursor: grab; */
}
.no-drag {
	-webkit-app-region: no-drag;
}
.container {
	margin: 0;
	display: flex;
	flex-direction: column;
	justify-content: center;
	text-align: center;
	width: 100%;
	height: 100%;
	background-color: #e8e8e800;
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

#cmdInput {
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