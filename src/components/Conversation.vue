<template>
    <div class="model-output-container rounded-lg">
        <div
            ref="outputContainer"
            class="output-content rounded-lg"
            :class="{ 'manual-scroll': !autoScroll }"
            @scroll="handleScroll"
            v-html="processedContent"
        ></div>
    </div>
</template>

<script>
import { library } from "@fortawesome/fontawesome-svg-core";
import { faSpinner, faCheck } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { marked } from "marked";

library.add(faSpinner, faCheck);

export default {
    components: {
        FontAwesomeIcon,
    },
    props: {
        content: {
            type: String,
            default: "",
        },
        isStreaming: {
            type: Boolean,
            default: false,
        },
        fontSize: {
            type: String,
            default: "0.7rem",
        },
    },
    data() {
        return {
            autoScroll: true,
            lastScrollPosition: 0,
        };
    },
    computed: {
        processedContent() {
            return marked.parse(this.content);
        },
    },
    watch: {
        content() {
            this.$nextTick(() => {
                if (this.autoScroll) {
                    this.scrollToBottom();
                }
            });
        },
        isStreaming(newVal) {
            if (!newVal && !this.autoScroll) {
                // 当内容停止输出且用户之前暂停了自动滚动时，恢复自动滚动
                this.autoScroll = true;
                this.scrollToBottom();
            }
        },
    },
    methods: {
        setContent(ctx) {
            this.content = ctx;
        },
        scrollToBottom() {
            const container = this.$refs.outputContainer;
            container.scrollTop = container.scrollHeight;
        },
        handleScroll() {
            const container = this.$refs.outputContainer;
            const scrollPosition = container.scrollTop;
            const containerHeight = container.clientHeight;
            const contentHeight = container.scrollHeight;

            // 判断用户是否滚动到底部
            const isAtBottom =
                scrollPosition + containerHeight >= contentHeight - 20; // 20px阈值

            if (isAtBottom) {
                this.autoScroll = true;
            } else {
                // 只有当用户主动滚动时才暂停自动滚动
                if (Math.abs(scrollPosition - this.lastScrollPosition) > 5) {
                    this.autoScroll = false;
                }
            }

            this.lastScrollPosition = scrollPosition;
        },
    },
    mounted() {
        this.scrollToBottom();
    },
};
</script>

<style scoped>
.model-output-container {
    position: relative;
    width: 100%;
    height: auto;
    margin: auto;
    border-radius: 5px;
    overflow-y: scroll;
    font-family:
        -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial,
        sans-serif;
    scrollbar-width: none; /* Firefox */
    -ms-overflow-style: none; /* IE and Edge */
}

.output-content {
    padding: 10px;
    overflow-y: scroll;
    height: auto;
    min-height: 100px;
    line-height: 1.6;
    color: #333;
    font-size: v-bind(fontSize);
    word-wrap: break-word;
    overflow-wrap: break-word;
    white-space: pre-wrap;
    scrollbar-width: none; /* Firefox */
    -ms-overflow-style: none; /* IE and Edge */
}

.output-content::-webkit-scrollbar {
    display: none; /* Chrome, Safari and Opera */
}

.output-content.manual-scroll {
    /* 当用户手动滚动时，可以添加一些视觉提示 */
    box-shadow: inset 0 -5px 10px -5px rgba(0, 0, 0, 0.1);
}

/* 富文本内容样式 */
.output-content >>> p {
    margin: 0 0 1em 0;
}

.output-content >>> code {
    background-color: #f5f5f5;
    padding: 2px 4px;
    border-radius: 3px;
    font-family:
        "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
    font-size: 0.9em;
}

.output-content >>> pre {
    background-color: #f5f5f5;
    padding: 12px;
    border-radius: 3px;
    overflow-x: auto;
    margin: 0 0 1em 0;
}

conversation .output-content >>> pre code {
    background-color: transparent;
    padding: 0;
}

.output-content >>> blockquote {
    border-left: 4px solid #ddd;
    margin: 0 0 1em 0;
    padding-left: 16px;
    color: #666;
}
</style>
