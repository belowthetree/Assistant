<script setup>
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";
import { chat_with_tool, inputCommand, generate_instructions, generate } from "./lib/llm_interface";
import { ask, confirm } from "@tauri-apps/plugin-dialog";
import { TypescriptProcess } from "./frontend/typescript_process";
import { ECmdMode, EModelType } from "./frontend/frontenddata";
import { CurrentApiKey, CurrentModelType } from "./global";

var models = EModelType.Deepseek

const cmdInput = ref("")
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
	CurrentApiKey = apiKey.value
	if (model.value == EModelType.Deepseek) {
		ps.deepseek.setApiKey(apiKey.value)
	}
}

async function commitCommand() {
	return
	console.log(modeSelect)
	CurrentModelType = model.value
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

async function invokeCommand() {
	ps.nextCommand()
}

function setIndex() {
	ps.index = indexInput.value
}

var isDragging = false
var startX = 0
var startY = 0

import { getCurrentWindow, LogicalPosition } from '@tauri-apps/api/window';
import { Position } from "@tauri-apps/api/dpi";

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

function onInput() {
	const input = document.getElementById("cmdInput")
	input.style.height = 'auto'
	input.style.height = `${input.scrollHeight}px`;
}

</script>

<template>
	<main class="container drag-area"
		@mousedown="startDrag"
		@mousemove="onDrag"
		@mouseup="stopDrag"
		@mouseleave="stopDrag">
		<h1>LLM 快捷指令</h1>
		<div class="row no-drag" onmousedown="event.stopPropagation()">
			<form class="row" @submit.prevent="commitCommand">
				<textarea @input="onInput" id="cmdInput" class="cmd-input" rows="5" v-model="cmdInput" placeholder="输入指令"></textarea>
			</form>
		</div>
	</main>
</template>

<style>
.drag-area {
	 /* 允许拖拽 */
	-webkit-app-region: drag;
	/* cursor: grab; */
}
.no-drag {
	-webkit-app-region: no-drag;
}
.container {
	margin: auto;
	display: flex;
	flex-direction: column;
	justify-content: center;
	text-align: center;
	width: 100%;
	height: 100%;
}

:root {
	font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
	font-size: 16px;
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
}

h1 {
	text-align: center;
}

.row {
	display: flex;
	flex-direction: column;
	padding: 5px;
	height: auto;
	min-height: 100px;
}

select,
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
}

.cmd-input {
	outline: none;
	height: auto;
	border-radius: 5px;
	padding: 5px;
}

button {
	cursor: pointer;
}

button:hover {
	border-color: #396cd8;
}
button:active {
	border-color: #396cd8;
	background-color: #e8e8e8;
}

select,
input,
button {
	outline: none;
}

#select-model {
	margin: 5px;
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

	input,
	button {
		color: #ffffff;
		background-color: #0f0f0f98;
	}
	button:active {
		background-color: #0f0f0f69;
	}
}
</style>