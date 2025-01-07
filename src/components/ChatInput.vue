<template>
    <div class="chat-input-container">
        <!-- 输入框和按钮的组合 -->
        <div class="input-wrapper" :class="theme">
            <!-- 文本输入框 -->
            <textarea v-model="message" @keydown.enter.exact.prevent="sendMessage" placeholder="Type a message..."
                class="chat-input"></textarea>

            <!-- 表情按钮 -->
            <button @click="toggleEmojiPicker" class="icon-button">
                😀
            </button>

            <!-- 文件上传按钮 -->
            <button @click="triggerFileUpload" class="icon-button">
                📎
            </button>
        </div>

        <!-- 表情选择器 -->
        <div v-if="showEmojiPicker" class="emoji-picker">
            <span v-for="emoji in emojis" :key="emoji" @click="addEmoji(emoji)">
                {{ emoji }}
            </span>
        </div>

        <!-- 文件上传输入 -->
        <input type="file" ref="fileInput" @change="handleFileUpload" style="display: none;" />

        <!-- 发送按钮 -->
        <button @click="sendMessage" class="send-button">
            Send
        </button>
    </div>
</template>

<script>
export default {
    props: {
        theme: {
            type: String,
            default: 'light', // 默认主题为 light
            validator(value) {
                return ['light', 'dark'].includes(value); // 支持 light 和 dark 主题
            },
        },
    },
    data() {
        return {
            message: '',
            showEmojiPicker: false,
            emojis: ['😀', '😎', '😍', '🤔', '🎉', '👍', '👋', '❤️', '🔥', '🚀'],
            file: null,
        };
    },
    methods: {
        sendMessage() {
            if (this.message.trim() || this.file) {
                this.$emit('send', {
                    message: this.message,
                    file: this.file,
                });
                this.message = '';
                this.file = null;
                this.$refs.fileInput.value = ''; // 清空文件输入
            }
        },
        toggleEmojiPicker() {
            this.showEmojiPicker = !this.showEmojiPicker;
        },
        addEmoji(emoji) {
            this.message += emoji;
            this.showEmojiPicker = false;
        },
        triggerFileUpload() {
            this.$refs.fileInput.click();
        },
        handleFileUpload(event) {
            this.file = event.target.files[0];
        },
    },
};
</script>

<style scoped>
.chat-input-container {
    display: flex;
    align-items: center;
    padding: 10px;
    border-top: 1px solid #ccc;
    background-color: var(--background-color, #f9f9f9);
    /* 使用 CSS 变量 */
}

.input-wrapper {
    flex: 1;
    display: flex;
    align-items: center;
    border: 1px solid var(--border-color, #ccc);
    /* 使用 CSS 变量 */
    border-radius: 20px;
    /* 圆角 */
    padding: 5px 10px;
    background-color: var(--input-background, white);
    /* 使用 CSS 变量 */
}

/* 浅色主题 */
.input-wrapper.light {
    --background-color: #f9f9f9;
    --border-color: #ccc;
    --input-background: white;
}

/* 深色主题 */
.input-wrapper.dark {
    --background-color: #333;
    --border-color: #555;
    --input-background: #444;
    color: white;
    /* 文字颜色 */
}

.chat-input {
    flex: 1;
    border: none;
    outline: none;
    resize: none;
    padding: 8px;
    font-size: 16px;
    background-color: transparent;
    color: inherit;
    /* 继承父级颜色 */
}

.icon-button {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 20px;
    margin: 0 5px;
    color: var(--icon-color, #666);
    /* 使用 CSS 变量 */
}

.icon-button:hover {
    color: var(--icon-hover-color, #333);
    /* 使用 CSS 变量 */
}

.send-button {
    background-color: var(--send-button-bg, #007bff);
    /* 使用 CSS 变量 */
    color: white;
    border: none;
    border-radius: 20px;
    padding: 10px 20px;
    margin-left: 10px;
    cursor: pointer;
    font-size: 16px;
}

.send-button:hover {
    background-color: var(--send-button-hover-bg, #0056b3);
    /* 使用 CSS 变量 */
}

.emoji-picker {
    position: absolute;
    bottom: 60px;
    right: 10px;
    background: var(--emoji-picker-bg, white);
    /* 使用 CSS 变量 */
    border: 1px solid var(--border-color, #ccc);
    /* 使用 CSS 变量 */
    padding: 10px;
    border-radius: 10px;
    display: flex;
    flex-wrap: wrap;
    width: 150px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.emoji-picker span {
    cursor: pointer;
    margin: 5px;
}
</style>