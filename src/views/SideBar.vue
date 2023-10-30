<template>
  <a-layout class="parent-element">
    <a-layout-sider
      v-model:collapsed="collapsed"
      breakpoint="md"
      theme="light"
    >
      <!--<a-button @click="testFn"></a-button>-->
      <a-button type="text" @click="() => {collapsed = !collapsed}">
        <Transition mode="out-in" name="slide-fade">
          <menu-unfold-outlined v-if="collapsed"/>
          <menu-fold-outlined v-else/>
        </Transition>
      </a-button>
      <!--菜单-->
      <a-menu style="border-right: 0" v-model:selectedKeys="selectedKeys" theme="light" mode="inline">
        <a-menu-item key="/start">
          <rocket-outlined/>
          <span>主页</span>
        </a-menu-item>
        <a-menu-item key="/download">
          <cloud-download-outlined/>
          <span>下载</span>
        </a-menu-item>
        <a-menu-item key="/issue-fix">
          <bug-outlined/>
          <span>问题修复</span>
        </a-menu-item>
        <a-menu-item key="/debug">
          <desktop-outlined/>
          <span>调试</span>
        </a-menu-item>
        <a-menu-item key="/settings">
          <SettingOutlined/>
          <span>设置</span>
        </a-menu-item>
        <a-menu-item key="/about">
          <info-circle-outlined/>
          <span>关于</span>
        </a-menu-item>
      </a-menu>
    </a-layout-sider>
  </a-layout>
</template>
<script lang="ts" setup>
import {
  CloudDownloadOutlined,
  InfoCircleOutlined,
  DesktopOutlined,
  BugOutlined,
  MenuUnfoldOutlined,
  MenuFoldOutlined,
  RocketOutlined,
  SettingOutlined,
} from '@ant-design/icons-vue';
import {onMounted, ref, watch} from 'vue';
import {useRoute} from 'vue-router'


// const testFn = () => {
//   const webview = new WebviewWindow('theUniqueLabel', {
//     url: './issue-fix'
//   });
// };

// 收放按钮
const collapsed = ref<boolean>(false);
const route = useRoute();

// 选择的菜单项
const selectedKeys = ref<string[]>(['']);
const emit = defineEmits(["changeSelect"]);
// 监控路由变化来改变选择的菜单项
onMounted(async () => {
  setTimeout(() => {
    selectedKeys.value = [route.fullPath];
  }, 100)
});

// 路由跳转
watch(() => selectedKeys.value, (value) => {
  emit('changeSelect', value);
});


</script>
<style scoped>
.parent-element, .parent-element * {
  background-color: transparent;
}
.slide-fade-leave-active {
  transition: all 0.1s cubic-bezier(1, 0.5, 0.8, 1);
  animation: rotateOut 0.1s cubic-bezier(1, 0.5, 0.8, 1);
}

.slide-fade-leave-to {
  transform: rotate(360deg);
  opacity: 0;
}

.slide-fade-enter-from {
  transform: rotate(360deg);
  opacity: 0;
}

.slide-fade-enter-active {
  transition: all 0.3s ease-out;
  animation: rotateIn 0.3s ease-out;
}

@keyframes rotateOut {
  from {
    transform: none;
  }
  to {
    transform: rotate(360deg);
    opacity: 0;
  }
}

@keyframes rotateIn {
  from {
    transform: rotate(360deg);
    opacity: 0;
  }
  to {
    transform: none;
    opacity: 1;
  }
}


</style>
