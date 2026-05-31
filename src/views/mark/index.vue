<template>
  <div class="mark-container">
    <!-- 头部区域 -->
    <el-row :gutter="20">
      <el-col :span="18" :offset="3">
        <div class="header-wrapper">
          <el-page-header @back="goBack">
            <template #content>
              <span class="page-title">印记</span>
            </template>
          </el-page-header>
        </div>
      </el-col>
    </el-row>
    
    <!-- 标签页区域 -->
    <el-row :gutter="20">
      <el-col :span="18" :offset="3">
        <div class="tabs-wrapper">
          <el-tabs type="border-card" class="fresh-tabs" v-model="activeTab">
            
            <!-- Tab 1: 印记 -->
            <el-tab-pane label="印记" name="marks">
              <template #label>
                <span class="custom-tabs-label">
                  <span class="tab-icon">📝</span>
                  <span>印记</span>
                </span>
              </template>
              <div class="tab-content">
                <!-- 添加印记表单 -->
                <div class="form-section" id="mark-form-top">
                  <h3 class="section-title">{{ isEditingMark ? '编辑印记' : '添加新印记' }}</h3>
                  <el-form :model="markForm" label-width="100px" class="fresh-form">
                    <el-form-item label="选择项目：">
                      <el-select 
                        v-model="markForm.project_id" 
                        placeholder="请选择项目" 
                        filterable
                        style="width: 100%"
                      >
                        <el-option
                          v-for="project in projects"
                          :key="project.project_id"
                          :label="project.name"
                          :value="project.project_id"
                        />
                      </el-select>
                    </el-form-item>
                    <el-form-item label="分类：">
                      <el-select v-model="markForm.category_id" placeholder="请选择分类" style="width: 100%">
                        <el-option label="需求" :value="1" />
                        <el-option label="Bug" :value="2" />
                        <el-option label="其他" :value="3" />
                      </el-select>
                    </el-form-item>
                    <el-form-item label="内容：">
                      <el-input 
                        v-model="markForm.content" 
                        type="textarea" 
                        :rows="3"
                        placeholder="请输入印记内容"
                      />
                    </el-form-item>
                    <el-form-item>
                      <el-button v-if="!isEditingMark" type="success" class="fresh-btn fresh-btn-success" @click="handleAddMark">添加印记</el-button>
                      <template v-else>
                        <el-button type="warning" class="fresh-btn fresh-btn-warning" @click="handleUpdateMark">保存</el-button>
                        <el-button @click="cancelEditMark">取消</el-button>
                      </template>
                    </el-form-item>
                  </el-form>
                </div>

                <!-- 印记列表 -->
                <div class="list-section">
                  <el-empty v-if="marks.length === 0" description="暂无印记" />
                  <div v-else class="mark-list">
                    <el-card 
                      v-for="mark in marks" 
                      :key="mark.mark_id" 
                      class="mark-card"
                      shadow="hover"
                    >
                      <div class="mark-inner">
                        <div class="mark-header">
                          <el-tag :type="getCategoryType(mark.category_id)" size="small">
                            {{ getCategoryName(mark.category_id) }}
                          </el-tag>
                          <span class="mark-time">{{ formatTime(mark.create_time) }}</span>
                        </div>
                        <div class="mark-project">
                          <strong>{{ getProjectName(mark.project_id) }}</strong>
                        </div>
                        <div class="mark-content">{{ mark.content }}</div>
                      </div>
                      <div class="mark-actions">
                        <el-button type="primary" size="small" @click="handleEditMark(mark)">编辑</el-button>
                        <el-button type="danger" size="small" @click="handleDeleteMark(mark.mark_id)">删除</el-button>
                      </div>
                    </el-card>
                  </div>
                </div>
              </div>
            </el-tab-pane>

            <!-- Tab 2: 项目 -->
            <el-tab-pane label="项目" name="projects">
              <template #label>
                <span class="custom-tabs-label">
                  <span class="tab-icon">📁</span>
                  <span>项目</span>
                </span>
              </template>
              <div class="tab-content">
                <!-- 添加/编辑项目表单 -->
                <div class="form-section"  id="project-form-top">
                  <h3 class="section-title">{{ isEditing ? '编辑项目' : '添加新项目' }}</h3>
                  <el-form :model="projectForm" label-width="100px" class="fresh-form">
                    <el-form-item label="项目名称：">
                      <el-input 
                        v-model="projectForm.name" 
                        placeholder="请输入项目名称"
                      />
                    </el-form-item>
                    <el-form-item>
                      <el-button 
                        v-if="!isEditing" 
                        type="primary" 
                        class="fresh-btn fresh-btn-primary" 
                        @click="handleAddProject"
                      >
                        添加项目
                      </el-button>
                      <template v-else>
                        <el-button type="warning" class="fresh-btn fresh-btn-warning" @click="handleUpdateProject">保存</el-button>
                        <el-button @click="cancelEdit">取消</el-button>
                      </template>
                    </el-form-item>
                  </el-form>
                </div>

                <!-- 项目列表 -->
                <div class="list-section">
                  <el-empty v-if="projects.length === 0" description="暂无项目" />
                  <div v-else class="project-list">
                    <el-row :gutter="10">
                      <el-col 
                          v-for="project in projects" 
                          :key="project.project_id" 
                          :xs="24" 
                          :sm="12" 
                          :md="8" 
                          :lg="6" 
                          :xl="2"
                          class="project-col"
                      >
                        <el-card 
                          class="project-card"
                          shadow="hover"
                        >
                          <div class="project-info">
                            <div class="project-name">{{ project.name }}</div>
                            <div class="project-time">{{ formatTime(project.create_time) }}</div>
                          </div>
                          <div class="project-actions">
                            <el-button type="primary" size="small" @click="handleEditProject(project)">编辑</el-button>
                            <el-button type="danger" size="small" @click="handleDeleteProject(project.project_id)">删除</el-button>
                          </div>
                        </el-card>
                      </el-col>
                    </el-row>
                  </div>
                </div>
              </div>
            </el-tab-pane>

            <!-- Tab 3: 总览 -->
            <el-tab-pane label="总览" name="overview">
              <template #label>
                <span class="custom-tabs-label">
                  <span class="tab-icon">📊</span>
                  <span>总览</span>
                </span>
              </template>
              <div class="tab-content">
                <div class="overview-section">
                  <el-empty v-if="weeklyData.length === 0" description="暂无数据" />
                  <div v-else class="week-wrap">
                    <div
                      v-for="(week, weekIdx) in weeklyData"
                      :key="weekIdx"
                      class="week-card"
                    >
                      <div class="week-date-title">
                        <span>{{ week.week_start }} - {{ week.week_end }}</span>
                        <div class="copy-btn-wrap" @click="copyWeekContent(week)">
                          📋 复制本周内容
                        </div>
                      </div>

                      <div
                        class="project-wrap"
                        v-for="project in week.projects"
                        :key="project.project_id"
                      >
                        <div class="project-name">{{ project.project_name }}</div>

                        <div
                          class="category-wrap"
                          v-for="category in project.categories"
                          :key="category.category_id"
                        >
                          <span class="category-label" :class="getCategoryClass(category.category_id)">
                            {{ getCategoryName(category.category_id) }}
                          </span>
                          <div class="content-list">
                            <div 
                              class="content-line"
                              v-for="(item, idx) in category.contents" 
                              :key="idx"
                            >
                              {{ idx + 1 }}、{{ item }}
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </el-tab-pane>

          </el-tabs>
        </div>
      </el-col>
    </el-row>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessageBox, ElMessage } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'

