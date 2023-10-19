<template>
  <div class="container">
    <div>
      <SideBar @changeSelect="handleChangeSelect"/>
    </div>
    <div class="content">
      <router-view/>
    </div>
  </div>
</template>

<script setup lang="ts">
import SideBar from "./SideBar.vue";
import router from "../router";

const handleChangeSelect = (value: Array<string>) => {
  //修改路由
  router.push(value.toString())
};
</script>

<style scoped>
.container {
  display: flex;
}


.content {
  flex-grow: 1;
}
</style>