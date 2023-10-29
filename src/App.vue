<template>
  <Suspense> <!--全局异步挂载-->
    <a-app> <!--全局挂载 message-->
      <div :class="{'not-win11':notWin11}">
        <IndexPage/>
      </div>
    </a-app>
  </Suspense>
</template>

<script setup lang="ts">
import IndexPage from "./views/IndexPage.vue";
import {onMounted, ref} from "vue";
import {fetch} from '@tauri-apps/api/http';
import {writeTextFile, exists, readTextFile} from '@tauri-apps/api/fs';
import {getAnnouncementPath} from "@/entity/hyzlPath.ts";
import {Modal} from 'ant-design-vue';
import { appWindow } from '@tauri-apps/api/window';

// 公告
onMounted(async () => {
  await getAnnouncement()
  await appWindow.setDecorations(true);
});
interface announcement {
  version: number,
  content: string
}
async function getAnnouncement() {
  const response = (await fetch('https://gist.githubusercontent.com/bling-yshs/70898cb0d69bef4c16cf7823a1a767b5/raw/', {
    method: 'GET',
    headers: {
      'Cache-Control': 'no-cache'
    }
  })).data as announcement;
  if (!await exists(await getAnnouncementPath())) {
    await writeTextFile(await getAnnouncementPath(), JSON.stringify(response));
    console.log('初始化公告文件成功')
  } else {
    //读取文件，比较版本号
    const latestVersion = response.version;
    const localData = (JSON.parse(await readTextFile(await getAnnouncementPath())) as announcement).version;
    if (latestVersion <= localData) {
      console.log('公告已是最新')
      return
    }
  }
  // 展示公告
  Modal.info({
    title: '发现新公告',
    content: response.content,
    okText: '我知道了',
  });
  await writeTextFile(await getAnnouncementPath(), JSON.stringify(response));
}

// mica背景
let notWin11 = ref(true);
import {version} from '@tauri-apps/api/os';
onMounted(async () => {
  changeWhenNotWin11()
});
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
