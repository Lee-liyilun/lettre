<template>
  <div id="lettreApp">
    <router-view />

    <!-- 自定义的更新进度弹窗 -->
    <el-dialog
      v-model="updateDialogVisible"
      title="版本更新"
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
          :status="progressPercentage === 100 ? 'success' : 'exception'"
        />
        <!-- 下载完成后显示的重启按钮 -->
        <el-button 
          v-if="showRestartButton" 
          type="primary" 
          @click="handleInstallAndRelaunch"
        >
          立即安装并重启
        </el-button>
      </div>
    </el-dialog>
  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue'
import { ElMessageBox, ElProgress } from 'element-plus'
import { check } from '@tauri-apps/plugin-updater'
import { warn, info } from '@tauri-apps/plugin-log'
import { relaunch } from '@tauri-apps/plugin-process'

// 控制更新进度弹窗的显示
const updateDialogVisible = ref(false)
const updateMessage = ref('')
const showProgress = ref(false)
const progressPercentage = ref(0)
const showRestartButton = ref(false)
let updateObject = null

onMounted(async () => {
  try {
    info('开始检查更新...')
    const update = await check()
    if (update) {
      // 1. 使用 Element Plus 的美观弹窗来询问用户
      info(`发现新版本: v${update.version}`)
      updateObject = update
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
        await update.download((event) => {
          switch (event.event) {
            case 'Started':
              contentLength = event.data.contentLength
              updateMessage.value = `开始下载，总大小: ${(contentLength / 1024 / 1024).toFixed(2)} MB`
              break
            case 'Progress':
              downloaded += event.data.chunkLength
              progressPercentage.value = Math.min(100, Math.floor((downloaded / contentLength) * 100))
              updateMessage.value = `正在下载... ${progressPercentage.value}%`
              break
            case 'Finished':
              // 下载完成，手动将进度条设置为100%
              progressPercentage.value = 100
              updateMessage.value = '新版本下载完成！'
              showProgress.value = false // 隐藏进度条
              showRestartButton.value = true // 显示重启按钮
              break
          }
        })
      } catch (downloadErr) {
        warn(`下载或安装更新失败: ${downloadErr}`)
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
      warn(`检查更新失败: ${err}`) 
      updateDialogVisible.value = false
    }
  }
})

// 用户点击“立即重启”按钮时触发
const handleInstallAndRelaunch = async () => {
  updateMessage.value = '正在安装并重启应用...'
  showRestartButton.value = false // 隐藏按钮，防止重复点击
  
  try {
    // 执行安装
    await updateObject.install()
    // 安装成功后重启（在Windows上，install() 触发后应用通常会自动退出并开始安装）
    await relaunch()
  } catch (err) {
    warn(`安装或重启失败: ${err}`)
    updateMessage.value = '安装失败，请尝试手动重启应用'
  }
}
</script>

<style>
body { margin: 0; padding: 0; width: 100%; height: 100%; }
#lettreApp { width: 100%; height: 100%; display: flex; flex-direction: column; }
</style>