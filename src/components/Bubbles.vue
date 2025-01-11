<script>
import { marked } from 'marked';
import "@/style/common.css"

export default {
    data() {
        return {
            bubbles: [],
            bubbleHeight: 60, // 每个气泡的高度
            bubbleMargin: 10, // 气泡之间的间距
        };
    },
    methods: {
        test() {
            this.addBubble("test")
        },
        addBubble(ctx) {
            const newBubble = {
                content: marked.parse(ctx), // 富文本内容
            };
            console.log(newBubble.content)
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
<template>
    <div class="bubble-container">
        <div v-for="(bubble, index) in bubbles" :key="index" :style="getBubbleStyle(index)" class="bubble macos-background lightshadow"
            v-html="bubble.content"></div>
        <button @click="test" class="lightshadow lightbutton">添加气泡</button>
    </div>
</template>
<style>
.bubble-container {
    position: relative;
    height: 400px;
    /* 容器高度 */
    overflow: hidden;
}

.bubble {
    white-space: normal;
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    width: 200px;
    padding: 10px;
    color: rgb(0, 0, 0);
    border-radius: 10px;
    text-align: center;
    transition: opacity 0.5s ease-in-out;
}

.language-javascript {
    white-space: normal;
}
p {
    white-space: normal;
}
</style>