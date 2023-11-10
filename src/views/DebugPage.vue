<template>
  <div>
    <normal-content>
      <a-space direction="vertical" style="width: 100%">
        <a-space>
          <CodeOutlined/>
          <p>云崽终端</p>
        </a-space>
        <a-textarea
          :rows="10"
          readonly
          :value="terminalText"
          placeholder="欢迎使用 HYZL-Tauri"
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
          <a-button type="primary" @click="submitCommand">
            <send-outlined/>
            发送
          </a-button>
        </a-space-compact>
        <a-space style="display: flex; justify-content: space-between;">
          <a-button @click="copyTerminalLog">
            复制终端日志
          </a-button>
          <a-button
            @click="killCommandProcess"
            danger
            style="margin-left: auto"
          >
            终止当前命令
          </a-button>
        </a-space>
      </a-space>
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
              <a-button type="default" @click="openCacheFolder">缓存文件夹</a-button>
            </a-space>
          </a-space>
        </a-col>
      </a-row>
    </normal-content>
  </div>
</template>
<script setup lang="ts">
import {CodeOutlined, FolderOpenOutlined, SendOutlined} from '@ant-design/icons-vue';
import NormalContent from "@/component/NormalContent.vue"
import {message} from "ant-design-vue";
import {Child, Command, open} from '@tauri-apps/api/shell';
import {getAppCacheDir, getYunzaiDir} from "@/entity/hyzlPath.ts";
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
  const command = new Command('ps', commandText.value, {cwd: await getYunzaiDir(), encoding: 'gbk'});
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
  await writeText(terminalText.value)
  message.success({content: '复制成功', duration: 2})
}

async function openYunzaiFolder() {
  if (!await exists(await getYunzaiDir())) {
    message.error({content: '云崽文件夹不存在', duration: 2})
    return
  }
  await open(await getYunzaiDir())
}

async function openCacheFolder() {
  await open(await getAppCacheDir())
}







</script>



<style scoped>

</style>