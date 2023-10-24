<template>
  <div :class="{'not-win11':notWin11}">
    <NormalPage/>
  </div>
</template>

<script setup lang="ts">
import NormalPage from "./views/IndexPage.vue";
import {onMounted, ref} from "vue";

let notWin11 = ref(true);
import {version} from '@tauri-apps/api/os';

onMounted(async () => {
  changeWhenNotWin11()
});

async function changeWhenNotWin11() {
  const osVersion = await version();
  Number(osVersion.split('.')[2]) > 22000 ? notWin11.value = false : notWin11.value = true
}
</script>
<style scoped>
.not-win11 {
  background-color: #fff;
  position: absolute;
  min-height: 100%;
  min-width: 100%;
}
</style>
