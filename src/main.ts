import { createApp } from 'vue'
import App from './App.vue'

// ant-design
import Antd from 'ant-design-vue';
import 'ant-design-vue/dist/reset.css';

const app = createApp(App);

// ant-design
app.use(Antd)

app.mount('#app')
