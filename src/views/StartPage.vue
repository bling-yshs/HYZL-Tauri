<template>
  <NormalContent>
    <a-space direction="vertical">
      <a-space direction="horizontal">
        <a-button type="primary" @click="startWithApi">启动云崽&签名API</a-button>
        <a-button type="default" @click="downloadTest">下载测试</a-button>
      </a-space>
    </a-space>
  </NormalContent>
</template>
<script setup lang="ts">
import NormalContent from "./NormalContent.vue";
import {message} from "ant-design-vue";
import {invoke} from "@tauri-apps/api/tauri";
import {DataResponse} from "../entity/response.ts";
import {getClient, ResponseType} from "@tauri-apps/api/http";
import * as fs from "@tauri-apps/api/fs";

//下载测试
const downloadTest = async () => {
  const url = 'https://gitee.com/bling_yshs/YzLauncher-windows/releases/download/v0.1.58/yzMD5.txt'
  const client = await getClient();
  let response = await client.get(url, {
    responseType: ResponseType.Binary
  });
  await fs.writeBinaryFile(
    'a.txt',
    response.data as fs.BinaryFileContents
  )
  message.success("yes")
}


// 启动云崽&签名API
const startWithApi = async () => {
  const response = await invoke('start_yunzai_and_api');
  const res = JSON.parse(response as string) as DataResponse
  if (res.code === 200) {
    message.success(res.message)
    return
  } else {
    message.error('启动失败')
    return
  }
}
</script>
<style scoped>
</style>