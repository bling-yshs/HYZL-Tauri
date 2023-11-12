<template>
  <div>
    <a-modal title="辅助安装QQNT消息链接模块" v-model:open="openStep.step0"
             ok-text="现在开始" @ok="okStep0"
             cancel-text="下次再说"
             :maskClosable="false"
             :after-close="afterClose">
      <span>即将开始辅助安装QQNT消息链接模块，由于能力有限，所以可能部分设置需要大家配合</span>
    </a-modal>
    
    <a-modal title="辅助安装QQNT消息链接模块" v-model:open="openStep.step1"
             ok-text="下一步" @ok="okStep1" :ok-button-props="disableOkStep1"
             cancel-text="取消安装" :maskClosable="false"
             :after-close="afterClose">
      <a-space class="start-end">
        <span style="font-size: 0.9rem">第一步：通过 Git 下载所需材料</span>
        <a-button type="primary" @click="downloadGitFiles">
          开始下载
        </a-button>
      </a-space>
    </a-modal>
    
    <a-modal v-if="openStep.step2"
             title="辅助安装QQNT消息链接模块" v-model:open="openStep.step2"
             ok-text="下一步" @ok="okStep2"
             cancel-text="取消安装" :maskClosable="false"
             :after-close="afterClose">
      <a-space class="start-end">
        <a-space direction="vertical">
          <a-typography-paragraph style="margin: 0">
            第二步：下载并安装 9.9.2.16183 版本的 QQNT
          </a-typography-paragraph>
          <a-typography-text strong>(安装完成后请暂时
            <a-typography-text mark>不要启动</a-typography-text>
            QQNT)
          </a-typography-text>
        </a-space>
        <a-button type="primary" @click="downloadQQNT" :loading="loadingDownloadQQNT">
          下载并安装
        </a-button>
      </a-space>
      <a-progress v-if="isDownloadingQQNT" :percent="downloadQQNTPercent"/>
    </a-modal>
    
    <a-modal title="辅助安装QQNT消息链接模块" v-model:open="openStep.step3"
             ok-text="下一步" @ok="okStep3"
             cancel-text="取消安装" :maskClosable="false"
             :after-close="afterClose">
      <a-space class="start-end">
        <a-typography-paragraph style="margin: 0">
          第三步：安装VC运行库
        </a-typography-paragraph>
        <a-button type="primary" @click="installVC">
          开始安装
        </a-button>
      </a-space>
    </a-modal>
    
    <a-modal title="辅助安装QQNT消息链接模块" v-model:open="openStep.step4"
             ok-text="下一步" @ok="okStep4"
             cancel-text="取消安装" :maskClosable="false"
             :confirm-loading="QQNTConfirmLoading"
             :after-close="afterClose"
    >
      <a-space direction="vertical">
        <a-typography-paragraph style="margin: 0">第四步：输入 QQNT 文件夹路径，程序的默认路径为</a-typography-paragraph>
        <a-typography-paragraph style="margin: 0" copyable>C:\Program Files\Tencent\QQNT</a-typography-paragraph>
        <a-space-compact block>
          <a-input v-model:value="QQNTDir" placeholder="请输入 QQNT 文件夹路径"/>
          <a-button type="primary" @click="pasteQQNTDir">
            <snippets-outlined/>
          </a-button>
        </a-space-compact>
      </a-space>
    </a-modal>
    
    <a-modal title="辅助安装QQNT消息链接模块" v-model:open="openStep.step5"
             ok-text="完成安装" @ok="okStep5" :ok-button-props="disableOkStep5"
             cancel-text="取消安装" :maskClosable="false"
             :after-close="afterClose">
      <a-space direction="vertical">
        <a-space>
          <span>{{ isInitializingQQNTText }}</span>
          <a-spin v-if="isInitializingQQNT" :indicator="indicator"/>
        </a-space>
      </a-space>
    </a-modal>
  
  
  </div>
</template>
<script lang="ts" setup>
import {h, onMounted, ref, watch} from "vue";
import {message} from "ant-design-vue";
import {Command} from "@tauri-apps/api/shell";
import {getAppCacheDir, getYunzaiDir} from "@/entity/hyzlPath.ts";
import {download} from "tauri-plugin-upload-api";
import {desktopDir, join} from "@tauri-apps/api/path";
import {copyFile, exists, readDir, readTextFile, writeTextFile} from "@tauri-apps/api/fs";
import {DataResponse} from "@/entity/response.ts";
import {invoke} from "@tauri-apps/api";
import {LoadingOutlined, SnippetsOutlined} from '@ant-design/icons-vue';
import {readText} from "@tauri-apps/api/clipboard";
import {dump, load} from "js-yaml";
import fastCommand from "@/utils/fastCommand.ts";


