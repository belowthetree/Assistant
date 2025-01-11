<template>
    <div class="app">
        <bubbles ref="bubbles"></Bubbles>
    </div>
</template>

<script>
import Bubbles from '~/src/components/Bubbles.vue';
import { listenModelResultEvent } from '~/src/events/model_event';

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
        listenModelResultEvent(this.onAddBubble)
    }
};
</script>

<style scoped>
.app {
    text-align: center;
    margin-top: 50px;
}
</style>