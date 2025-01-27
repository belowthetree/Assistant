<template>
    <div class="app">
        <bubbles ref="bubbles"></Bubbles>
    </div>
</template>

<script>
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { moveWindow, Position } from '@tauri-apps/plugin-positioner';
import Bubbles from '~/src/components/Bubbles.vue';
import { listenModelResultEvent } from '~/src/events/model_event';
import { emitTalkViewQueryEvent } from '~/src/events/window_event';

export default {
    components: {
        'bubbles': Bubbles,
    },
    data() {
        return {
            message: 'Hello, Vue!'
        };
    },
    setup() {
        moveWindow(Position.BottomRight);
    },
    methods: {
        onAddBubble(event) {
            console.log("收到")
            console.log(this.$refs.bubbles)
            this.$refs.bubbles.addBubble(event.payload.ctx)
            console.log(this.$refs.bubbles)
        }
    },
    created() {
        console.log("listen")
        listenModelResultEvent(this.onAddBubble)
        emitTalkViewQueryEvent()
    }
};
</script>

<style scoped>
.app {
    text-align: center;
    margin-top: 50px;
    background: transparent;
    background-color: rgba(240, 248, 255, 0);
}
</style>