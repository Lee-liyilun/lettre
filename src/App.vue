<template>
  <div id="app">
    <!-- 这里只会渲染 views/index.vue -->
    <router-view />
  </div>
</template>

<script setup>
// 不需要额外逻辑
</script>

<style>
/* 全局样式，比如去除默认边距 */
body {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
}
#app {
  width: 100%;
  height: 100%;
  display: flex; /* 开启 Flex 布局 */
  flex-direction: column; /* 垂直排列 */
}
</style>