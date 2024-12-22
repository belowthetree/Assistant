import { createApp } from "vue";
// import App from "./App.vue";
import App from "./AssistantMainView.vue"
import { registerWindowEvents } from "./events/window_event";

createApp(App).mount("#app");

registerWindowEvents()