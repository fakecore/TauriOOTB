import { createApp } from "vue";
import "./styles.css";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from "./App.vue";
import zhCn from "element-plus/es/locale/lang/zh-cn";
import { router } from './router';

const app = createApp(App)
app.use(ElementPlus, {
    locale: zhCn,
})
app.use(router)
app.use(ElementPlus)
app.mount('#app')