// 获取路由实例
const router = useRouter()

// 返回首页方法
const goBack = () => {
  router.push('/')
}

// 当前激活的标签页
const activeTab = ref('marks')

// ========== 项目相关数据和方法 ==========
const projects = ref([])
const projectForm = reactive({
  project_id: null,
  name: ''
})
const isEditing = ref(false)

// 获取所有项目
const loadProjects = async () => {
  try {
    const result = await invoke('get_all_projects')
    projects.value = result
  } catch (error) {
    ElMessage.error('加载项目失败：' + error)
  }
}

// 添加项目
const handleAddProject = async () => {
  if (!projectForm.name.trim()) {
    ElMessageBox.alert('请输入项目名称', '提示', {
      type: 'warning',
      center: true,
    })
    return
  }
  
  try {
    await invoke('add_project', { name: projectForm.name })
    ElMessage.success('添加成功')
    projectForm.name = ''
    await loadProjects()
  } catch (error) {
    ElMessage.error('添加失败：' + error)
  }
}

// 编辑项目
const handleEditProject = (project) => {
  projectForm.project_id = project.project_id
  projectForm.name = project.name
  isEditing.value = true
  // --- 新增：滚动到表单区域顶部 ---
  const formElement = document.getElementById('project-form-top')
  if (formElement) {
    formElement.scrollIntoView({ 
      behavior: 'smooth', 
      block: 'start' 
    })
  }
}

