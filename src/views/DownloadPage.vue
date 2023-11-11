<template>
  <div>
    <QQNTInstall @cancelQQNTInstall="cancelQQNTInstall" v-if="isOpenInstallQQNTLink"/>
    <normal-content>
      <h4>云崽</h4>
      <a-space direction="horizontal">
        <a-button @click="downloadRedis">Redis</a-button>
        <a-button @click="downloadMiaoYunzai">喵喵云崽</a-button>
      </a-space>
    </normal-content>
    <normal-content>
      <h4>云崽相关</h4>
      <a-space direction="horizontal">
        <a-button @click="installQQNTLink">辅助安装QQNT消息链接模块</a-button>
      </a-space>
    </normal-content>
  </div>
</template>
<script setup lang="ts">

import NormalContent from "@/component/NormalContent.vue"
import {message} from 'ant-design-vue';
import {getAppDir, getYunzaiDir} from "@/entity/hyzlPath.ts";
import QQNTInstall from "@/component/QQNTInstall.vue";
import {ref} from "vue";
import fastCommand from "@/utils/fastCommand.ts";
import {exists} from "@tauri-apps/api/fs";
import {invoke} from "@tauri-apps/api";
import {join} from "@tauri-apps/api/path";

async function downloadRedis() {
  // 检查是否已经下载Redis
  if (await exists(await join(await getAppDir(), 'redis-windows-7.0.4'))) {
    message.info('已经下载过Redis了');
    return
  }
  // 下载Redis
  message.loading({content: '正在下载Redis...', key: 'downloadRedis', duration: 0});
  const command = fastCommand('git clone --depth=1 https://gitee.com/bling_yshs/redis-windows-7.0.4', await getAppDir(), true);
  await command.execute();
  message.success({content: '下载成功', key: 'downloadRedis', duration: 3});
}

async function downloadMiaoYunzai() {
  // 检查是否已经下载喵喵云崽
  if (await exists(await getYunzaiDir())) {
    message.info('已经下载过喵喵云崽了');
    return
  }
  //定义全局提示key
  let downloadKey = 'downloadMiaoYunzai'
  // 检查是否已经下载 pnpm
  let command1 = fastCommand('pnpm -v');
  let res = await command1.execute();
  if (res.code != 0) {
    message.loading({content: '正在下载pnpm...', key: downloadKey, duration: 0});
    await fastCommand('npm install pnpm -g --registry=https://registry.npmmirror.com').execute()
    message.loading({content: '正在设置镜像源...', key: downloadKey, duration: 0});
    await fastCommand('npm config set registry https://registry.npmmirror.com').execute()
    await fastCommand('pnpm config set registry https://registry.npmmirror.com').execute()
  }
  // 下载喵喵云崽
  message.loading({content: '正在下载喵喵云崽...', key: downloadKey, duration: 0});
  const command2 = fastCommand('git clone --depth=1 https://gitee.com/yoimiya-kokomi/Miao-Yunzai.git', await getAppDir(), true);
  await command2.execute();
  // 安装依赖
  message.loading({content: '正在安装依赖...', key: downloadKey, duration: 0});
  const command3 = fastCommand('pnpm install', await getYunzaiDir(), true);
  await command3.execute();
  // 安装喵喵插件
  message.loading({content: '正在安装喵喵插件...', key: downloadKey, duration: 0});
  const command4 = fastCommand('git clone --depth=1 -b master https://gitee.com/yoimiya-kokomi/miao-plugin.git ./plugins/miao-plugin/', await getYunzaiDir(), true);
  await command4.execute();
  // 复制云崽文件夹/config/default_config 文件夹 到 云崽文件夹/config/config
  await invoke('copy_directory', {
    source: await join(await getYunzaiDir(), 'config/default_config'),
    destination: await join(await getYunzaiDir(), 'config/config'),
  })
  message.success({content: '下载成功', key: downloadKey, duration: 3});
}

//下载QQNT消息链接模块
let isOpenInstallQQNTLink = ref(false)

async function cancelQQNTInstall() {
  isOpenInstallQQNTLink.value = false
}

async function installQQNTLink() {
  isOpenInstallQQNTLink.value = true
}
</script>

<style scoped>


</style>