<template>
  <div>
    <QQNTInstall @cancelQQNTInstall="cancelQQNTInstall" v-if="isOpenInstallQQNTLink"/>
    <normal-content>
      <h4>云崽</h4>
      <a-space direction="horizontal">
        <a-button type="default" @click="downloadMiaoYunzai">喵喵云崽</a-button>
      </a-space>
    </normal-content>
    <normal-content>
      <h4>云崽相关</h4>
      <a-space direction="horizontal">
        <a-button type="default" @click="installQQNTLink">辅助安装QQNT消息链接模块</a-button>
      </a-space>
    </normal-content>
  </div>
</template>
<script setup lang="ts">

import NormalContent from "@/component/NormalContent.vue"
import {message} from 'ant-design-vue';
import {Command} from "@tauri-apps/api/shell";
import {getAppDir} from "@/entity/hyzlPath.ts";
import QQNTInstall from "@/component/QQNTInstall.vue";
import {ref} from "vue";

async function downloadMiaoYunzai() {
  let downloadKey = '下载喵喵云崽'
  message.loading({content: '正在下载喵喵云崽', key: downloadKey, duration: 0});
  const command = new Command('git', ['clone', '--depth', '1', 'https://gitee.com/yoimiya-kokomi/Miao-Yunzai.git'], {cwd: await getAppDir()});
  command.on('close', (code) => {
    if (code.code === 128) {
      message.error({content: '喵喵云崽文件夹已存在', key: downloadKey, duration: 2});
    }
    if (code.code === 0) {
      message.success({content: '下载喵喵云崽成功', key: downloadKey, duration: 2})
    }
  })
  command.spawn();
}
//下载QQNT消息链接模块
let isOpenInstallQQNTLink = ref(false)
async function cancelQQNTInstall(){
  isOpenInstallQQNTLink.value = false
}

async function installQQNTLink(){
  isOpenInstallQQNTLink.value = true
}
</script>

<style scoped>


</style>