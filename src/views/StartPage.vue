<template>
  <div>
    <context-holder/>
    <normal-content>
      <a-progress v-if="isDownloading" :percent="downloadPercent" size="300"/>
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
import {createDir} from '@tauri-apps/api/fs';
import {invoke} from "@tauri-apps/api/tauri";
import {DataResponse} from "../entity/response.ts";
import {download} from "tauri-plugin-upload-api";
import {ref} from "vue";

const [messageApi, contextHolder] = message.useMessage();

let downloadPercent = ref(0)
let isDownloading = ref(false)
//下载测试

import {appDataDir} from '@tauri-apps/api/path';
import {path} from "@tauri-apps/api";

const downloadTest = async () => {
  const appDataDirPath = await appDataDir();
  await createDir(appDataDirPath, {recursive: true})
  const svgPath = await path.join(appDataDirPath, 't01.svg');
  isDownloading.value = true
  downloadPercent.value = 0
  await download(
    "https://tauri.app/meta/tauri_logo_light.svg",
    svgPath,
    (progress, total) => {
      downloadPercent.value += progress / total * 100
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