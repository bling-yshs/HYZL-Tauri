<template>
  <div>
    <UpdateModal v-if="isNeedUpdate" :manifest="latestManifest"/>
    <normal-content>
      <a-space direction="vertical">
        <span>启动器</span>
        <a-space>
          <a-button @click="checkLauncherUpdate">检查更新</a-button>
        </a-space>
      </a-space>
    </normal-content>
  </div>
</template>
<script setup lang="ts">
import NormalContent from "@/component/NormalContent.vue"
import {checkUpdate} from '@tauri-apps/api/updater'
import {ref} from "vue";
import UpdateModal from "@/component/UpdateModal.vue";
import {message} from "ant-design-vue";


interface iManifest {
  version: string,
  date: string,
  body: string
}
let latestManifest = ref<iManifest>();
let isNeedUpdate = ref(false);

async function checkLauncherUpdate() {
  try {
    const {shouldUpdate, manifest} = await checkUpdate()
    if (shouldUpdate) {
      console.log('触发更新')
      latestManifest.value = manifest;
      isNeedUpdate.value = true;
    }else {
      message.info('已经是最新版本啦~')
    }
  } catch (error) {
    console.error(error)
  }
}


</script>

<style scoped>


</style>