<template>
    <transition name="slide-down">
        <div v-if="visible" class="toast macosfont" :style="styleColor">
            {{ message }}
        </div>
    </transition>
</template>

<script>
export default {
    data() {
        return {
            visible: false,
            message: '',
            timeout: null,
            color: "#E6A23C"
        };
    },
    computed: {
        styleColor() {
            return {
                backgroundColor: this.color
            }
        }
    },
    methods: {
        show(message, duration = 2000, color = "#67C23A") {
            this.message = message;
            this.visible = true;
            this.color = color

            if (this.timeout) {
                clearTimeout(this.timeout);
            }

            this.timeout = setTimeout(() => {
                this.visible = false;
            }, duration);
        },
        warning(message, duration = 2000) {
            this.show(message, duration, "#E6A23C")
        },
        danger(message, duration = 2000) {
            this.show(message, duration, "#F56C6C")
        }
    }
};
</script>

<style scoped>
.toast {
    position: fixed;
    top: 20px;
    left: 50%;
    transform: translateX(-50%);
    padding: 10px 20px;
    background-color: #333;
    color: #fff;
    border-radius: 4px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    z-index: 1000;
    font-size: 14px;
    text-align: center;
}

/* 入场和离场动画 */
.slide-down-enter-active,
.slide-down-leave-active {
    transition: all 0.5s ease;
}

.slide-down-enter-from,
.slide-down-leave-to {
    opacity: 0;
    transform: translate(-50%, -100%);
    /* 从上方进入 */
}
</style>