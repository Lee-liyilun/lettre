<template>
  <div class="secret-container">
    <!-- 头部区域 -->
    <el-row>
      <el-col :span="22" :offset="1">
        <div class="header-wrapper">
          <el-page-header @back="goBack">
            <template #content>
              <span class="page-title">密语</span>
            </template>
            <template #extra>
              <div class="flex items-center">
                <el-button type="warning" size="small" @click="backupKey">
                  备份密钥
                </el-button>
                <el-button type="success" size="small" @click="importKey">
                  导入密钥
                </el-button>
              </div>
            </template>
          </el-page-header>
        </div>
      </el-col>
    </el-row>

    <!-- 主体卡片区域 -->
    <el-row>
      <el-col :span="22" :offset="1">
        <div class="card-wrapper">
          <div class="secret-layout">
            <!-- 左侧搜索 + 列表 -->
            <div class="secret-sidebar">
              <!-- 搜索 + 列表 -->
              <template v-if="!showToc">
                <el-input
                  v-model="kw"
                  placeholder="搜索项目名称/公司/内容"
                  @input="doSearch"
                  clearable
                />

                <div class="sidebar-list">
                  <div class="btn-row" :class="{ 'has-export': showExportBtn }">
                    <el-button type="primary" @click="create">
                      新建密语
                    </el-button>
                    <el-button
                      v-show="showExportBtn"
                      class="export-btn"
                      @click="exportSearchResults"
                    >
                      导出密语
                    </el-button>
                  </div>

                  <div
                    v-for="item in list"
                    :key="item.id"
                    class="list-item"
                    @click="open(item)"
                  >
                    <div class="item-name">{{ item.name }}</div>
                    <div class="item-company">{{ item.company }}</div>
                  </div>
                </div>
              </template>

              <!-- 目录导航 -->
              <div v-else class="toc-panel">
                <div class="toc-header">
                  <el-button class="back-btn" link @click="backToList">
                    <el-icon><ArrowLeft /></el-icon>
                    <span>返回</span>
                  </el-button>
                </div>

                <div class="toc-list">
                  <div
                    v-for="(item, index) in tocList"
                    :key="index"
                    class="toc-item"
                    :class="`toc-level-${item.level}`"
                    @click="scrollToHeading(item.line)"
                  >
                    {{ item.text }}
                  </div>
                  <div v-if="tocList.length === 0" class="toc-empty">
                    暂无目录
                  </div>
                </div>
              </div>
            </div>

            <!-- 右侧编辑器 -->
            <div class="secret-editor">
              <!-- 标题与公司 一行两列 -->
              <div class="input-row">
                <el-input
                  v-model="current.company"
                  placeholder="请输入公司/平台名称"
                  size="large"
                  class="flex-1"
                />
                <el-input
                  v-model="current.name"
                  placeholder="请输入项目名称"
                  size="large"
                  class="flex-1"
                />
              </div>

              <!-- 时间展示 -->
              <div class="time-info" v-if="current.id">
                <span>创建：{{ current.create_time }}</span>
                <span style="margin-left: 16px">更新：{{ current.update_time }}</span>
              </div>

              <!-- MdEditor 组件 -->
              <MdEditor ref="mdEditorRef" v-model="current.content" />

              <!-- 按钮组：保存 + 删除 -->
              <div class="btn-group">
                <el-button type="primary" size="large" @click="save">
                  保存当前密语
                </el-button>

                <div class="right-btns">
                  <el-button
                    v-if="current.id"
                    class="export-btn-large"
                    size="large"
                    @click="exportSingleCipher"
                  >
                    导出密语
                  </el-button>
                  <el-button
                    type="danger"
                    size="large"
                    @click="deleteItem"
                    v-if="current.id"
                  >
                    删除该密语
                  </el-button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </el-col>
    </el-row>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { save as fileSave, open as fileOpen } from '@tauri-apps/plugin-dialog'
import { writeFile, readFile } from '@tauri-apps/plugin-fs'
import { ArrowLeft } from '@element-plus/icons-vue'
import MdEditor from '@/components/MdEditor.vue'
import { ElMessage, ElMessageBox } from 'element-plus'

const router = useRouter()
const goBack = () => router.push('/')

const list = ref([])
const kw = ref('')
const showExportBtn = ref(false)
const showToc = ref(false)
const tocList = ref([])
const mdEditorRef = ref(null)
const current = ref({
  id: '',
  name: '',
  company: '',
  content: '',
  create_time: '',
  update_time: ''
})

onMounted(async () => {
  await initFirstKey()
  list.value = await invoke('get_all_ciphers', { limit: 10 })
  
  // 添加 Ctrl+S 快捷键
  window.addEventListener('keydown', handleKeydown)
})

// 键盘事件处理
function handleKeydown(e) {
  if ((e.ctrlKey || e.metaKey) && e.key === 's') {
    e.preventDefault()
    save()
  }
}

