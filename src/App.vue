<template>
  <div :class="{'not-win11':notWin11}">
    <NormalPage/>
  </div>
</template>

<script setup lang="ts">
import NormalPage from "./views/IndexPage.vue";
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {DataResponse} from "./entity/response.ts";
let notWin11 = ref(true);

onMounted(async () => {
  let response = await invoke('is_win11');
  let res = JSON.parse(response as string) as DataResponse;
  if (res.code === 200) {
    notWin11.value = false;
  }
});
</script>
<style scoped>
.not-win11 {
  background-color: #fff;
  position: absolute;
  min-height: 100%;
  min-width: 100%;
}
</style>
