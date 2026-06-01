<template>
  <div class="secret-container">
    <!-- 头部区域 -->
    <el-row :gutter="20">
      <el-col :span="18" :offset="3">
        <div class="header-wrapper">
          <el-page-header @back="goBack">
            <template #content>
              <span class="page-title">密语</span>
            </template>
          </el-page-header>
        </div>
      </el-col>
    </el-row>

    <!-- 主体卡片区域 -->
    <el-row :gutter="20">
      <el-col :span="18" :offset="3">
        <div class="card-wrapper">
          <div class="secret-layout">
            <!-- 左侧搜索 + 列表 -->
            <div class="secret-sidebar">
              <el-input
                v-model="kw"
                placeholder="搜索项目名称或内容"
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
                  {{ item.name }}
                </div>
              </div>
            </div>

            <!-- 右侧编辑器 -->
            <div class="secret-editor">
              <el-input
                v-model="current.name"
                placeholder="请输入项目名称"
                size="large"
              />

              <!-- MdEditor 组件（你放在 components 里的） -->
              <MdEditor v-model="current.content" />

              <el-button type="primary" size="large" @click="save">
                保存当前密语
              </el-button>
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
import MdEditor from '@/components/MdEditor.vue'

const router = useRouter()
const goBack = () => router.push('/')

const list = ref([])
const kw = ref('')
const current = ref({ id: '', name: '', content: '' })

onMounted(async () => {
  list.value = await invoke('list_secrets')
})

async function doSearch() {
  if (!kw.value) {
    list.value = await invoke('list_secrets')
    return
  }
  list.value = await invoke('search_secrets', { keyword: kw.value })
}

async function save() {
  if (!current.value.name) {
    ElMessage.warning('请输入项目名称')
    return
  }

  if (current.value.id) {
    await invoke('update_secret', {
      id: Number(current.value.id),
      name: current.value.name,
      content: current.value.content,
    })
  } else {
    const id = await invoke('add_secret', {
      name: current.value.name,
      content: current.value.content,
    })
    current.value.id = id
  }
  doSearch()
}

function create() {
  current.value = { id: '', name: '新建密语', content: '' }
}

function open(item) {
  current.value = { ...item }
}
</script>

<style scoped>
/* 整体容器（和简财完全一样） */
.secret-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #e0f2f1 0%, #b2dfdb 100%);
  padding-bottom: 60px;
  padding-top: 20px;
}

/* 头部 */
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

/* 卡片主体 */
.card-wrapper {
  background: #ffffff;
  border-radius: 16px;
  box-shadow: 0 8px 20px rgba(0, 77, 64, 0.08);
  overflow: hidden;
  border: 1px solid rgba(255, 255, 255, 0.8);
  padding: 25px;
}

/* 左右布局 */
.secret-layout {
  display: flex;
  gap: 20px;
  height: 70vh;
}

/* 左侧 */
.secret-sidebar {
  width: 240px;
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

/* 右侧 */
.secret-editor {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
</style>