<template>
  <div id="app">
    <!-- 这里只会渲染 views/index.vue -->
    <router-view />
  </div>
</template>

<script setup>
import { onMounted } from 'vue'
// Tauri 2 必须使用新的插件路径
import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'
import { ask } from '@tauri-apps/plugin-dialog'

onMounted(async () => {
  // 1. 检查本地存储，如果用户今天已经拒绝过更新，则不再提示
  const hasDeclinedUpdate = localStorage.getItem('has_declined_update')
  if (hasDeclinedUpdate === 'true') {
    console.log('用户已选择暂不更新，本次启动不再提示。')
    return
  }

  // 2. 检查是否有新版本
  try {
    const update = await check()
    console.log(update)
    if (update) {
      // 3. 发现新版本，弹出原生询问对话框
      const shouldUpdate = await ask(
        `发现新版本 ${update.version}！\n\n更新说明：${update.body}\n\n是否立即下载并重启应用？`,
        { title: '版本更新', type: 'info', okLabel: '立即更新', cancelLabel: '暂不更新' }
      )

      if (shouldUpdate) {
        // 4. 用户点击“立即更新”
        console.log('开始下载更新...')
        await update.downloadAndInstall((event) => {
          // 可选：在这里监听下载进度
          if (event.event === 'Progress') {
            console.log(`下载进度：${event.data.chunkLength} bytes`)
          }
        })
        // 5. 安装完成，重启应用
        await relaunch()
      } else {
        // 6. 用户点击“暂不更新”，记录状态到本地存储
        localStorage.setItem('has_declined_update', 'true')
        console.log('用户选择暂不更新，已记录状态。')
      }
    } else {
      console.log('当前已是最新版本。')
    }
  } catch (error) {
    console.error('检查更新失败：', error)
  }
})

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