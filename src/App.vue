<script setup lang="ts">
import { message } from "ant-design-vue";
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";

const [messageApi, contextHolder] = message.useMessage();

const info = () => {
  messageApi.info("Hello, Ant Design Vue!");
};

let msg = ref("Click here to get a message from backend");

const getMessage = () => {
  invoke("get_message").then((res) => {
    msg.value = res as string;
  });
};

let num = ref(50);

const plusFive = () => {
  invoke("plus_five", { number: num.value }).then((res) => {
    num.value = res as number;
  });
};
</script>

<template>
  <div id="app">
    <a-space direction="vertical" :align="'center'">
      <a-space>
        <context-holder />
        <a-button type="primary" @click="info">Display ant-design normal message</a-button>
      </a-space>
      <a-space>
        <a-button type="default" @click="getMessage">{{ msg }}</a-button>
      </a-space>
      <a-space>
        <a-button type="default" @click="plusFive">Click here to add 5 to this number: {{ num }}</a-button>
      </a-space>
    </a-space>
  </div>
</template>

<style scoped>
#app {
  height: 100%;
  width: 100%;
  position: fixed;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
