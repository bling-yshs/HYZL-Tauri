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
        <div v-if="showRobotInfo">
          <a-space direction="vertical">
            <a-space>
              <setting-outlined/>
              <span>云崽设置</span>
            </a-space>
            <a-space>
              <a-input v-model:value="robotInfo.robotQQ" placeholder="机器人QQ号">
                <template #prefix>
                  <qq-outlined/>
                </template>
              </a-input>
              <a-input-password v-model:value="robotInfo.robotPassword" placeholder="机器人密码">
                <template #prefix>
                  <lock-outlined/>
                </template>
              </a-input-password>
              <a-input v-model:value="robotInfo.masterQQ" placeholder="主人QQ号">
                <template #prefix>
                  <user-outlined/>
                </template>
              </a-input>
            </a-space>
          </a-space>
        </div>
        
        <a-space>
          <CodeOutlined/>
          <p>云崽日志</p>
        </a-space>
        <a-textarea
          :rows="8"
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
import {CodeOutlined, LockOutlined, QqOutlined, SettingOutlined, UserOutlined} from '@ant-design/icons-vue';
import NormalContent from "@/component/NormalContent.vue"
import indexImage from "@/assets/index-iamge.jpg";
import {onMounted, ref, watch} from "vue";
import {Child} from "@tauri-apps/api/shell";
import {getAppDir, getYunzaiDir} from "@/entity/hyzlPath.ts";
import checkProcessExist from "@/utils/checkProcessExist.ts";
import {message} from "ant-design-vue";
import {exists, readTextFile, writeTextFile} from "@tauri-apps/api/fs";
import {join} from "@tauri-apps/api/path";
import {dump, load} from "js-yaml";
import fastCommand from "@/utils/fastCommand.ts";
import checkProgramExist from "@/utils/checkProgramExist.ts";


// 机器人信息设置

let showRobotInfo = ref(false)

interface RobotInfo {
  robotQQ: string,
  robotPassword: string,
  masterQQ: string
}

onMounted(async () => {
  // 判断是否存在云崽/config/config文件夹
  if (await exists(await join(await getYunzaiDir(), 'config/config/qq.yaml')) && await exists(await join(await getYunzaiDir(), 'config/config/other.yaml'))) {
    showRobotInfo.value = true
    qqYamlContext = await readTextFile(await join(await getYunzaiDir(), 'config/config/qq.yaml'));
    qqYamlObject = load(qqYamlContext) as any;
    robotInfo.value.robotQQ = qqYamlObject.qq;
    robotInfo.value.robotPassword = qqYamlObject.pwd;
    otherYamlContext = await readTextFile(await join(await getYunzaiDir(), 'config/config/other.yaml'));
    otherYamlObject = load(otherYamlContext) as any;
    robotInfo.value.masterQQ = (otherYamlObject.masterQQ ? otherYamlObject.masterQQ[0] : '')
  }
})

let qqYamlContext: string = '';
let qqYamlObject: any = {};
let otherYamlContext: string = '';
let otherYamlObject: any = {};

let robotInfo = ref<RobotInfo>({
  robotQQ: qqYamlObject.qq,
  robotPassword: qqYamlObject.pwd,
  masterQQ: (otherYamlObject.masterQQ ? otherYamlObject.masterQQ[0] : '')
})

watch(robotInfo.value, async (newValue) => {
  if (newValue.robotQQ != qqYamlObject.qq) {
    qqYamlObject.qq = newValue.robotQQ;
    qqYamlContext = dump(qqYamlObject)
    await writeTextFile(await join(await getYunzaiDir(), 'config/config/qq.yaml'), qqYamlContext);
  }
  if (newValue.robotPassword != qqYamlObject.pwd) {
    qqYamlObject.pwd = newValue.robotPassword;
    qqYamlContext = dump(qqYamlObject)
    await writeTextFile(await join(await getYunzaiDir(), 'config/config/qq.yaml'), qqYamlContext);
  }
  if (newValue.masterQQ != (otherYamlObject.masterQQ ? otherYamlObject.masterQQ[0] : '')) {
    otherYamlObject.masterQQ = [newValue.masterQQ];
    otherYamlContext = dump(otherYamlObject)
    await writeTextFile(await join(await getYunzaiDir(), 'config/config/other.yaml'), otherYamlContext);
  }
})


// 终端


let tempYunzaiProcess: Child = new Child(31113);
let yunzaiTerminalText = ref('')
let isYunzaiOriginWindow = ref(true)
let isStartWithQQNT = ref(true)


// 如果未安装Redis，返回false，其它情况返回true
async function startRedis(): Promise<boolean> {
  // 检查redis是否安装
  if (!await exists(await join(await getAppDir(), 'redis-windows-7.0.4'))) {
    message.error("请先下载 Redis")
    return false
  }
  // 检查redis是否启动
  if (!await checkProgramExist('redis')) {
    await fastCommand(
      './redis-server.exe ./redis.conf',
      await join(await getAppDir(), 'redis-windows-7.0.4'),
      true
    ).spawn();
  }
  return true
}

async function startYunzai() {
  if (!await exists(await getYunzaiDir())) {
    message.error("云崽文件夹不存在")
    return
  }
  if (!await startRedis()) {
    return
  }
  // 判断账号，还有主人QQ是否为空，如果为空，就不启动云崽
  if (robotInfo.value.robotQQ === '') {
    message.error("请先设置云崽QQ账号")
    return
  }
  if (robotInfo.value.masterQQ === '') {
    message.error("请先设置云崽主人QQ")
    return
  }
  if (isStartWithQQNT.value === true) {
    // 检查QQ是否已经启动
    if (isYunzaiOriginWindow.value === true) {
      fastCommand('node apps', await getYunzaiDir(), true).spawn()
      return
    } else {
      const yunzai = fastCommand('node apps', await getYunzaiDir(), false);
      yunzai.stdout.on('data', (data) => {
        yunzaiTerminalText.value += data;
      })
      yunzai.stderr.on('data', (data) => {
        yunzaiTerminalText.value += data;
      })
      yunzai.on('close', (code) => {
        yunzaiTerminalText.value += `云崽已退出，退出码：${code.code}`
      });
      tempYunzaiProcess = await yunzai.spawn();
    }
  }
  if (isStartWithQQNT.value === false) {
    if (isYunzaiOriginWindow.value === true) {
      fastCommand('node app', await getYunzaiDir(), true).spawn()
      return
    } else {
      const yunzai = fastCommand('node app', await getYunzaiDir(), false);
      yunzai.stdout.on('data', (data) => {
        yunzaiTerminalText.value += data;
      })
      yunzai.stderr.on('data', (data) => {
        yunzaiTerminalText.value += data;
      })
      yunzai.on('close', (code) => {
        yunzaiTerminalText.value += `云崽已退出，退出码：${code.code}`
      });
      tempYunzaiProcess = await yunzai.spawn();
    }
  }
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
  const res = await fastCommand('taskkill /f /im node.exe').execute();
  if (res.code === 128) {
    message.error('没有找到存活的 node 进程')
  }
  if (res.code === 0) {
    // res.stdout 按换行符分割
    let strings = res.stdout.split(/\r?\n/);
    for (let string of strings) {
      if (string === '') {
        continue
      }
      message.success(string)
    }
  }
}

</script>
<style scoped>
</style>