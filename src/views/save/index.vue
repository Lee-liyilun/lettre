<template>
  <div class="save-container">
    <!-- 头部区域 -->
    <el-row>
      <el-col :span="20" :offset="2">
        <div class="header-wrapper">
          <el-page-header @back="goBack">
            <template #content>
              <span class="page-title">锦投</span>
            </template>
          </el-page-header>
        </div>
      </el-col>
    </el-row>

    <!-- 卡片列表区域 -->
    <el-row v-if="!showIframe">
      <el-col :span="20" :offset="2">
        <div class="cards-wrapper">
          <div v-loading="loading" element-loading-text="加载中..." class="cards-grid">
            <div
              v-for="(item, index) in stockList"
              :key="index"
              class="stock-card"
              @click="openLink(item.link)"
            >
              <div class="card-header">
                <h3 class="stock-name">{{ item.name }}</h3>
                <span class="stock-tag" :class="getValuationClass(item.eva_type)">{{ formatValuation(item.eva_type) }}</span>
              </div>
              <div class="card-body">
                <div class="info-row">
                  <span class="info-label">PE值：</span>
                  <span class="info-value" :class="getPEClass(item.pe)">{{ item.pe.toFixed(2) }}</span>
                </div>
                <div class="info-row">
                  <span class="info-label">PE百分位：</span>
                  <span class="info-value" :class="getPercentileClass(item.pe_percentile)">
                    {{ item.pe_percentile.toFixed(2) }}%
                  </span>
                </div>
              </div>
              <div class="card-footer">
                <el-icon class="link-icon"><Link /></el-icon>
                <span class="link-text">点击查看详情</span>
              </div>
            </div>
          </div>
        </div>
      </el-col>
    </el-row>

    <!-- iframe 显示区域 -->
    <el-row v-else>
      <el-col :span="20" :offset="2">
        <div class="iframe-wrapper">
          <div class="iframe-header">
            <el-button type="primary" @click="closeIframe" class="back-btn">
              <el-icon><ArrowLeft /></el-icon>
              返回列表
            </el-button>
          </div>
          <div class="iframe-content">
            <iframe :src="currentLink" frameborder="0" class="stock-iframe"></iframe>
          </div>
        </div>
      </el-col>
    </el-row>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ArrowLeft, Link } from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'

// 获取路由实例
const router = useRouter()

// 返回首页方法
const goBack = () => {
  router.push('/')
}

// 是否显示 iframe
const showIframe = ref(false)

// 当前链接
const currentLink = ref('')

// 股票列表数据
const stockList = ref([])

// 是否加载中
const loading = ref(true)

// 加载股票数据
const loadStockList = async () => {
  loading.value = true
  try {
    const data = await invoke('get_stock_list')
    stockList.value = data
  } catch (error) {
    ElMessage.error('获取股票数据失败：' + (error.message || '未知错误'))
  } finally {
    loading.value = false
  }
}

// 打开链接
const openLink = (link) => {
  currentLink.value = link
  showIframe.value = true
}

// 关闭 iframe
const closeIframe = () => {
  showIframe.value = false
  currentLink.value = ''
}

// 根据 PE 值返回样式类
const getPEClass = (pe) => {
  if (pe < 30) return 'pe-low'
  if (pe < 70) return 'pe-medium'
  return 'pe-high'
}

// 根据 PE 百分位返回样式类
const getPercentileClass = (percentile) => {
  if (percentile < 30) return 'percentile-low'
  if (percentile < 70) return 'percentile-medium'
  return 'percentile-high'
}

// 格式化估值类型
const formatValuation = (evaType) => {
  const map = {
    'high': '高估',
    'mid': '合理',
    'low': '低估'
  }
  return map[evaType] || evaType
}

// 获取估值标签样式类
const getValuationClass = (evaType) => {
  const classMap = {
    'high': 'tag-high',
    'mid': 'tag-mid',
    'low': 'tag-low'
  }
  return classMap[evaType] || ''
}

// 页面加载时获取数据
onMounted(() => {
  loadStockList()
})
</script>

<style scoped>
/* 页面容器：使用与 calc 一致的渐变背景 */
.save-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #e0f2f1 0%, #b2dfdb 100%);
}

