<template>
  <div>
    <a-modal v-model:open="isOpen" title="公告" @ok="handleOk"
             :after-close="()=>emit('close')"
    >
      <span>{{ announcementContext }}</span>
    </a-modal>
  </div>
</template>


<script setup lang="ts">
import {onMounted, ref} from "vue";
import {fetch} from "@tauri-apps/api/http";
import {readTextFile, writeTextFile} from '@tauri-apps/api/fs';
import {getAnnouncementPath} from "@/entity/hyzlPath.ts";

let isOpen = ref(false);
let announcementContext = ref('');
// 接收父组件传递过来的值
let props = defineProps({
  needUpdate: Boolean
});
// 关闭后传递给父组件
let emit = defineEmits(['close'])

async function handleOk() {
  isOpen.value = false;
}

// 公告
interface announcement {
  version: number,
  content: string
}

onMounted(async () => {
  if (props.needUpdate) {
    console.log('需要更新')
    const response = (await fetch('https://gist.githubusercontent.com/bling-yshs/70898cb0d69bef4c16cf7823a1a767b5/raw/', {
      method: 'GET',
      headers: {
        'Cache-Control': 'no-cache'
      }
    })).data as announcement;
    announcementContext.value = response.content;
    await writeTextFile(await getAnnouncementPath(), JSON.stringify(response));
  } else {
    console.log('不需要更新')
    const response = JSON.parse(await readTextFile(await getAnnouncementPath())) as announcement;
    announcementContext.value = response.content;
  }
  isOpen.value = true;
})

</script>


<style scoped>

</style>