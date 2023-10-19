<template>
  <NormalContent>
    <a-space direction="vertical">
      <a-space direction="horizontal">
        <a-button type="primary" @click="startWithApi">启动云崽&签名API</a-button>
      </a-space>
    </a-space>
  </NormalContent>
</template>
<script setup lang="ts">
import NormalContent from "./NormalContent.vue";
import {message} from "ant-design-vue";
import {invoke} from "@tauri-apps/api/tauri";
import {DataResponse} from "../entity/response.ts";
// 启动云崽&签名API
const startWithApi = async () => {
  const response = await invoke('start_yunzai_and_api');
  const res = JSON.parse(response as string) as DataResponse
  if (res.code === 200) {
    message.success(res.message)
    message.success('启动成功')
    return
  } else {
    message.error('启动失败')
    return
  }
  
}
</script>
<style scoped>
</style>