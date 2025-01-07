<script lang="ts">
    import { ECmdMode, EModelType } from "../frontend/frontenddata";
</script>

<template>
    <div class="home">
		<div class="row">
			<select name="model" id="select-model" v-model="model">
				<option v-for="name in EModelType" :value="name">{{ name }}</option>
			</select>
			<input class="cmd-input" v-model="apiKey" placeholder="输入 apikey" type="password" @input="setApiKey"/>
		</div>
		<div class="row">
			<select name="cmdMode" id="select-mode" v-model="modeSelect">
				<option v-for="(_, v) in ECmdMode" :value="v">{{ v }}</option>
			</select>
		</div>
    </div>
</template>
