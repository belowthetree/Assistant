<script setup>
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";
import { chat_with_tool, inputCommand, generate_instructions, generate } from "./lib/llm_interface";
import { ask, confirm } from "@tauri-apps/plugin-dialog";
import { CommandProcess } from "./frontend/command_process";

const models = [
	"qwen2.5-coder:1.5b",
	"qwen2.5-coder:latest",
	"deepseek-coder-v2:latest",
	"qwq:latest",
]

const cmdInput = ref("")
const commandInput = ref("")
const response = ref("")
const model = ref("qwen2.5-coder:1.5b")
var commands = []
const output = ref("")
var ps = new CommandProcess(model.value)

async function commitCommand() {
	ps.model = model.value
	ps.inputQuestion(cmdInput.value)
	// const res = await generate_instructions(model.value, cmdInput.value)
    // response.value = "回答：" + res
	// console.log(response.value)
	// const js = JSON.parse(res)
	// commands = js.instructions
	// output.value = ""
	// processCommand(res)
}

async function commitCommandRaw() {
	const res = await generate(model.value, commandInput.value)
    response.value = "回答：" + res
	console.log(response.value)
}

async function invokeCommand() {
	const res = await ps.invokeCommand()
	console.log(res)
}

function processCommand(res) {
	const js = JSON.parse(res)
	if (js.function && js.function.length > 0) {
		let arr = []
		for (let i = 0;i < js.function.length; ++i) {
			const param = {}
			for (let p in js.function[i].parameter) {
				const t = js.function[i].parameter[p]
				param[t.name] = t.value
			}
			arr.push({
				name: js.function[i].name,
				param: param,
			})
		}
		ask('是否执行：' + res, {
			title: "确认命令",
			kind: "warning",
		}).then((res)=>{
			if (res) {
				console.log("准备执行", arr)
				for (let idx in arr) {
					const v = arr[idx]
					console.log(v.name, v.param)
					invoke(v.name, v.param)
				}
			}
		})
	}
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
            <form class="row" @submit.prevent="commitCommandRaw">
                <input id="chat-input" v-model="commandInput" placeholder="输入指令"/>
                <button type="submit">提交</button>
            </form>
        </div>
        <div class="row">
            <form class="row" @submit.prevent="commitCommand">
                <input id="cmd-input" v-model="cmdInput" placeholder="输入指令"/>
                <button type="submit">提交</button>
            </form>
        </div>
        <p>{{ response }}</p>
		<button @click="invokeCommand">下个命令</button>
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

#cmd-input {
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