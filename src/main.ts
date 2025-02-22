import { createApp } from "vue";
// import App from "./App.vue";
import App from "./Empty.vue"
import { registerWindowEvents } from "./events/window_event";
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import { loadConfig } from "./config";
import router from "./router";

const app = createApp(App)
for (const [key, com] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, com)
}
app.use(router)
app.use(ElementPlus)
app.mount("#app")

registerWindowEvents()

loadConfig()

const t = await import("/Users/zgg/Documents/project/test.ts")
console.log(t)