<template>
  <div class="container">
    <div>
      <SideBar @changeSelect="handleChangeSelect"/>
    </div>
    <div class="content">
      <router-view v-slot="{ Component }">
        <transition name="slide-fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
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

.slide-fade-enter-active {
  transition: all 0.3s ease-out;
}

.slide-fade-leave-active {
  transition: all 0.1s cubic-bezier(1, 0.5, 0.8, 1);
}

.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateX(1rem);
  opacity: 0;
}
</style>