<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
	// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
	greetMsg.value = await invoke("greet", { nameN: name.value });
}
</script>

<template>
	<div class="app">
		<h1>无边框窗口</h1>
		<p>这是一个无边框窗口的示例。</p>
	</div>
</template>

<style scoped>
body {
	margin: 0;
	padding: 0;
	overflow: hidden;
	background: transparent;
}

.app {
	width: 100%;
	height: 100%;
	background: #ffffff;
	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
	font-family: Arial, sans-serif;
	border: 1px solid #ccc;
	/* 添加边框以区分窗口 */
}
</style>