onUnmounted(() => {
  // 移除快捷键监听
  window.removeEventListener('keydown', handleKeydown)
})

// 监听内容变化，自动更新目录
watch(() => current.value.content, (newContent) => {
  if (showToc.value) {
    generateToc(newContent || '')
  }
})

// 搜索
async function doSearch() {
  list.value = kw.value
    ? await invoke('search_cipher', { keyword: kw.value })
    : await invoke('get_all_ciphers', { limit: 10 })
  showExportBtn.value = !!kw.value && list.value.length > 0
}

// 保存
async function save() {
  if (!current.value.name) {
    ElMessage.warning('请输入项目名称')
    return
  }

  try {
    if (current.value.id) {
      // 更新
      await invoke('update_cipher', {
        cipher: {
          id: Number(current.value.id),
          name: current.value.name,
          company: current.value.company,
          content: current.value.content,
        }
      })
      ElMessage.success('更新成功')
    } else {
      // 新增
      const id = await invoke('add_cipher', {
        cipher: {
          name: current.value.name,
          company: current.value.company,
          content: current.value.content,
        }
      })
      current.value.id = id
      ElMessage.success('新增成功')
      showToc.value = true  // 切换到目录导航
      generateToc(current.value.content || '')  // 生成目录
    }
    // 刷新列表
    list.value = await invoke('get_all_ciphers', { limit: 10 })
  } catch (error) {
    ElMessage.error(current.value.id ? '更新失败：' + (error.message || '未知错误') : '新增失败：' + (error.message || '未知错误'))
  }
}

// 删除
async function deleteItem() {
  if (!current.value.id) return

  try {
    await ElMessageBox.confirm(
      '确定要删除该密语吗？删除后无法恢复！',
      '危险操作',
      {
        confirmButtonText: '确定删除',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )

    // 调用后端删除
    await invoke('delete_cipher', { id: Number(current.value.id) })
    ElMessage.success('删除成功')

    // 清空表单
    create()
    // 刷新列表
    list.value = await invoke('get_all_ciphers', { limit: 10 })
  } catch {
    ElMessage.info('已取消删除')
  }
}

// 新建
function create() {
  current.value = {
    id: '',
    name: '',
    company: '',
    content: '',
    create_time: '',
    update_time: ''
  }
  showToc.value = false
  tocList.value = []
}

// 打开
function open(item) {
  current.value = { ...item }
  if (item.id) {
    current.value.id = String(item.id)  // 确保类型一致
  }
  generateToc(item.content || '')
  showToc.value = true
}

// 生成目录
function generateToc(content) {
  if (!content) {
    tocList.value = []
    return
  }
  const lines = content.split('\n')
  const toc = []
  lines.forEach((line, index) => {
    const match = line.match(/^(#{1,6})\s+(.+)$/)
    if (match) {
      toc.push({
        level: match[1].length,
        text: match[2].trim(),
        line: index + 1
      })
    }
  })
  tocList.value = toc
}

// 返回列表
function backToList() {
  showToc.value = false
  // 清空编辑器内容
  current.value = {
    id: '',
    name: '',
    company: '',
    content: '',
    create_time: '',
    update_time: ''
  }
}

// 滚动到指定标题
function scrollToHeading(lineNumber) {
  mdEditorRef.value?.scrollToLine(lineNumber, false)
}

// 首次使用生成密钥
async function initFirstKey() {
  const exists = await invoke('check_key_exists')
  if (!exists) {
    await invoke('generate_first_key')
    ElMessage.success('首次使用，密钥已生成！请立即备份！')
    await backupKey()
  }
}

// 备份密钥
async function backupKey() {
  const key = await invoke('export_key')
  const path = await fileSave({
    title: '保存密钥文件',
    defaultPath: 'secret.key',
  })
  if (path) {
    await writeFile(path, new Uint8Array(key))
    ElMessage.success('密钥备份成功！此文件可以恢复所有数据')
  }
}

// 导入密钥
async function importKey() {
  const selected = await fileOpen({
    title: '选择密钥文件',
    filters: [{ name: 'Key', extensions: ['key'] }],
  })
  if (selected) {
    const data = await readFile(selected)
    await invoke('import_key', { keyData: Array.from(data) })
    ElMessage.success('密钥导入成功')
  }
}

// 导出搜索结果为Markdown
async function exportSearchResults() {
  if (!showExportBtn.value || list.value.length === 0) return

  const itemsContent = list.value.map(item => {
    return `## ${item.name || '未命名项目'}

${item.content || ''}

---

`
  }).join('\n')

  const markdownContent = `# 项目信息

${itemsContent}`

  const path = await fileSave({
    title: '导出密语',
    defaultPath: `项目信息_${Date.now()}.md`,
    filters: [{ name: 'Markdown', extensions: ['md'] }],
  })
  if (path) {
    try {
      await writeTextFile(path, markdownContent)
      ElMessage.success('导出成功')
      showExportBtn.value = false
      kw.value = ''
    } catch (error) {
      ElMessage.error('导出失败：' + (error.message || '未知错误'))
    }
  }
}

// 导出当前单个密语为Markdown
async function exportSingleCipher() {
  if (!current.value.id) return

  const markdownContent = `# ${current.value.company || '未命名公司'}

## ${current.value.name || '未命名项目'}

${current.value.content || ''}
`

  const path = await fileSave({
    title: '导出密语',
    defaultPath: `${current.value.name || 'cipher'}.md`,
    filters: [{ name: 'Markdown', extensions: ['md'] }],
  })
  if (path) {
    try {
      console.log(markdownContent);
      await writeTextFile(path, markdownContent)
      ElMessage.success('导出成功')
    } catch (error) {
      ElMessage.error('导出失败：' + (error.message || '未知错误'))
    }
  }
}
</script>

<style scoped>
.secret-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #e0f2f1 0%, #b2dfdb 100%);
}

.header-wrapper {
  margin: 20px 0;
  background: rgba(255, 255, 255, 0.6);
  padding: 15px 20px;
  border-radius: 12px;
  backdrop-filter: blur(5px);
}

.page-title {
  font-size: 20px;
  font-weight: 600;
  color: #004d40;
  letter-spacing: 1px;
}

.card-wrapper {
  background: #ffffff;
  border-radius: 16px;
  box-shadow: 0 8px 20px rgba(0, 77, 64, 0.08);
  overflow: hidden;
  border: 1px solid rgba(255, 255, 255, 0.8);
  padding: 20px;
  margin-bottom: 20px;
}

.secret-layout {
  display: flex;
  gap: 20px;
  height: 75vh;
}

.secret-sidebar {
  width: 200px;
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.sidebar-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  overflow-y: auto;
  padding-right: 5px;
}

.list-item {
  padding: 12px 14px;
  background: #f0f9f8;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s;
  color: #00796b;
}

.list-item:hover {
  background: #e0f2f1;
}

.item-name {
  font-weight: 500;
  margin-bottom: 4px;
}

.item-company {
  font-size: 12px;
  opacity: 0.8;
}

/* 目录导航 */
.toc-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  gap: 12px;
}

.toc-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding-bottom: 12px;
  border-bottom: 1px solid #e0f2f1;
}

