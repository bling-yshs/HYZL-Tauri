<template>
  <div>
    <normal-content>
      <a-space direction="vertical">
        <a-row>
          <a-col :span="24">
            <a-image
              :preview="false"
              style="border-radius: 0.5rem;object-fit: cover;"
              :src="indexImage"
            />
          </a-col>
        </a-row>
        <a-space>
          <CodeOutlined/>
          <p>云崽日志</p>
        </a-space>
        <a-textarea
          :rows="10"
          readonly
          :value="yunzaiTerminalText"
          placeholder="欢迎使用 HYZL-Tauri"
          id="yunzaiTerminal"
        >
        </a-textarea>
        <a-space style="display: flex; justify-content: space-between;">
          <a-space>
            <a-button type="primary" @click="startYunzai">启动云崽</a-button>
            <a-checkbox v-model:checked="isStartWithSignApi">同时启动签名API</a-checkbox>
            <a-tooltip title="会在命令行窗口中打开云崽">
              <a-checkbox v-model:checked="isYunzaiOriginWindow">原生窗口</a-checkbox>
            </a-tooltip>
          </a-space>
          <a-space>
            <a-button>复制云崽日志</a-button>
            <a-dropdown-button @click="killYunzaiProcess" danger>
              停止云崽
              <template #overlay>
                <a-menu>
                  <a-menu-item key="1" danger @click="killAllNode">
                    强制停止所有 Node 程序
                  </a-menu-item>
                </a-menu>
              </template>
            </a-dropdown-button>
          </a-space>
        </a-space>
        
        <a-space>
          <CodeOutlined/>
          <p>签名API日志</p>
        </a-space>
        <a-textarea
          :rows="10"
          readonly
          :value="signApiTerminalText"
          placeholder="欢迎使用 HYZL-Tauri"
          id="signApiTerminal"
        >
        </a-textarea>
        <a-space style="display: flex; justify-content: space-between;">
          <a-space>
            <a-button type="primary" @click="startSignApi">启动签名API</a-button>
            <a-tooltip title="会在命令行窗口中打开签名API">
              <a-checkbox v-model:checked="isSignApiOriginWindow">原生窗口</a-checkbox>
            </a-tooltip>
          </a-space>
          <a-space>
            <a-button>复制签名API日志</a-button>
            <a-button @click="killSignApiProcess" danger>停止签名API</a-button>
          </a-space>
        </a-space>
      </a-space>
    </normal-content>
  </div>
</template>
<script setup lang="ts">
import {CodeOutlined} from '@ant-design/icons-vue';
import NormalContent from "@/component/NormalContent.vue"
import indexImage from "@/assets/index-iamge.jpg";
import {ref, watch} from "vue";
import {Child, Command} from "@tauri-apps/api/shell";
import {getSignApiDir, getYunzaiDir} from "@/entity/hyzlPath.ts";
import checkProcessExist from "@/utils/checkProcessExist.ts";
import {message} from "ant-design-vue";
import {exists} from "@tauri-apps/api/fs";

let tempYunzaiProcess: Child = new Child(31113);
let yunzaiTerminalText = ref('')
let isStartWithSignApi = ref(false)
let isYunzaiOriginWindow = ref(false)

async function startYunzai() {
  if (!await exists(await getYunzaiDir())) {
    message.error("云崽文件夹不存在")
    return
  }
  if (isStartWithSignApi.value) {
    await startSignApi()
  }
  if (isYunzaiOriginWindow.value) {
    console.log("原生窗口")
    const yunzai = new Command('cmd', ['/c', 'start', 'cmd', '/k', 'node', 'app'], {cwd: await getYunzaiDir()});
    yunzai.spawn()
    return
  }
  const yunzai = new Command('node', 'app', {cwd: await getYunzaiDir()});
  yunzai.stdout.on('data', (data) => {
    yunzaiTerminalText.value += data;
  })
  yunzai.stderr.on('data', (data) => {
    yunzaiTerminalText.value += data;
  })
  yunzai.on('close', (code) => {
    yunzaiTerminalText.value += `云崽已退出，退出码：${code.code}`
    console.log(code)
  });
  tempYunzaiProcess = await yunzai.spawn();
}

async function killYunzaiProcess() {
  if (await checkProcessExist(tempYunzaiProcess.pid)) {
    tempYunzaiProcess.kill();
    message.success({content: '成功停止云崽', duration: 2})
  } else {
    message.error({content: '当前没有正在运行的云崽', duration: 2})
  }
}

watch(yunzaiTerminalText, async () => {
  const terminal = document.getElementById('yunzaiTerminal') as HTMLTextAreaElement;
  terminal.scrollTop = terminal.scrollHeight;
})

async function killAllNode() {
  const killAllNode = new Command('cmd', ['/c', 'taskkill', '/f', '/im', 'node.exe']);
  killAllNode.spawn();
}


let tempSignApiProcess: Child = new Child(31113);
let isSignApiOriginWindow = ref(false)
let signApiTerminalText = ref('')

async function startSignApi() {
  if (!await exists(await getSignApiDir())) {
    message.error("签名API文件夹不存在")
    return
  }
  if (isSignApiOriginWindow.value) {
    const signApi = new Command('cmd', ['/c', 'start', 'cmd', '/k', 'start.bat'], {cwd: await getSignApiDir()});
    signApi.spawn()
    return
  }
  const signApi = new Command('ps', ['./start.bat'], {cwd: await getSignApiDir()});
  signApi.stdout.on('data', (data) => {
    signApiTerminalText.value += data;
  })
  signApi.stderr.on('data', (data) => {
    signApiTerminalText.value += data;
  })
  signApi.on('close', (code) => {
    signApiTerminalText.value += `签名API已退出，退出码：${code.code}`
    console.log(code)
  });
  tempSignApiProcess = await signApi.spawn();
}

async function killSignApiProcess() {
  console.log(tempSignApiProcess.pid)
  if (await checkProcessExist(tempSignApiProcess.pid)) {
    tempSignApiProcess.kill();
    message.success({content: '成功停止签名API', duration: 2})
  } else {
    message.error({content: '当前没有正在运行的签名API', duration: 2})
  }
}

watch(signApiTerminalText, async () => {
  const terminal = document.getElementById('signApiTerminal') as HTMLTextAreaElement;
  terminal.scrollTop = terminal.scrollHeight;
})


</script>
<style scoped>
/* 下面我们会解释这些 class 是做什么的 */
.v-enter-active,
.v-leave-active {
  transition: opacity 0.5s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>