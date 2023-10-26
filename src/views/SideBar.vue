<template>
  <a-layout class="parent-element">
    <a-layout-sider
      v-model:collapsed="collapsed"
      breakpoint="md"
      theme="light"
    >
      <!--<a-button @click="testFn"></a-button>-->
      <a-button type="text" @click="() => {collapsed = !collapsed}">
        <menu-unfold-outlined v-if="collapsed"/>
        <menu-fold-outlined v-else/>
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
//   console.log(route.fullPath)
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
.parent-element {
  background-color: transparent;
}

.parent-element * {
  background-color: transparent;
}

</style>