.back-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  color: #00796b;
  font-size: 14px;
}

.toc-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  overflow-y: auto;
  padding-right: 5px;
}

.toc-item {
  padding: 8px 10px;
  background: #f0f9f8;
  border-radius: 6px;
  color: #00796b;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
  border-left: 3px solid transparent;
}

.toc-item:hover {
  background: #e0f2f1;
  border-left-color: #4fc3f7;
}

.toc-level-1 {
  font-weight: 600;
  font-size: 14px;
}

.toc-level-2 {
  padding-left: 20px;
}

.toc-level-3 {
  padding-left: 32px;
  font-size: 12px;
  opacity: 0.85;
}

.toc-level-4 {
  padding-left: 44px;
  font-size: 12px;
  opacity: 0.75;
}

.toc-level-5,
.toc-level-6 {
  padding-left: 56px;
  font-size: 11px;
  opacity: 0.65;
}

.toc-empty {
  padding: 20px;
  text-align: center;
  color: #90a4ae;
  font-size: 13px;
}

.secret-editor {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 12px;
  min-width: 0;
}

.input-row {
  display: flex;
  gap: 12px;
  width: 100%;
}
.flex-1 {
  flex: 1;
}

.time-info {
  font-size: 12px;
  color: #90a4ae;
  padding: 0 4px;
}

.btn-group {
  display: flex;
  justify-content: space-between;
  width: 100%;
}

.right-btns {
  display: flex;
  gap: 12px;
}

.btn-row {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.btn-row.has-export {
  flex-direction: row;
}

.btn-row.has-export .el-button {
  flex: 1;
}

.export-btn {
  background: linear-gradient(135deg, #81d4fa 0%, #4fc3f7 100%);
  border: none;
  color: #fff;
  font-weight: 500;
  box-shadow: 0 2px 8px rgba(79, 195, 247, 0.3);
  transition: all 0.3s ease;
}

.export-btn:hover {
  background: linear-gradient(135deg, #4fc3f7 0%, #29b6f6 100%);
}

.export-btn-large {
  background: linear-gradient(135deg, #81d4fa 0%, #4fc3f7 100%);
  border: none;
  color: #fff;
  font-weight: 500;
  box-shadow: 0 2px 8px rgba(79, 195, 247, 0.3);
  transition: all 0.3s ease;
}

.export-btn-large:hover {
  background: linear-gradient(135deg, #4fc3f7 0%, #29b6f6 100%);
}
</style>