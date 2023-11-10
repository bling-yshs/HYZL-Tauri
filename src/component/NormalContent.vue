<template>
  <div :class="content">
    <slot/>
  </div>
</template>

<script setup lang="ts">
import {computed} from "vue";

const props = defineProps({
  withoutPadding: Boolean
})

const content = computed(() =>
  props.withoutPadding ? 'normal-content-without-padding' : 'normal-content'
)

</script>


<style scoped>
.normal-content {
  padding: 0.7rem;
  background-color: #f9f9fc;
  margin: 0.5rem;
  border-radius: 0.5rem;
  border-color: #e8e3e8;
  border-width: 1px;
  border-style: solid
}

.normal-content-without-padding {
  padding: 0;
  background-color: #f9f9fc;
  margin: 0.5rem;
  border-radius: 0.5rem;
  border-color: #e8e3e8;
  border-width: 1px;
  border-style: solid
}
</style>