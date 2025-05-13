<template>
  <div class="model-output-container">
    <div
      ref="outputContainer"
      class="output-content"
      :class="{ 'manual-scroll': !autoScroll }"
      @scroll="handleScroll"
      v-html="processedContent"
    ></div>
    
    <div class="status-indicator" :class="{ 'loading': isStreaming }">
      <font-awesome-icon 
        :icon="isStreaming ? 'spinner' : 'check'" 
        :spin="isStreaming"
      />
    </div>
  </div>
</template>

<script>
import { library } from '@fortawesome/fontawesome-svg-core'
import { faSpinner, faCheck } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

library.add(faSpinner, faCheck)

export default {
  components: {
    FontAwesomeIcon
  },
  props: {
    content: {
      type: String,
      default: ''
    },
    isStreaming: {
      type: Boolean,
      default: false
    }
  },
  data() {
    return {
      autoScroll: true,
      lastScrollPosition: 0
    }
  },
  computed: {
    processedContent() {
      // 这里可以添加富文本处理逻辑，如Markdown转HTML等
      return this.content
    }
  },
  watch: {
    content() {
      this.$nextTick(() => {
        if (this.autoScroll) {
          this.scrollToBottom()
        }
      })
    },
    isStreaming(newVal) {
      if (!newVal && !this.autoScroll) {
        // 当内容停止输出且用户之前暂停了自动滚动时，恢复自动滚动
        this.autoScroll = true
        this.scrollToBottom()
      }
    }
  },
  methods: {
    scrollToBottom() {
      const container = this.$refs.outputContainer
      container.scrollTop = container.scrollHeight
    },
    handleScroll() {
      const container = this.$refs.outputContainer
      const scrollPosition = container.scrollTop
      const containerHeight = container.clientHeight
      const contentHeight = container.scrollHeight
      
      // 判断用户是否滚动到底部
      const isAtBottom = scrollPosition + containerHeight >= contentHeight - 20 // 20px阈值
      
      if (isAtBottom) {
        this.autoScroll = true
      } else {
        // 只有当用户主动滚动时才暂停自动滚动
        if (Math.abs(scrollPosition - this.lastScrollPosition) > 5) {
          this.autoScroll = false
        }
      }
      
      this.lastScrollPosition = scrollPosition
    }
  },
  mounted() {
    this.scrollToBottom()
  }
}
</script>

<style scoped>
.model-output-container {
  position: relative;
  width: 800px; /* 固定宽度 */
  max-height: 600px; /* 最大高度 */
  margin: 0 auto;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  overflow: hidden;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
}

.output-content {
  padding: 16px;
  overflow-y: auto;
  height: auto;
  min-height: 200px;
  max-height: 600px;
  line-height: 1.6;
  color: #333;
}

.output-content.manual-scroll {
  /* 当用户手动滚动时，可以添加一些视觉提示 */
  box-shadow: inset 0 -5px 10px -5px rgba(0, 0, 0, 0.1);
}

.status-indicator {
  position: absolute;
  right: 16px;
  bottom: 16px;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background-color: #f5f5f5;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
  transition: all 0.3s ease;
}

.status-indicator.loading {
  background-color: #e3f2fd;
  color: #1976d2;
}

.status-indicator:not(.loading) {
  background-color: #e8f5e9;
  color: #388e3c;
}

/* 富文本内容样式 */
.output-content >>> p {
  margin: 0 0 1em 0;
}

.output-content >>> code {
  background-color: #f5f5f5;
  padding: 2px 4px;
  border-radius: 3px;
  font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
  font-size: 0.9em;
}

.output-content >>> pre {
  background-color: #f5f5f5;
  padding: 12px;
  border-radius: 3px;
  overflow-x: auto;
  margin: 0 0 1em 0;
}

.output-content >>> pre code {
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