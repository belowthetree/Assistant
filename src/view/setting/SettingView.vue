
<template>
    <main class="drag-area">
        <h1>设置界面</h1>
    </main>
</template>

<script lang="ts">
export default {
    setup() {
        console.log("Init")
    }
}
</script>

<style>
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
	justify-content: center;
	text-align: center;
}
</style>