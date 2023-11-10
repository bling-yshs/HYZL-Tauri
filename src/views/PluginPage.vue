<template>
  <div>
    <normal-content :without-padding="true">
      <a-alert message="该界面还在开发中，没有任何实际功能" type="warning" />
    </normal-content>
    <normal-content>
      <a-space>
        <a-button>打开云崽插件库</a-button>
        <a-button @click="()=>{message.info(columns.length)}">test</a-button>
      </a-space>
    </normal-content>
    <normal-content :without-padding="true">
      <a-table style="border-radius: 0.5rem"
               :columns="columns" :data-source="pluginData"
               bordered
               size="small"
               :pagination="pagination"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.dataIndex === 'tags'">
            <a-tag
              v-for="tag in record.tags"
              :key="tag"
              :color="tag === '已安装' ? 'green' : tag === '未安装' ? 'red' : 'blue'"
            >
              {{ tag.toUpperCase() }}
            </a-tag>
          </template>
          <template v-if="column.dataIndex==='action'">
            <a-space>
              <a-button v-if="record.tags.includes('未安装')" type="dashed" size="small">安装</a-button>
              <a-button v-if="record.tags.includes('已安装')" type="primary" size="small">更新</a-button>
              <a-button size="small">查看仓库</a-button>
              <a-button v-if="record.tags.includes('已安装')" size="small" danger>卸载</a-button>
            </a-space>
          </template>
        </template>
      </a-table>
    </normal-content>
  </div>
</template>


<script setup lang="ts">
import NormalContent from "@/component/NormalContent.vue";
import {computed} from "vue";
import {message} from "ant-design-vue";

const pagination = computed(() => ({
  total: pluginData.length,
  pageSize: 6
}));
const columns = [
  {
    title: '插件名称',
    dataIndex: 'name',
  },
  {
    title: '标签',
    dataIndex: 'tags',
  },
  {
    title: '操作',
    dataIndex: 'action',
  },
];

const pluginData = [
  {
    key: '1',
    name: '喵喵插件',
    tags: ['已安装'],
  },
  {
    key: '2',
    name: '锅巴插件',
    tags: ['已安装'],
  },
  {
    key: '3',
    name: 'Joe Black',
    tags: ['未安装'],
  },
  {
    key: '4',
    name: 'Joe Black',
    tags: ['未安装'],
  },
  {
    key: '5',
    name: 'Joe Black',
    tags: ['未安装'],
  },
  {
    key: '6',
    name: 'Joe Black',
    tags: ['未安装'],
  },
  {
    key: '7',
    name: 'Joe Black',
    tags: ['未安装'],
  },
  {
    key: '8',
    name: 'Joe Black',
    tags: ['未安装'],
  },
  {
    key: '9',
    name: 'Joe Black9',
    tags: ['未安装'],
  },
  {
    key: '10',
    name: 'Joe Black10',
    tags: ['未安装'],
  },
  {
    key: '11',
    name: 'Joe Black11',
    tags: ['未安装'],
  },
  {
    key: '12',
    name: 'Joe Black12',
    tags: ['未安装'],
  },
  {
    key: '13',
    name: 'Joe Black13',
    tags: ['未安装'],
  },
];
</script>

<style scoped>
</style>