interface Step {
  step0: boolean,
  step1: boolean,
  step2: boolean,
  step3: boolean,
  step4: boolean,
  step5: boolean,
  step6: boolean,
}

const openStep = ref<Step>({
  step0: true,
  step1: false,
  step2: false,
  step3: false,
  step4: false,
  step5: false,
  step6: false,
})


async function afterClose() {
  let afterCloseAll = true
  Object.values(openStep.value).filter((v) => {
    if (v) {
      afterCloseAll = false
    }
  })
  if (afterCloseAll) {
    emits('cancelQQNTInstall')
  }
}


const emits = defineEmits(['cancelQQNTInstall']);

const yzSignPackagePath = await join(await getAppCacheDir(), 'yz-sign-package')

// 第0步
async function okStep0() {
  openStep.value.step0 = false
  openStep.value.step1 = true
}

// 第1步，下载材料
let disableOkStep1 = ref({disabled: true});

onMounted(async () => {
  if (await exists(yzSignPackagePath)) {
    disableOkStep1.value = {disabled: false}
  }
})

async function downloadGitFiles() {
  message.loading({content: '正在通过 Git 下载所需文件', duration: 0, key: 'installQQNTLink'})
  const command = fastCommand('git clone --depth=1 https://gitee.com/bling_yshs/yz-sign-package', await getAppCacheDir(), true);
  command.on('close', (code) => {
    if (code.code === 0) {
      message.success({content: '下载成功', key: 'installQQNTLink', duration: 2})
      disableOkStep1.value = {disabled: false}
    } else {
      message.error({content: '下载失败', key: 'installQQNTLink', duration: 2})
    }
  })
  command.spawn()
}

async function okStep1() {
  openStep.value.step1 = false
  openStep.value.step2 = true
}

// 第2步，下载官方QQNT
let downloadQQNTPercent = ref(0)
let isDownloadingQQNT = ref(false)
let loadingDownloadQQNT = ref(false)

async function okStep2() {
  openStep.value.step2 = false
  openStep.value.step3 = true
}

async function downloadQQNT() {
  loadingDownloadQQNT.value = true
  // 檢查是否存在QQNT.exe
  if (await exists(await join(await getAppCacheDir(), "QQNT.exe"))) {
    const sha = await invoke("calc_sha256", {path: await join(await getAppCacheDir(), "QQNT.exe")}) as DataResponse;
    if ('0b91fb920b205f41aadafa16621b0072a4a8f7eecc53fb82f46c5ee902176e6d' === sha.data.toString()) {
      message.info('QQNT已下载，正在启动安装程序...')
      loadingDownloadQQNT.value = false
      fastCommand(`start ${await join(await getAppCacheDir(), "QQNT.exe")}`).spawn()
      return
    }
  }
  isDownloadingQQNT.value = true
  let percent = 0
  await download(
    "https://dldir1.qq.com/qqfile/qq/QQNT/bef02a45/QQ9.9.2.16183_x64.exe",
    await join(await getAppCacheDir(), "QQNT.exe"),
    (progress, total) => {
      // 当前分片下载 / 总大小
      const p = (progress / total) * 100
      percent += p
      downloadQQNTPercent.value = Math.ceil(percent)
    },
  );
  const sha = await invoke("calc_sha256", {path: await join(await getAppCacheDir(), "QQNT.exe")}) as DataResponse;
  if (!('0b91fb920b205f41aadafa16621b0072a4a8f7eecc53fb82f46c5ee902176e6d' === sha.data.toString())) {
    message.error('QQNT安装包校验失败，请尝试重新下载')
    loadingDownloadQQNT.value = false
    isDownloadingQQNT.value = false
    return
  }
  fastCommand(`start ${await join(await getAppCacheDir(), "QQNT.exe")}`).spawn()
  loadingDownloadQQNT.value = false
  isDownloadingQQNT.value = false
}

// 第3步，安装VC运行库
async function okStep3() {
  openStep.value.step3 = false
  openStep.value.step4 = true
}

async function installVC() {
  fastCommand('start https://aka.ms/vs/16/release/vc_redist.x64.exe').spawn()
}

async function pasteQQNTDir() {
  let text = await readText();
  QQNTDir.value = text as string
}

// 第4步，输入QQNT文件夹路径
let QQNTDir = ref('')
let QQNTConfirmLoading = ref(false)

