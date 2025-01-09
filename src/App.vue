<script>
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { chat_with_tool, inputCommand, generate_instructions, generate } from "./lib/llm_interface";
import { ask, confirm } from "@tauri-apps/plugin-dialog";
import { TypescriptProcess } from "./frontend/typescript_process";
import { ECmdMode, EModelType, SetApiKey, SetModelType } from "./config";
import { getCurrentWindow, LogicalPosition } from '@tauri-apps/api/window';
import { LogicalSize, Position } from "@tauri-apps/api/dpi";
import { Setting } from "@element-plus/icons-vue";
import { createWindow } from "./lib/window";

var models = EModelType.Deepseek

const response = ref("")
const indexInput = ref("")
const apiKey = ref("")
const model = ref("qwen2.5-coder:7b")
const cmdMode = ECmdMode.Exec
var modeSelect = "Exec"
var commands = []
const output = ref("")
var ps = new TypescriptProcess(model.value)

async function setApiKey() {
	SetApiKey(apiKey.value)
	if (model.value == EModelType.Deepseek) {
		ps.deepseek.setApiKey(apiKey.value)
	}
}

async function commitCommand() {
	return
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
	data() {
		return {
			inputText: '',
			maxHeight: 500,
			currentHeight: 100,
			bottomHeight: 15,
			cmdInput: ""
		}
	},
	computed: {
		textareaStyle() {
			return {
			height: this.currentHeight,
			overflowY: this.currentHeight === this.maxHeight ? 'auto' : 'hidden'
			};
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
			createWindow(
				'setting',
				"设置",
				300,
				300,
				true
			)
		},
		onInput() {
			console.log("resize")
			const Window = getCurrentWindow()
			Window.outerSize().then(e=>console.log(e))
			Window.innerSize().then(e=>console.log(e))
			const input = document.getElementById("cmdInput")
			input.style.height = 'auto'
			const height = Math.min(input.scrollHeight, this.maxHeight)
			input.style.height = `${height}px`;
			if (input.scrollHeight > this.maxHeight) {
				input.style.overflowY = 'scroll'
			}
			console.log(height, input.style.height)
			Window.setSize(new LogicalSize(300, height + 65)).catch(e=>console.log(e))
		}
	},
	onMounted() {
		this.onInput()
	}
}


</script>

<template>
	<main class="container drag-area" id="container">
		<div class="row macos-background">
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

/* select,
input,
button {
	border-radius: 8px;
	border: 1px solid transparent;
	padding: 0.6em 1.2em;
	font-size: 1em;
	font-weight: 500;
	font-family: inherit;
	color: #0f0f0f;
	background-color: #ffffff;
	transition: border-color 0.25s;
	box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
} */

#cmdInput {
	outline: none;
	height: 100%;
	border-radius: 5px;
	resize: none;
	border: none;
	text-align: left;
	padding: 15px;
	padding-bottom: 0;
	background: linear-gradient(135deg, #f0f0f0, #ffffff); /* 浅灰色到白色的渐变 */
	font-size: 26px;
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