/* 头部包装器 */
.header-wrapper {
  margin: 25px 0;
  background: rgba(255, 255, 255, 0.6);
  padding: 15px 20px;
  border-radius: 12px;
  backdrop-filter: blur(5px);
}

/* 页面标题样式 */
.page-title {
  font-size: 20px;
  font-weight: 600;
  color: #004d40;
  letter-spacing: 1px;
}

/* 卡片包装器 */
.cards-wrapper {
  background: #ffffff;
  border-radius: 16px;
  box-shadow: 0 8px 20px rgba(0, 77, 64, 0.08);
  overflow: hidden;
  border: 1px solid rgba(255, 255, 255, 0.8);
  margin-bottom: 20px;
  padding: 25px;
}

/* 加载状态 */
.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 0;
  gap: 15px;
}

.loading-text {
  font-size: 16px;
  color: #00796b;
}

.is-loading {
  animation: rotating 2s linear infinite;
}

@keyframes rotating {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

/* 卡片网格布局 */
.cards-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 20px;
  min-height: 400px;
}

/* 单个股票卡片 */
.stock-card {
  background: linear-gradient(135deg, #f0f9f8 0%, #ffffff 100%);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid #e0f2f1;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 2px 8px rgba(0, 77, 64, 0.05);
}

.stock-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 6px 16px rgba(0, 77, 64, 0.15);
  border-color: #58c9b9;
}

/* 卡片头部 */
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
  padding-bottom: 12px;
  border-bottom: 1px solid #e0f2f1;
}

/* 股票名称 */
.stock-name {
  font-size: 16px;
  font-weight: 600;
  color: #004d40;
  margin: 0;
}

/* 估值标签 */
.stock-tag {
  padding: 4px 10px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
  background: #58c9b9;
  color: #ffffff;
}

/* 高估标签 - 纯红色 */
.tag-high {
  background: #ff0000;
  color: #ffffff;
}

/* 合理标签 - 纯黄色 */
.tag-mid {
  background: #ffcc00;
  color: #333333;
}

/* 低估标签 - 纯绿色 */
.tag-low {
  background: #00cc00;
  color: #ffffff;
}

/* 卡片主体 */
.card-body {
  margin-bottom: 15px;
}

/* 信息行 */
.info-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
  font-size: 14px;
}

.info-label {
  color: #666;
  font-weight: 500;
}

.info-value {
  font-weight: 600;
  color: #00796b;
}

/* PE 值样式 */
.pe-low {
  color: #67c23a;
}

.pe-medium {
  color: #e6a23c;
}

.pe-high {
  color: #f56c6c;
}

/* PE 百分位样式 */
.percentile-low {
  color: #67c23a;
}

.percentile-medium {
  color: #e6a23c;
}

.percentile-high {
  color: #f56c6c;
}

/* 卡片底部 */
.card-footer {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  color: #58c9b9;
  font-size: 13px;
}

.link-icon {
  font-size: 16px;
}

.link-text {
  font-weight: 500;
}

/* iframe 包装器 */
.iframe-wrapper {
  background: #ffffff;
  border-radius: 16px;
  box-shadow: 0 8px 20px rgba(0, 77, 64, 0.08);
  overflow: hidden;
  border: 1px solid rgba(255, 255, 255, 0.8);
  margin-bottom: 20px;
  height: 75vh;
  display: flex;
  flex-direction: column;
}

/* iframe 头部 */
.iframe-header {
  padding: 15px 20px;
  background: #f0f9f8;
  border-bottom: 1px solid #e0f2f1;
}

/* 返回按钮 */
.back-btn {
  background: #58c9b9;
  border-color: #58c9b9;
  color: #ffffff;
  border-radius: 8px;
  padding: 8px 20px;
  font-weight: 500;
  transition: all 0.3s ease;
}

.back-btn:hover {
  background: #4ab8a8;
  border-color: #4ab8a8;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(88, 201, 185, 0.3);
}

/* iframe 内容区域 */
.iframe-content {
  flex: 1;
  overflow: hidden;
}

/* iframe 样式 */
.stock-iframe {
  width: 100%;
  height: 100%;
  border: none;
}
</style>