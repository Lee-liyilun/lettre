<template>
  <div class="secret-container">
    <!-- 头部区域 -->
    <el-row :gutter="20">
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
    <el-row :gutter="20">
      <el-col :span="22" :offset="1">
        <div class="card-wrapper">
          <div class="secret-layout">
            <!-- 左侧搜索 + 列表 -->
            <div class="secret-sidebar">
              <el-input
                v-model="kw"
                placeholder="搜索项目名称/公司/内容"
                @input="doSearch"
                clearable
              />

              <div class="sidebar-list">
                <el-button type="primary" block @click="create">
                  新建密语
                </el-button>

                <div
                  v-for="item in list"
                  :key="item.id"
                  class="list-item"
                  :class="{ active: current.id === item.id }"
                  @click="open(item)"
                >
                  <div class="item-name">{{ item.name }}</div>
                  <div class="item-company">{{ item.company }}</div>
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
              <MdEditor v-model="current.content" />

              <!-- 按钮组：保存 + 删除 -->
              <div class="btn-group">
                <el-button type="primary" size="large" @click="save">
                  保存当前密语
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
      </el-col>
    </el-row>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { save as fileSave, open as fileOpen } from '@tauri-apps/plugin-dialog'
import { writeFile, readFile } from '@tauri-apps/plugin-fs'
import MdEditor from '@/components/MdEditor.vue'
import { ElMessage, ElMessageBox } from 'element-plus'

const router = useRouter()
const goBack = () => router.push('/')

const list = ref([])
const kw = ref('')
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
})

// 搜索
async function doSearch() {
  list.value = kw.value
    ? await invoke('search_cipher', { keyword: kw.value })
    : await invoke('get_all_ciphers', { limit: 10 })
}

// 保存
async function save() {
  if (!current.value.name) {
    ElMessage.warning('请输入项目名称')
    return
  }

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
  }
  // 刷新列表
  list.value = await invoke('get_all_ciphers', { limit: 10 })
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
}

// 打开
function open(item) {
  current.value = { ...item }
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
</script>

<style scoped>
.secret-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #e0f2f1 0%, #b2dfdb 100%);
  padding: 20px 0 60px;
}

.header-wrapper {
  margin-bottom: 25px;
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
  padding: 25px;
}

.secret-layout {
  display: flex;
  gap: 20px;
  height: 70vh;
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

.list-item.active {
  background: #00796b;
  color: #fff;
}

.item-name {
  font-weight: 500;
  margin-bottom: 4px;
}

.item-company {
  font-size: 12px;
  opacity: 0.8;
}

.secret-editor {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 12px;
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
</style>