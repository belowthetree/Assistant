<script setup>
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";
import { chat_with_tool, inputCommand, generate_instructions, generate } from "./lib/llm_interface";
import { ask, confirm } from "@tauri-apps/plugin-dialog";
import { TypescriptProcess } from "./frontend/typescript_process";
import { ECmdMode } from "./frontend/frontenddata";

const models = [
	"qwen2.5-coder:latest",
	"deepseek-coder-v2:latest",
	"deepseek",
	"qwq:latest",
]

const cmdInput = ref("")
const response = ref("")
const indexInput = ref("")
const model = ref("qwen2.5-coder:latest")
const cmdMode = ECmdMode.TypescriptCode
var modeSelect = "TypescriptCode"
var commands = []
const output = ref("")
var ps = new TypescriptProcess(model.value)

async function commitCommand() {
	console.log(modeSelect)
	ps.model = model.value
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
	}
}

async function invokeCommand() {
	ps.nextCommand()
}

function setIndex() {
	ps.index = indexInput.value
}

</script>

<template>
	<main class="container">
		<h1>LLM 快捷指令</h1>
		<div>
			<p>{{ model }}</p>
		</div>
		<div class="row">
			<select name="model" id="select-model" v-model="model">
				<option v-for="name in models" :value="name">{{ name }}</option>
			</select>
		</div>
		<div class="row">
			<select name="cmdMode" id="select-mode" v-model="modeSelect">
				<option v-for="(name, v) in ECmdMode" :value="v">{{ v }}</option>
			</select>
		</div>
		<div class="row">
			<form class="row" @submit.prevent="commitCommand">
				<input class="cmd-input" v-model="cmdInput" placeholder="输入指令"/>
				<button type="submit">提交</button>
			</form>
		</div>
		<p>{{ response }}</p>
		<div class="row">
			<form class="row" @submit.prevent="setIndex">
				<input class="cmd-input" v-model="indexInput" placeholder="输入命令下标"/>
				<button type="submit">提交</button>
			</form>
			<button @click="invokeCommand">下个命令</button>
		</div>
	</main>
</template>

<style>
.container {
	margin: 0;
	padding-top: 10vh;
	display: flex;
	flex-direction: column;
	justify-content: center;
	text-align: center;
}

:root {
		font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
		font-size: 16px;
		line-height: 24px;
		font-weight: 400;

		color: #0f0f0f;
		background-color: #f6f6f6;

		font-synthesis: none;
		text-rendering: optimizeLegibility;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
		-webkit-text-size-adjust: 100%;
}

h1 {
		text-align: center;
}

.row {
	display: flex;
	justify-content: center;
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

.cmd-input {
	margin-right: 5px;
}

#select-model {
	margin: 5px;
}

@media (prefers-color-scheme: dark) {
	:root {
		color: #f6f6f6;
		background-color: #2f2f2f;
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