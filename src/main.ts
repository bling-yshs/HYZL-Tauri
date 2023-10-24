import {createApp} from 'vue'
import App from './App.vue'

const app = createApp(App);

// ant-design
import 'ant-design-vue/dist/reset.css';
import Antd from 'ant-design-vue';

app.use(Antd)

// router
import router from './router'

app.use(router)

// 项目初始化
import './assets/main.css'
import init from "./init/init.ts";

init()

app.mount('#app')