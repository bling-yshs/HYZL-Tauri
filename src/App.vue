<template>
  <a-app>
    <a-float-button
      type="primary"
      tooltip="启动云崽"
      @click="startYunzai"
    >
      <template #icon>
        <right-circle-outlined/>
      </template>
    </a-float-button>
    <div :class="{'not-win11':notWin11}">
      <NormalPage/>
    </div>
  </a-app>
</template>

<script setup lang="ts">
import {RightCircleOutlined} from '@ant-design/icons-vue';
import NormalPage from "./views/IndexPage.vue";
import {onMounted, ref} from "vue";
import {Command} from '@tauri-apps/api/shell'

let notWin11 = ref(true);
import {yunzaiDir} from "./entity/hyzlPath.ts";

import {version} from '@tauri-apps/api/os';

onMounted(async () => {
  changeWhenNotWin11()
});

async function startYunzai() {
  new Command('cmd', ['/c', 'start', 'cmd', '/k', 'node', 'app'], {cwd: await yunzaiDir()}).spawn();
}

async function changeWhenNotWin11() {
  const osVersion = await version();
  Number(osVersion.split('.')[2]) > 22000 ? notWin11.value = false : notWin11.value = true
}
</script>
<style scoped>
.not-win11 {
  background-color: #fff;
  position: absolute;
  min-height: 100%;
  min-width: 100%;
}
</style>
