<template>
  <div>
    <a-modal :title="'发现新版本: v'+props.manifest.version" @ok="handleOk" v-model:open="isOpen" ok-text="立即更新"
             :after-close="()=>emit('close')"
             cancel-text="下次再说" @cancel="handleCancel" :maskClosable="false">
      <pre style="white-space: pre-wrap">{{ changelog }}<span
        v-if="showMore"
        @click="()=>{changelog=props.manifest.body;showMore=false}"
        style="color: #247fff">展开剩余的{{remainLog}}行
        </span></pre>
    </a-modal>
  </div>
</template>
<script lang="ts" setup>
import {installUpdate} from '@tauri-apps/api/updater'
import {onMounted, ref} from "vue";
import {relaunch} from '@tauri-apps/api/process'
import {message} from "ant-design-vue";

let showMore = ref(false)
let isOpen = ref(true);
// 接收父组件传递过来的值
let props = defineProps(['manifest']);
// 关闭后传递给父组件
let emit = defineEmits(['close'])
// 统计剩余的更新日志行数
let remainLog = ref(0)

onMounted(async () => {
  let arr = props.manifest.body.split('\n') as Array<string>
  if (arr.length > 5) {
    remainLog.value = arr.length - 5
    arr.splice(5)
    arr[4] += ' ...'
    showMore.value = true
  }
  let res = arr.join('\n')
  changelog.value = res
})

let changelog = ref('')

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

