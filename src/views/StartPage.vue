<template>
  <div>
    <normal-content>
      <a-space direction="vertical">
        <a-row>
          <a-col :span="24">
            <a-image
              :preview="false"
              style="border-radius: 0.5rem;object-fit: cover;"
              :src="indexImage"
            />
          </a-col>
        </a-row>
        <a-space>
          <CodeOutlined/>
          <p>日志</p>
        </a-space>
        <a-textarea
          :rows="5"
          readonly
          :value="terminalText"
          placeholder="欢迎使用 HYZL-Tauri"
          id="terminal"
        >
        </a-textarea>
        <a-space style="display: flex; justify-content: space-between;">
          <a-space>
            <a-button type="primary" @click="startYunzai">启动云崽</a-button>
            <a-checkbox v-model:checked="isStartWithSignApi">同时启动签名API</a-checkbox>
            <a-tooltip title="会在命令行窗口中打开云崽">
              <a-checkbox v-model:checked="isOriginWindow">原生窗口</a-checkbox>
            </a-tooltip>
          </a-space>
          <a-space>
            <a-button>云崽日志</a-button>
            <a-button @click="killYunzaiProcess" danger>停止云崽</a-button>
          </a-space>
        </a-space>
      </a-space>
    </normal-content>
  </div>
</template>
<script setup lang="ts">
import {CodeOutlined} from '@ant-design/icons-vue';
import NormalContent from "@/component/NormalContent.vue"
import indexImage from "@/assets/index-iamge.jpg";
import {ref} from "vue";
import {Child, Command} from "@tauri-apps/api/shell";
import {getYunzaiDir} from "@/entity/hyzlPath.ts";
import checkProcessExist from "@/utils/checkProcessExist.ts";
import {message} from "ant-design-vue";

let tempYunzaiProcess: Child = new Child(31113);
let terminalText = ref('')
let isStartWithSignApi = ref(false)
let isOriginWindow = ref(false)

async function startYunzai() {
  const yunzai = new Command('node', 'app', {cwd: await getYunzaiDir()});
  yunzai.stdout.on('data', (data) => {
    terminalText.value += data;
  })
  tempYunzaiProcess = await yunzai.spawn();
}


async function killYunzaiProcess() {
  if (await checkProcessExist(tempYunzaiProcess.pid)) {
    tempYunzaiProcess.kill();
    message.success({content: '成功停止云崽', duration: 2})
  } else {
    message.error({content: '当前没有正在运行的云崽', duration: 2})
  }
}
</script>
<style scoped>
/* 下面我们会解释这些 class 是做什么的 */
.v-enter-active,
.v-leave-active {
  transition: opacity 0.5s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>