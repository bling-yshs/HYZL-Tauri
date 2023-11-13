<template>
  <Suspense> <!--全局异步挂载-->
    <a-config-provider :locale="zhCN">
      <a-app> <!--全局挂载 message-->
        <div :class="{'not-win11':notWin11}">
          <AnnouncementModal v-if="isOpenAnnouncement" @close="()=>isOpenAnnouncement=false"/>
          <UpdateModal v-if="isNeedUpdate" :manifest="latestManifest"/>
          <IndexPage/>
        </div>
      </a-app>
    </a-config-provider>
  </Suspense>
</template>

<script setup lang="ts">
import IndexPage from "./views/IndexPage.vue";
import {onMounted, ref} from "vue";
import {fetch} from '@tauri-apps/api/http';
import {exists, readTextFile, writeTextFile} from '@tauri-apps/api/fs';
import {getAnnouncementPath} from "@/entity/hyzlPath.ts";
import {checkUpdate} from '@tauri-apps/api/updater'
import {version} from '@tauri-apps/api/os';
import UpdateModal from "@/component/UpdateModal.vue";
import zhCN from 'ant-design-vue/es/locale/zh_CN';
import AnnouncementModal from "@/component/AnnouncementModal.vue";


// 公告

let isOpenAnnouncement = ref(false);

onMounted(async () => {
  await getAnnouncement()
  await checkUpdateFn()
});


// 检查更新

let isNeedUpdate = ref(false);
let latestManifest = ref<iManifest>();

interface iManifest {
  version: string,
  date: string,
  body: string
}

async function checkUpdateFn() {
  try {
    const {shouldUpdate, manifest} = await checkUpdate()
    if (shouldUpdate) {
      console.log('触发更新')
      latestManifest.value = manifest;
      isNeedUpdate.value = true;
    }
  } catch (error) {
    console.error(error)
  }
}


// 公告
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
    isOpenAnnouncement.value = true;
    return
  }
  const latestVersion = response.version;
  const localVersion = (JSON.parse(await readTextFile(await getAnnouncementPath())) as announcement).version;
  if (latestVersion <= localVersion) {
    console.log('公告已是最新')
    return
  }
  isOpenAnnouncement.value = true;
}

// mica背景
let notWin11 = ref(true);
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
