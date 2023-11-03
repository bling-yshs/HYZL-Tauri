<template>
  <div>
    <a-modal :title="'发现新版本: v'+props.manifest.version" @ok="handleOk" :open="isOpen" ok-text="立即更新"
             cancel-text="下次再说" @cancel="handleCancel" :maskClosable="false">
      <h3>📋 更新内容:</h3>
      <pre>{{ changelog }}</pre>
    </a-modal>
  </div>
</template>
<script lang="ts" setup>
import {installUpdate,} from '@tauri-apps/api/updater'
import {computed, ref} from "vue";
import {relaunch} from '@tauri-apps/api/process'
import {message} from "ant-design-vue";

let isOpen = ref(true);
// 接收父组件传递过来的值
let props = defineProps(['manifest']);

const changelog = computed(() => {
let arr = props.manifest.body.split('\n') as Array<string>
  if (arr.length > 5) {
    arr.splice(5)
    arr[4] += ' ...'
  }
  let res = arr.join('\n')
  return res
})
const handleOk = async () => {
  message.info({
    content: '正在下载更新包...',
    duration: 0,
    key: 'update',
  });
  await installUpdate()
  message.info({
    content: '更新完成，正在重启...',
    duration: 0,
    key: 'update',
  });
  await relaunch()
  isOpen.value = false;
};
const handleCancel = () => {
  isOpen.value = false;
};
</script>