// 更新项目
const handleUpdateProject = async () => {
  if (!projectForm.name.trim()) {
    ElMessageBox.alert('请输入项目名称', '提示', {
      type: 'warning',
      center: true,
    })
    return
  }
  
  try {
    await invoke('update_project', { 
      projectId: projectForm.project_id, 
      name: projectForm.name 
    })
    ElMessage.success('更新成功')
    cancelEdit()
    await loadProjects()
  } catch (error) {
    ElMessage.error('更新失败：' + error)
  }
}

// 取消编辑
const cancelEdit = () => {
  projectForm.project_id = null
  projectForm.name = ''
  isEditing.value = false
}

// 删除项目
const handleDeleteProject = async (projectId) => {
  try {
    await ElMessageBox.confirm('确定要删除该项目吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    })
    
    await invoke('delete_project', { projectId })
    ElMessage.success('删除成功')
    await loadProjects()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败：' + error)
    }
  }
}

// ========== 印记相关数据和方法 ==========
const marks = ref([])
const markForm = reactive({
  project_id: null,
  category_id: null,
  content: ''
})
const isEditingMark = ref(false)

// 获取所有印记
const loadMarks = async () => {
  try {
    const result = await invoke('get_all_marks')
    marks.value = result
  } catch (error) {
    ElMessage.error('加载印记失败：' + error)
  }
}

// 添加印记
const handleAddMark = async () => {
  if (!markForm.project_id || !markForm.category_id || !markForm.content.trim()) {
    ElMessageBox.alert('请填写完整的印记信息', '提示', {
      type: 'warning',
      center: true,
    })
    return
  }
  
  try {
    await invoke('add_mark', {
      mark: { 
        project_id: markForm.project_id,
        category_id: markForm.category_id,
        content: markForm.content 
      }
    })
    ElMessage.success('添加成功')
    markForm.project_id = null
    markForm.category_id = null
    markForm.content = ''
    await loadMarks()
  } catch (error) {
    ElMessage.error('添加失败：' + error)
  }
}

// 编辑印记
const handleEditMark = (mark) => {
  markForm.mark_id = mark.mark_id
  markForm.project_id = mark.project_id
  markForm.category_id = mark.category_id
  markForm.content = mark.content
  isEditingMark.value = true
  // --- 新增：滚动到表单区域顶部 ---
  const formElement = document.getElementById('mark-form-top')
  if (formElement) {
    formElement.scrollIntoView({ 
      behavior: 'smooth', 
      block: 'start' 
    })
  }
}

