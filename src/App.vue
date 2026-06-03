<template>
  <div id="app">
    <router-view />

    <!-- 自定义的更新进度弹窗 -->
    <el-dialog
      v-model="updateDialogVisible"
      title="正在更新"
      width="400px"
      :close-on-click-modal="true"
      :close-on-press-escape="true"
      :show-close="true"
    >
      <div style="text-align: center; padding: 20px 0;">
        <p>{{ updateMessage }}</p>
        <el-progress 
          v-if="showProgress" 
          :percentage="progressPercentage" 
          :status="progressPercentage === 100 ? 'success' : 'exception'"
        />
      </div>
    </el-dialog>
  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue'
import { ElMessageBox, ElProgress } from 'element-plus'
import { check } from '@tauri-apps/plugin-updater'
import { error, info } from '@tauri-apps/plugin-log'
import { relaunch } from '@tauri-apps/plugin-process'

// 控制更新进度弹窗的显示
const updateDialogVisible = ref(false)
const updateMessage = ref('')
const showProgress = ref(false)
const progressPercentage = ref(0)

onMounted(async () => {
  try {
    info('开始检查更新...')
    const update = await check()
    if (update) {
      // 1. 使用 Element Plus 的美观弹窗来询问用户
      info(`发现新版本: v${update.version}`)
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
      try {
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
        setTimeout(async () => {
          await relaunch()
        }, 3000) 
      } catch (downloadErr) {
        error(`下载或安装更新失败: ${downloadErr}`)
        updateMessage.value = '更新失败，请检查网络后重试'
        progressPercentage.value = 0
        showProgress.value = false
      }

    } else {
      info('当前已是最新版本。')
    }
  } catch (err) {
    // 如果用户点击了“暂不更新”，ElMessageBox 会抛出 'cancel' 异常
    if (err === 'cancel') {
      info('用户选择暂不更新') // 👈 正常记录业务操作
    } else {
      error(`检查更新失败: ${err}`) 
      updateDialogVisible.value = false
    }
  }
})
</script>

<style>
body { margin: 0; padding: 0; width: 100%; height: 100%; }
#app { width: 100%; height: 100%; display: flex; flex-direction: column; }
</style>