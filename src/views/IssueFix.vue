<template>
  <NormalContent>
    <a-space direction="vertical">
      <a-space direction="horizontal">
        <a-button type="default" @click="reinstallDependence">重装依赖</a-button>
      </a-space>
    </a-space>
  </NormalContent>
</template>
<script setup lang="ts">
import NormalContent from "./NormalContent.vue";
import {message} from "ant-design-vue";
import {invoke} from "@tauri-apps/api/tauri";
import {DataResponse} from "../entity/response.ts";

const reinstallDependence = async () => {
  const key = '重装依赖'
  message.loading({content: '正在重装依赖', key, duration: 0});
  const response = await invoke('reinstall_dependence');
  const res = JSON.parse(response as string) as DataResponse
  if (res.code === 200) {
    message.success({content: res.message, key, duration: 2});
    return
  } else {
    message.error({content: res.message, key, duration: 2});
    return
  }
  
}
</script>

<style scoped>


</style>