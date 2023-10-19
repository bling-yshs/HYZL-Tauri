import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
// ant-design
import Antd from 'ant-design-vue';
import 'ant-design-vue/dist/reset.css';

const app = createApp(App);

// ant-design
app.use(Antd)
// router
app.use(router)
app.mount('#app')
