import { createApp } from "vue";
// import App from "./App.vue";
import App from "./Empty.vue"
import { registerWindowEvents } from "./events/window_event";
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import 'element-plus/dist/index.css'
import { loadConfig } from "./config";
import router from "./router";
import { library } from '@fortawesome/fontawesome-svg-core'
import { faCog } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { ElementPlus } from "@element-plus/icons-vue";

library.add(faCog) // 添加 faCog 图标

const app = createApp(App)
app.component('FontAwesomeIcon', FontAwesomeIcon) // 注册组件
for (const [key, com] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, com)
}
app.use(router)
app.use(ElementPlus.plugins)
app.mount("#app")

registerWindowEvents()

loadConfig()