// 更新印记
const handleUpdateMark = async () => {
  if (!markForm.mark_id) {
    ElMessageBox.alert('缺少主键无法更新印记信息', '提示', {
      type: 'warning',
      center: true,
    })
    return
  }
  
  try {
    await invoke('update_mark', {
      mark: { 
        mark_id: markForm.mark_id, 
        project_id: markForm.project_id,
        category_id: markForm.category_id,
        content: markForm.content
      }
    })
    ElMessage.success('更新成功')
    cancelEditMark()
    await loadMarks()
  } catch (error) {
    ElMessage.error('更新失败：' + error)
  }
}

// 取消编辑
const cancelEditMark = () => {
  markForm.mark_id = null
  markForm.project_id = null
  markForm.category_id = null
  markForm.content = ''
  isEditingMark.value = false
}

// 删除印记
const handleDeleteMark = async (markId) => {
  try {
    await ElMessageBox.confirm('确定要删除该印记吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    })
    
    await invoke('delete_mark', { markId })
    ElMessage.success('删除成功')
    await loadMarks()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败：' + error)
    }
  }
}

// 获取分类名称
const getCategoryName = (categoryId) => {
  const categories = { 1: '需求', 2: 'Bug', 3: '其他' }
  return categories[categoryId] || '未知'
}

// 获取分类标签类型
const getCategoryType = (categoryId) => {
  const types = { 1: 'success', 2: 'danger', 3: 'info' }
  return types[categoryId] || ''
}

// 获取项目名称
const getProjectName = (projectId) => {
  const project = projects.value.find(p => p.project_id === projectId)
  return project ? project.name : '未知项目'
}

// 格式化时间
const formatTime = (timeStr) => {
  if (!timeStr) return ''
  return timeStr.replace('T', ' ').substring(0, 19)
}

// ========== 总览相关数据和方法 ==========
const weeklyData = ref([])

// 获取每周总览数据
const loadWeeklyOverview = async () => {
  try {
    const result = await invoke('get_weekly_overview')
    
    // 处理数据
    const processedData = result.map(week => ({
      ...week,
      projects: week.projects.map(project => ({
        ...project,
        categories: project.categories.map(cat => ({
          ...cat,
          // 确保 contents 是数组，如果后端返回 null 则给空数组
          contents: Array.isArray(cat.contents) ? cat.contents : []
        }))
      }))
    }));
    console.log('processedData:', processedData);
    weeklyData.value = processedData;
  } catch (error) {
    ElMessage.error('加载总览数据失败：' + error)
  }
}

// 分类样式class
const getCategoryClass = (categoryId) => {
  return {
    1: 'cat-demand',
    2: 'cat-bug',
    3: 'cat-other'
  }[categoryId] || ''
}

// 一键复制整周内容
const copyWeekContent = (week) => {
  let text = ''
  
  // 拼接复制文本（和你要的格式完全一致）
  week.projects.forEach(project => {
    text += project.project_name + '\n'
    
    project.categories.forEach(category => {
      text += getCategoryName(category.category_id) + '\n'
      
      category.contents.forEach((item, idx) => {
        text += `${idx + 1}、${item}\n`
      })
    })
    text += '\n'
  })

  // 复制到剪贴板
  navigator.clipboard.writeText(text.trim()).then(() => {
    ElMessage.success('复制成功！')
  }).catch(() => {
    ElMessage.error('复制失败，请手动复制')
  })
}

// 组件挂载时加载数据
onMounted(() => {
  loadProjects()
  loadMarks()
  loadWeeklyOverview()
})

// 👇 监听 Tab 切换
watch(activeTab, (newTab) => {
  if (newTab === 'overview') {
    loadWeeklyOverview()
  }
})

</script>

<style scoped>
/* 页面容器：使用与首页一致的渐变背景 */
.mark-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #e0f2f1 0%, #b2dfdb 100%);
  padding-bottom: 60px;
  padding-top: 20px;
}

/* 头部包装器：增加与下方内容的间距 */
.header-wrapper {
  margin-bottom: 25px;
  background: rgba(255, 255, 255, 0.6);
  padding: 15px 20px;
  border-radius: 12px;
  backdrop-filter: blur(5px);
}

