<template>
  <div>
    <normal-content>
      <h4>云崽</h4>
      <a-space direction="horizontal">
        <a-button type="default" @click="downloadMiaoYunzai">喵喵云崽</a-button>
      </a-space>
    </normal-content>
    <normal-content>
      <h4>签名API</h4>
      <a-space direction="horizontal">
        <a-button type="default" @click="downloadUFQ">unidbg-fetch-qsign</a-button>
      </a-space>
    </normal-content>
  </div>
</template>
<script setup lang="ts">

import NormalContent from "./NormalContent.vue";
import {message} from 'ant-design-vue';

async function downloadUFQ() {
  message.info("暂未制作")
}

async function downloadMiaoYunzai() {
  message.info("暂未制作")
}
</script>

<style scoped>


</style>