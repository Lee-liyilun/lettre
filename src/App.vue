<template>
  <div id="app">
    <router-view />

    <!-- 自定义的更新进度弹窗 -->
    <el-dialog
      v-model="updateDialogVisible"
      title="正在更新"
      width="400px"
      :close-on-click-modal="false"
      :close-on-press-escape="false"
      :show-close="false"
    >
      <div style="text-align: center; padding: 20px 0;">
        <p>{{ updateMessage }}</p>
        <el-progress 
          v-if="showProgress" 
          :percentage="progressPercentage" 
          :status="progressPercentage === 100 ? 'success' : ''"
        />
      </div>
    </el-dialog>
  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue'
import { ElMessageBox, ElProgress } from 'element-plus'
import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'

// 控制更新进度弹窗的显示
const updateDialogVisible = ref(false)
const updateMessage = ref('')
const showProgress = ref(false)
const progressPercentage = ref(0)

onMounted(async () => {
  try {
    const update = await check()
    if (update) {
      // 1. 使用 Element Plus 的美观弹窗来询问用户
      await ElMessageBox.confirm(
        `发现新版本 v${update.version}！\n\n`,
        '版本更新',
        {
          confirmButtonText: '立即更新',
          cancelButtonText: '暂不更新',
          type: 'info',
        }
      )

      // 2. 用户点击“立即更新”后，显示自定义的进度条弹窗
      updateDialogVisible.value = true
      updateMessage.value = '准备开始下载...'
      showProgress.value = true

      let downloaded = 0
      let contentLength = 0

      // 3. 下载并安装，监听详细的事件来更新进度条
      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            contentLength = event.data.contentLength
            updateMessage.value = `开始下载，总大小: ${(contentLength / 1024 / 1024).toFixed(2)} MB`
            break
          case 'Progress':
            downloaded += event.data.chunkLength
            // 计算并更新百分比
            progressPercentage.value = Math.floor((downloaded / contentLength) * 100)
            updateMessage.value = `正在下载... ${progressPercentage.value}%`
            break
          case 'Finished':
            updateMessage.value = '下载完成，正在安装并重启应用...'
            progressPercentage.value = 100
            break
        }
      })

      // 4. 安装完成，重启应用
      await relaunch()

    } else {
      console.log('当前已是最新版本。')
    }
  } catch (error) {
    // 如果用户点击了“暂不更新”，ElMessageBox 会抛出异常，这里直接捕获即可
    if (error !== 'cancel') {
      console.error('检查更新失败：', error)
      updateDialogVisible.value = false
    }
  }
})
</script>

<style>
body { margin: 0; padding: 0; width: 100%; height: 100%; }
#app { width: 100%; height: 100%; display: flex; flex-direction: column; }
</style>