/* 页面标题样式：深绿色主题 */
.page-title {
  font-size: 20px;
  font-weight: 600;
  color: #004d40;
  letter-spacing: 1px;
}

/* 标签页包装器：白色卡片样式 */
.tabs-wrapper {
  background: #ffffff;
  border-radius: 16px;
  box-shadow: 0 8px 20px rgba(0, 77, 64, 0.08);
  overflow: hidden;
  border: 1px solid rgba(255, 255, 255, 0.8);
}

/* 覆盖Element Plus标签页默认样式 */
:deep(.fresh-tabs.el-tabs--border-card) {
  border: none;
  box-shadow: none;
  background: transparent;
}

/* 标签页头部样式 */
:deep(.fresh-tabs .el-tabs__header) {
  background-color: #f0f9f8;
  border-bottom: 1px solid #e0f2f1;
  margin: 0;
}

/* 标签项样式 */
:deep(.fresh-tabs .el-tabs__item) {
  color: #00796b;
  font-weight: 500;
  transition: all 0.3s ease;
}

/* 激活的标签项样式 */
:deep(.fresh-tabs .el-tabs__item.is-active) {
  color: #004d40;
  background-color: #ffffff;
  border-right-color: #e0f2f1;
  border-left-color: #e0f2f1;
}

/* 标签页内容区域 */
:deep(.fresh-tabs .el-tabs__content) {
  padding: 25px;
  background-color: #ffffff;
}

/* 自定义标签样式（带图标） */
.custom-tabs-label {
  display: flex;
  align-items: center;
  gap: 6px;
}

/* 标签图标样式 */
.tab-icon {
  font-size: 16px;
}

/* 表单内容区域 */
.tab-content {
  max-width: 900px;
  margin: 0 auto;
  padding-top: 10px;
}

/* 分区标题 */
.section-title {
  font-size: 18px;
  color: #004d40;
  margin-bottom: 20px;
  padding-bottom: 10px;
  border-bottom: 2px solid #e0f2f1;
  font-weight: 600;
}

/* 表单区域 */
.form-section {
  margin-bottom: 30px;
  padding: 20px;
  background: linear-gradient(135deg, #f0f9f8 0%, #e8f5f3 100%);
  border-radius: 12px;
  border: 1px solid #e0f2f1;
}

.fresh-form {
  padding: 10px;
}

/* 列表区域 */
.list-section {
  margin-top: 30px;
}

/* 印记卡片 */
.mark-list {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.mark-card {
  border-radius: 12px;
  border: 1px solid #e0f2f1;
  transition: all 0.3s ease;
  display: flex;          /* 启用弹性盒子 */
  flex-direction: column; /* 纵向排列 */
  height: 100%;           /* 高度占满父容器 */
}

.mark-inner {
  flex: 1; /* 关键：这个div会自动变大，把.mark-actions推到最底部 */
  display: flex;
  flex-direction: column;
}

.mark-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(0, 77, 64, 0.12);
}

.mark-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
}

.mark-time {
  font-size: 12px;
  color: #999;
}

.mark-project {
  margin-bottom: 8px;
  color: #00796b;
  font-size: 15px;
}

.mark-content {
  color: #333;
  line-height: 1.6;
  font-size: 14px;
}

.mark-actions {
  display: flex;
  justify-content: flex-end; /* 关键：靠右对齐 */
  gap: 10px;                 /* 按钮之间的间距 */
}

/* 项目卡片 */
.project-list {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.project-col {
  padding-top: 5px;
  padding-bottom: 5px;
}

.project-card {
  border-radius: 12px;
  border: 1px solid #e0f2f1;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px 20px;
  transition: all 0.3s ease;
  background-color: #F0F8FF;
}

.project-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(0, 77, 64, 0.12);
}

.project-info {
  flex: 1;
}

.project-name {
  font-size: 16px;
  color: #004d40;
  font-weight: 600;
  margin-bottom: 5px;
}

