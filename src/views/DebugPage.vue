<template>
  <div>
    <normal-content>
      <div class="terminalTitle">
        <icon>
          <template #component>
            <svg width="1em" height="1em" viewBox="0 0 1024 1024">
              <path
                  d="M512 640h256v85.333333h-256zM243.2 631.466667l59.733333 59.733333 179.2-179.2-179.2-179.2-59.733333 59.733333L362.666667 512z"
                  fill="#09121F"/>
              <path
                  d="M896 128H128c-25.6 0-42.666667 17.066667-42.666667 42.666667v682.666666c0 25.6 17.066667 42.666667 42.666667 42.666667h768c25.6 0 42.666667-17.066667 42.666667-42.666667V170.666667c0-25.6-17.066667-42.666667-42.666667-42.666667z m-42.666667 682.666667H170.666667V213.333333h682.666666v597.333334z"
                  fill="#09121F"/>
            </svg>
          </template>
        </icon>
        <p>云崽终端</p>
        <a-button
            @click="killCommandProcess"
            danger
            style="margin-left: auto"
        >
          终止当前命令
        </a-button>
      </div>
      <a-textarea
          :rows="10"
          readonly
          :value="terminalText"
          placeholder="欢迎使用 HYZL-Tauri"
          style="margin-bottom: 0.5rem"
          id="terminal"
      >
      </a-textarea>
      <a-space-compact block>
        <a-input
            :allowClear="true"
            v-model:value="commandText"
            placeholder="在这里输入命令"
            @pressEnter="submitCommand"
        >
          <template #enterButton>
            <a-button>
              发送
              <send-outlined/>
            </a-button>
          </template>
        </a-input>
        <a-button @click="copyTerminalLog">
          复制终端日志
        </a-button>
        <a-button type="primary" @click="submitCommand">
          <send-outlined/>
          发送
        </a-button>
      </a-space-compact>
    </normal-content>
    <normal-content>
      <a-row>
        <a-col>
          <a-space direction="vertical">
            <a-space>
              <FolderOpenOutlined/>
              <p>打开</p>
            </a-space>
            <a-space>
              <a-button type="default" @click="openYunzaiFolder">云崽文件夹</a-button>
              <a-button type="default" @click="openUFQFolder">签名 API 文件夹</a-button>
            </a-space>
          </a-space>
        </a-col>
      </a-row>
    </normal-content>
  </div>
</template>
<script setup lang="ts">
import icon, {FolderOpenOutlined, SendOutlined} from '@ant-design/icons-vue';
import NormalContent from "./NormalContent.vue";
import {message} from "ant-design-vue";
import {Child, Command, open} from '@tauri-apps/api/shell';
import {getYunzaiDir, getSignApiDir} from "@/entity/hyzlPath.ts";
import {ref, watch} from "vue";
import {exists} from "@tauri-apps/api/fs";
import {writeText} from '@tauri-apps/api/clipboard';

const killCommandProcess = async () => {
  if (await checkProcessExist(tempCommandProcess.value.pid)) {
    tempCommandProcess.value.kill()
    message.success({content: '成功终止命令', duration: 2})
  } else {
    message.error({content: '当前没有正在运行的命令', duration: 2})
  }
};

//终端命令显示
let terminalText = ref<string>('')
let commandText = ref<string>('')
let tempCommandProcess = ref<Child>(new Child(31113))
let disableKill = ref<boolean>(true)

async function checkProcessExist(pid: number): Promise<boolean> {
  const checkCommand = new Command('ps', ['Get-Process', '-Id', pid.toString(), '-ErrorAction', 'SilentlyContinue']);
  const childProcess = await checkCommand.execute();
  if (childProcess.code === 1) {
    return false
  }
  if (childProcess.code === 0) {
    return true
  }
  return true
}

async function submitCommand() {
  if (await checkProcessExist(tempCommandProcess.value.pid)) {
    tempCommandProcess.value.write(commandText.value + '\n')
    return
  }
  const command = new Command('ps', commandText.value, {cwd: await getYunzaiDir()});
  command.on('error', error => {
    terminalText.value += error
    message.error({content: '命令执行失败' + error, duration: 2})
  });
  command.stdout.on('data', (data) => {
    terminalText.value += data
  })
  command.stderr.on('data', (data) => {
    terminalText.value += data
  })
  tempCommandProcess.value = await command.spawn();
  commandText.value = ''
}

watch(terminalText, async () => {
  const terminal = document.getElementById('terminal') as HTMLTextAreaElement;
  terminal.scrollTop = terminal.scrollHeight;
})

//监听tempCommandProcess
watch(tempCommandProcess, async (newProcess) => {
  if (newProcess != null) {
    disableKill.value = false
    return
  } else {
    disableKill.value = true
  }
})

async function copyTerminalLog() {
  writeText(terminalText.value)
}

//打开文件夹
async function openUFQFolder() {
  if (!await exists(await getSignApiDir())) {
    message.error({content: '签名 API 文件夹不存在', duration: 2})
    return
  }
  await open(await getSignApiDir())
}

async function openYunzaiFolder() {
  if (!await exists(await getYunzaiDir())) {
    message.error({content: '云崽文件夹不存在', duration: 2})
    return
  }
  await open(await getYunzaiDir())
}
</script>

<style scoped>
.center {
  display: flex;
  justify-content: center;
  align-items: center;
}

.terminalTitle {
  display: flex;
  align-items: center;
  margin-bottom: 0.5rem;
}

</style>