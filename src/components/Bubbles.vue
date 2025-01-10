<template>
    <div class="bubble-container">
        <div v-for="(bubble, index) in bubbles" :key="index" :style="getBubbleStyle(index)" class="bubble"
            v-html="bubble.content"></div>
        <button @click="addBubble">添加气泡</button>
    </div>
</template>

<script>
export default {
    data() {
        return {
            bubbles: [],
            bubbleHeight: 60, // 每个气泡的高度
            bubbleMargin: 10, // 气泡之间的间距
        };
    },
    methods: {
        addBubble() {
            const newBubble = {
                content: `<strong>气泡 ${this.bubbles.length + 1}</strong>`, // 富文本内容
            };
            this.bubbles.unshift(newBubble); // 将新气泡添加到数组的开头

            // 设置气泡消失的定时器
            setTimeout(() => {
                this.bubbles.pop(); // 移除最后一个气泡
            }, 5000); // 5秒后气泡消失
        },
        getBubbleStyle(index) {
            return {
                bottom: `${(this.bubbleHeight + this.bubbleMargin) * index}px`,
                transition: 'bottom 0.5s ease-in-out',
            };
        },
    },
};
</script>

<style>
.bubble-container {
    position: relative;
    height: 400px;
    /* 容器高度 */
    overflow: hidden;
}

.bubble {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    width: 200px;
    padding: 10px;
    background-color: #007bff;
    color: white;
    border-radius: 10px;
    text-align: center;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    transition: opacity 0.5s ease-in-out;
}
</style>