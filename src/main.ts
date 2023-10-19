import { createApp } from 'vue'
import App from './App.vue'
import css from './assets/main.css'
import router from './router'
// ant-design
import Antd from 'ant-design-vue';
import 'ant-design-vue/dist/reset.css';

const app = createApp(App);
app.use(css)
// ant-design
app.use(Antd)
// router
app.use(router)
app.mount('#app')
