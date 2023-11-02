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
            <a-tooltip title="连接到QQNT">
              <a-checkbox v-model:checked="isStartWithQQNT">QQNT</a-checkbox>
            </a-tooltip>
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
import {getYunzaiDir} from "@/entity/hyzlPath.ts";
import checkProcessExist from "@/utils/checkProcessExist.ts";
import {message} from "ant-design-vue";
import {exists} from "@tauri-apps/api/fs";

let tempYunzaiProcess: Child = new Child(31113);
let yunzaiTerminalText = ref('')
let isYunzaiOriginWindow = ref(true)
let isStartWithQQNT = ref(true)

async function startYunzai() {
  if (!await exists(await getYunzaiDir())) {
    message.error("云崽文件夹不存在")
    return
  }
  if (isStartWithQQNT){
    if (isYunzaiOriginWindow.value) {
      console.log("原生窗口")
      const yunzai = new Command('cmd', ['/c', 'start', 'cmd', '/k', 'node', 'apps'], {cwd: await getYunzaiDir()});
      yunzai.spawn()
      return
    }else {
      const yunzai = new Command('node', 'apps', {cwd: await getYunzaiDir()});
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