.project-time {
  font-size: 12px;
  color: #999;
}

.project-actions {
  display: flex;
  gap: 10px;
}

/* 总览区域 - 小清新主题 */
.overview-section {
  padding: 10px;
}
.week-wrap {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

/* 周卡片：温柔浅白卡片 */
.week-card {
  background: #fafcfb;
  border-radius: 16px;
  padding: 22px 26px;
  box-shadow: 0 2px 8px rgba(0, 150, 136, 0.06);
  border: 1px solid #e0f2f1;
}

/* 周标题 */
.week-date-title {
  font-size: 16px;
  font-weight: 600;
  color: #00695c;
  padding-bottom: 12px;
  margin-bottom: 20px;
  border-bottom: 1px solid #b2dfdb;
}

/* 项目名称 */
.project-wrap {
  margin-bottom: 20px;
}
.project-name {
  font-size: 15px;
  font-weight: 600;
  color: #00796b;
  margin-bottom: 10px;
}

/* 分类容器 */
.category-wrap {
  margin-bottom: 12px;
  padding-left: 6px;
}

/* 小清新分类标签 */
.category-label {
  display: inline-block;
  padding: 3px 10px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  margin-bottom: 6px;
  border: 1px solid transparent;
}

/* 需求：薄荷绿 */
.category-label.cat-demand {
  background: #e6f7f5;
  color: #3a9f96;
  border-color: #c1ede8;
}

/* Bug：淡粉橘 */
.category-label.cat-bug {
  background: #ffeef1;
  color: #f17082;
  border-color: #fcd4dc;
}

/* 其他：淡紫灰 */
.category-label.cat-other {
  background: #f5f8ff;
  color: #748cb3;
  border-color: #e1e8f7;
}

/* 内容列表 */
.content-list {
  padding-left: 14px;
}
.content-line {
  font-size: 14px;
  color: #444;
  line-height: 1.75;
  margin: 4px 0;
}

/* 周标题 + 复制按钮 */
.week-date-title {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 16px;
  font-weight: 600;
  color: #00695c;
  padding-bottom: 12px;
  margin-bottom: 20px;
  border-bottom: 1px solid #b2dfdb;
}

/* 自定义复制按钮 —— 完美垂直居中，无任何偏移 */
.copy-btn-wrap {
  height: 26px;
  line-height: 26px;
  padding: 0 12px;
  border-radius: 20px;
  font-size: 12px;
  background: #87e8de;
  color: #086f6f;
  border: 1px solid #87e8de;
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  user-select: none;
  transition: all 0.2s;
}
.copy-btn-wrap:hover {
  background: #6cd0c4;
  border-color: #6cd0c4;
}


/* 按钮基础样式：统一圆角和过渡效果 */
:deep(.fresh-btn) {
  border-radius: 8px;
  padding: 10px 24px;
  font-weight: 500;
  transition: all 0.3s ease;
}

/* Primary按钮：薄荷绿主题 */
:deep(.fresh-btn-primary.el-button--primary) {
  background-color: #58c9b9;
  border-color: #58c9b9;
}

:deep(.fresh-btn-primary.el-button--primary:hover) {
  background-color: #4ab8a8;
  border-color: #4ab8a8;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(88, 201, 185, 0.3);
}

/* Success按钮：清新绿色 */
:deep(.fresh-btn-success.el-button--success) {
  background-color: #76d49c;
  border-color: #76d49c;
}

:deep(.fresh-btn-success.el-button--success:hover) {
  background-color: #65c48b;
  border-color: #65c48b;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(118, 212, 156, 0.3);
}

/* Warning按钮：柔和橙色 */
:deep(.fresh-btn-warning.el-button--warning) {
  background-color: #f8c377;
  border-color: #f8c377;
}

:deep(.fresh-btn-warning.el-button--warning:hover) {
  background-color: #e8b367;
  border-color: #e8b367;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(248, 195, 119, 0.3);
}
</style>
