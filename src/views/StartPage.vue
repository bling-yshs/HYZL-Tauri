<template>
  <div>
    <context-holder/>
    <normal-content>
      <a-progress v-if="isDownloading" :percent="percent" size="300" :show-info="false"/>
      <a-space direction="vertical">
        <a-space direction="horizontal">
          <a-button type="primary" @click="startWithApi">启动云崽&签名API</a-button>
          <a-button type="default" @click="downloadTest">
            Test
          </a-button>
        </a-space>
      </a-space>
    </normal-content>
  </div>
</template>
<script setup lang="ts">
import NormalContent from "./NormalContent.vue";
import {message} from 'ant-design-vue';

const [messageApi, contextHolder] = message.useMessage();
import {invoke} from "@tauri-apps/api/tauri";
import {DataResponse} from "../entity/response.ts";
import {download} from "tauri-plugin-upload-api";
import {ref} from "vue";

let percent = ref(0)
let isDownloading = ref(false)
//下载测试
const downloadTest = async () => {
  isDownloading.value = true
  percent.value = 0
  await download(
    "https://tauri.app/meta/tauri_logo_light.svg",
    "./avbddd.svg",
    (progress, total) => {
      percent.value += progress / total * 100
      console.log(progress / total * 100)
      console.log(`Downloaded ${progress} of ${total} bytes`)
    },
  );
  setTimeout(() => {
    isDownloading.value = false
  }, 2000)
}


// 启动云崽 & 签名API
const startWithApi = async () => {
  const response = await invoke('start_yunzai_and_api');
  const res = JSON.parse(response as string) as DataResponse
  if (res.code === 200) {
    messageApi.success(res.message)
    return
  } else {
    messageApi.error('启动失败')
    return
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