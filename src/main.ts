import { createApp } from "vue";
// import App from "./App.vue";
import App from "./AssistantMainView.vue"
import { registerWindowEvents } from "./events/window_event";
import { moveWindow, Position } from "@tauri-apps/plugin-positioner";

const app = createApp(App)
app.mount("#app");

moveWindow(Position.BottomRight)

registerWindowEvents()