async function okStep4() {
  QQNTConfirmLoading.value = true
  let fileEntries = await readDir(QQNTDir.value, {recursive: true});
  if (!fileEntries.some(item => item.name === 'QQ.exe')) {
    message.error('未找到QQ.exe，请检查路径是否正确')
    QQNTConfirmLoading.value = false
    return
  }
  openStep.value.step4 = false
  openStep.value.step5 = true
}

// 第5步，进行一系列复制拷贝的任务

let disableOkStep5 = ref({disabled: true});

const indicator = h(LoadingOutlined, {
  style: {
    fontSize: '1rem',
  },
  spin: true,
});

let isInitializingQQNT = ref(true)
let isInitializingQQNTText = ref('正在自动进行一系列设置，请稍等')
watch(openStep.value, async (n) => {
  if (n.step5) {
    initQQNT()
  }
})

async function initQQNT() {
  // 复制yz-sign-package中的LiteLoaderQQNT-Launcher_x64.exe文件到QQNT主目录
  await copyFile(await join(yzSignPackagePath, 'LiteLoaderQQNT-Launcher_x64.exe'), await join(QQNTDir.value, 'LiteLoaderQQNT-Launcher_x64.exe'))
  
  // 复制yz-sign-package中的LiteLoader文件夹到 'QQNT主目录/resources/app' 下
  await invoke("copy_directory",
    {
      source: await join(yzSignPackagePath, 'LiteLoader'),
      destination: await join(QQNTDir.value, 'resources/app/LiteLoader')
    }) as DataResponse;
  
  // 复制yz-sign-package中的apps.js文件到 云崽目录 下
  await copyFile(await join(yzSignPackagePath, 'apps.js'), await join(await getYunzaiDir(), 'apps.js'))
  
  // 在云崽目录下执行 `  git clone -b red --depth=1 https://gitee.com/xiaoye12123/ws-plugin.git ./plugins/ws-plugin/`
  await fastCommand('git clone -b red --depth=1 https://gitee.com/xiaoye12123/ws-plugin.git ./plugins/ws-plugin/', await getYunzaiDir(), true).execute()
  
  // 复制 '云崽目录/plugins/ws-plugin/config/default_config' 到 '云崽目录/plugins/ws-plugin/config/config'
  await invoke("copy_directory",
    {
      source: await join(await getYunzaiDir(), 'plugins/ws-plugin/config/default_config'),
      destination: await join(await getYunzaiDir(), 'plugins/ws-plugin/config/config')
    }) as DataResponse;
  
  // 复制yz-sign-package中的ws-config.yaml文件到 '云崽目录/plugins/ws-plugin/config' 下
  await copyFile(await join(yzSignPackagePath, 'ws-config.yaml'), await join(await getYunzaiDir(), 'plugins/ws-plugin/config/config/ws-config.yaml'))
  
  
  // 修改 ws-config.yaml
  await changeWsConfig()
  
  // 安装 ws-plugin 的依赖
  await fastCommand('pnpm install', await getYunzaiDir()).execute()
  
  await createQQNTLinkToDesktop()
  
  isInitializingQQNTText.value = '初始化完成，QQNT消息链接模块已安装完毕'
  isInitializingQQNT.value = false;
  disableOkStep5.value = {disabled: false}
}

async function createQQNTLinkToDesktop() {
  // 执行 'mklink "QQNT目录/LiteLoaderQQNT-Launcher_x64.exe" "桌面/LiteLoaderQQNT-Launcher_x64.exe"'
  await new Command('ps', ['mklink', `"${await join(QQNTDir.value, 'LiteLoaderQQNT-Launcher_x64.exe')}"`, `"${await join(await desktopDir(), 'LiteLoaderQQNT-Launcher_x64.exe')}"`]).spawn();
}

async function changeWsConfig() {
  // 读取 '云崽目录/config/config/qq.yaml'
  let qqYamlContext = await readTextFile(await join(await getYunzaiDir(), 'config/config/qq.yaml'))
  let qq = (load(qqYamlContext) as any).qq as number;
  
  // 读取 '云崽目录/plugins/ws-plugin/config/ws-config.yaml'
  let wsConfigYamlContext = await readTextFile(await join(await getYunzaiDir(), 'plugins/ws-plugin/config/config/ws-config.yaml'))
  let wsConfig = load(wsConfigYamlContext) as any
  wsConfig.servers[0].uin = qq
  let wsConfigText = dump(wsConfig);
  // 写回去
  await writeTextFile(await join(await getYunzaiDir(), 'plugins/ws-plugin/config/config/ws-config.yaml'), wsConfigText)
}

async function okStep5() {
  openStep.value.step5 = false
}

</script>

