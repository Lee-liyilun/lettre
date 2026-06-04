<template>
  <div class="calc-container">
    <!-- 头部区域 -->
    <el-row>
      <el-col :span="20" :offset="2">
        <div class="header-wrapper">
          <el-page-header @back="goBack">
            <template #content>
              <span class="page-title">简财</span>
            </template>
          </el-page-header>
        </div>
      </el-col>
    </el-row>
    
    <!-- 标签页区域 -->
    <el-row>
      <el-col :span="20" :offset="2">
        <div class="tabs-wrapper">
          <el-tabs type="border-card" class="fresh-tabs">
            
            <!-- Tab 1: 最终收益计算器 -->
            <el-tab-pane label="最终收益计算器">
              <template #label>
                <span class="custom-tabs-label">
                  <span class="tab-icon">💰</span>
                  <span>最终收益计算器</span>
                </span>
              </template>
              <div class="tab-content">
                <el-form label-width="120px" class="fresh-form">
                  <el-form-item label="本金(元)：">
                    <el-input-number v-model="budgetData.principal" placeholder="请输入本金" style="width: 60%" />
                  </el-form-item>
                  <el-form-item label="年化收益率(%)：">
                    <el-input-number v-model="budgetData.rate" placeholder="请输入年化收益率" style="width: 60%" />
                  </el-form-item>
                  <el-form-item label="投资年限(年)：">
                    <el-input-number v-model="budgetData.years" placeholder="请输入投资年限" style="width: 60%" />
                  </el-form-item>
                  <el-form-item>
                    <el-button type="info" class="fresh-btn fresh-btn-info" @click="calculateBudget">计算收益</el-button>
                  </el-form-item>
                </el-form>
                <div class="result" v-if="budgetResult">
                  <p>最终收益：<strong>{{ budgetResult.toFixed(2) }}元</strong></p>
                </div>
              </div>
            </el-tab-pane>

            <!-- Tab 2: 收益率计算器 -->
            <el-tab-pane label="收益率计算器">
              <template #label>
                <span class="custom-tabs-label">
                  <span class="tab-icon">📈</span>
                  <span>收益率计算器</span>
                </span>
              </template>
              <div class="tab-content">
                <el-form label-width="120px" class="fresh-form">
                  <el-form-item label="买入价格(元)：">
                    <el-input-number v-model="rateData.buyPrice" placeholder="请输入买入价格" style="width: 60%" />
                  </el-form-item>
                  <el-form-item label="卖出价格(元)：">
                    <el-input-number v-model="rateData.sellPrice" placeholder="请输入卖出价格" style="width: 60%" />
                  </el-form-item>
                  <el-form-item label="持有年限(年)：">
                    <el-input-number v-model="rateData.holdYears" placeholder="请输入持有年限" style="width: 60%" />
                  </el-form-item>
                  <el-form-item>
                    <el-button type="info" class="fresh-btn fresh-btn-info" @click="calculateRate">计算收益率</el-button>
                  </el-form-item>
                </el-form>
                <div class="result" v-if="rateResult">
                  <p>总收益率：<strong :class="rateResult.totalRate >= 0 ? 'success' : 'error'">{{ rateResult.totalRate }}%</strong></p>
                  <p>年化收益率：<strong :class="rateResult.annualRate >= 0 ? 'success' : 'error'">{{ rateResult.annualRate }}%</strong></p>
                </div>
              </div>
            </el-tab-pane>

            <!-- Tab 3: 定投计算器 -->
            <el-tab-pane label="定投计算器">
              <template #label>
                <span class="custom-tabs-label">
                  <span class="tab-icon">🔁</span>
                  <span>定投计算器</span>
                </span>
              </template>
              <div class="tab-content">
                <el-form label-width="120px" class="fresh-form">
                  <el-form-item label="投资时长(年)：">
                    <el-input-number v-model="savingsData.years" placeholder="请输入投资时长" style="width: 60%" />
                  </el-form-item>
                  <el-form-item label="年化收益率(%)：">
                    <el-input-number v-model="savingsData.rate" placeholder="请输入年化收益率" style="width: 60%" />
                  </el-form-item>
                  <el-form-item label="每月存入(元)：">
                    <el-input-number v-model="savingsData.monthlyAmount" placeholder="请输入金额" style="width: 60%" />
                  </el-form-item>
                  <el-form-item>
                    <el-button type="info" class="fresh-btn fresh-btn-info" @click="calculateSavings">计算结果</el-button>
                  </el-form-item>
                </el-form>
                <div class="result" v-if="savingsResult">
                  <p>投资总收益：<strong>{{ savingsResult.toFixed(2) }}元</strong></p>
                </div>
              </div>
            </el-tab-pane>

            <!-- Tab 4: 财务自由计算器 -->
            <el-tab-pane label="财务自由计算器">
              <template #label>
                <span class="custom-tabs-label">
                  <span class="tab-icon">🗝️</span>
                  <span>财务自由计算器</span>
                </span>
              </template>
              <div class="tab-content">
                <el-form label-width="120px" class="fresh-form">
                  <el-form-item label="本金(元)：">
                    <el-input-number v-model="freedomData.principal" placeholder="请输入本金" style="width: 60%" />
                  </el-form-item>
                  <el-form-item label="月定投(元)：">
                    <el-input-number v-model="freedomData.monthlyInvest" placeholder="请输入月定投" style="width: 60%" />
                  </el-form-item>
                  <el-form-item label="年化收益(%)：">
                    <el-input-number v-model="freedomData.rate" placeholder="请输入年化收益" style="width: 60%" />
                  </el-form-item>
                  <el-form-item label="月支出(元)：">
                    <el-input-number v-model="freedomData.monthlyExpense" placeholder="请输入月支出" style="width: 60%" />
                  </el-form-item>
                  <el-form-item>
                    <el-button type="info" class="fresh-btn fresh-btn-info" @click="calculateFreedom">计算财务自由目标</el-button>
                  </el-form-item>
                </el-form>
                <div class="result" v-if="freedomResult">
                  <p>月均获得收益：<strong>{{ freedomResult.monthlyIncome.toFixed(2) }}元</strong></p>
                  <p>财务自由时间：<strong>{{ freedomResult.months }} 个月（约 {{ freedomResult.years }} 年 {{ freedomResult.remainingMonths }} 个月）</strong></p>
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
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessageBox } from 'element-plus'

// 获取路由实例
const router = useRouter()

// 返回首页方法
const goBack = () => {
  router.push('/')
}

// ========== 最终收益计算器数据和方法 ==========
const budgetData = reactive({
  principal: 0,      // 本金
  rate: 0,           // 年化收益率
  years: 0           // 投资年限
})
const budgetResult = ref(null)

// 计算最终收益
const calculateBudget = () => {
  // TODO: 实现复利计算公式
  // 公式：最终收益 = 本金 * (1 + 年化收益率)^投资年限
  const { principal, rate, years } = budgetData
  
  if (!principal || !rate || !years) {
    ElMessageBox.alert('请填写完整的计算信息', '提示', {
      type: 'warning',
      center: true,
    })
    return
  }
  
  // 临时返回示例值，后续需要实现真实计算
  const result = principal * Math.pow(1 + rate / 100, years)
  budgetResult.value = result
}

// ========== 收益率计算器数据和方法 ==========
const rateData = reactive({
  buyPrice: 0,       // 买入价格
  sellPrice: 0,      // 卖出价格
  holdYears: 0       // 持有年限
})
const rateResult = ref(null)

// 计算收益率
const calculateRate = () => {
  // TODO: 实现收益率计算公式
  // 总收益率 = (卖出价格 - 买入价格) / 买入价格 * 100%
  // 年化收益率 = ((1 + 总收益率)^(1/持有年限) - 1) * 100%
  const { buyPrice, sellPrice, holdYears } = rateData
  
  if (!buyPrice || !sellPrice || !holdYears) {
    ElMessageBox.alert('请填写完整的计算信息', '提示', {
      type: 'warning',
      center: true,
    })
    return
  }
  
  // 临时返回示例值，后续需要实现真实计算
  const totalRatePercent = ((sellPrice - buyPrice) / buyPrice * 100).toFixed(2)
  const annualRatePercent = (Math.pow(1 + (sellPrice - buyPrice) / buyPrice, 1 / holdYears) - 1) * 100
  
  rateResult.value = {
    totalRate: totalRatePercent,
    annualRate: annualRatePercent.toFixed(2)
  }
}

// ========== 定投计算器数据和方法 ==========
const savingsData = reactive({
  years: 0,              // 投资时长
  rate: 0,               // 年化收益率
  monthlyAmount: 0       // 每月存入金额
})
const savingsResult = ref(null)

// 计算定投收益
const calculateSavings = () => {
  // TODO: 实现定投复利计算公式
  // 公式：FV = FV = P * (1+r) * ((1+r)^n - 1) / r
  const { years, rate, monthlyAmount } = savingsData
  
  if (!years || !rate || !monthlyAmount) {
    ElMessageBox.alert('请填写完整的计算信息', '提示', {
      type: 'warning',
      center: true,
    })
    return
  }
  
  // 临时返回示例值，后续需要实现真实计算
  const monthlyRate = Math.pow(1 + rate/100, 1/12) - 1;   // 月利率（名义利率法）
  const months = years * 12
  const futureValue = monthlyAmount * (1 + monthlyRate) * (Math.pow(1 + monthlyRate, months) - 1) / monthlyRate;
  
  savingsResult.value = futureValue
}

// ========== 财务自由计算器数据和方法 ==========
const freedomData = reactive({
  principal: 0,          // 本金
  monthlyInvest: 0,      // 月定投
  rate: 0,               // 年化收益
  monthlyExpense: 0      // 月支出
})
const freedomResult = ref(null)

// 计算财务自由
const calculateFreedom = () => {
  // TODO: 实现财务自由计算逻辑
  // 需要计算达到财务自由所需的时间
  // 财务自由条件：被动收入 >= 月支出
  const { principal, monthlyInvest, rate, monthlyExpense } = freedomData
  
  if (!principal && !monthlyInvest) {
    ElMessageBox.alert('请至少填写本金或月定投金额', '提示', {
      type: 'warning',
      center: true,
    })
    return
  }
  
  if (!monthlyExpense) {
    ElMessageBox.alert('请填写月支出金额', '提示', {
      type: 'warning',
      center: true,
    })
    return
  }
  
  const r = Math.pow(1 + rate * 0.01, 1/12) - 1;
  let n = 1;
  let asset = 0;
  let monthlyIncome = 0;

  while (true) {
      const growthFactor = Math.pow(1 + r, n);
      // 月初定投：定投参与当月复利
      asset = principal * growthFactor + monthlyInvest * (1 + r) * (growthFactor - 1) / r;
      monthlyIncome = asset * r;
      if (monthlyIncome >= monthlyExpense) break;
      n++;
  }
  
  freedomResult.value = {
    monthlyIncome: monthlyIncome,
    months: n,
    years: Math.floor(n / 12),
    remainingMonths: n % 12
  }
}
</script>

<style scoped>
/* 页面容器：使用与首页一致的渐变背景 */
.calc-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #e0f2f1 0%, #b2dfdb 100%);
}

/* 头部包装器：增加与下方内容的间距 */
.header-wrapper {
  margin: 25px 0;
  background: rgba(255, 255, 255, 0.6);
  padding: 15px 20px;
  border-radius: 12px;
  backdrop-filter: blur(5px); /* 轻微磨砂玻璃效果 */
}

/* 页面标题样式：深绿色主题 */
.page-title {
  font-size: 20px;
  font-weight: 600;
  color: #004d40; /* 深墨绿色，与首页标题一致 */
  letter-spacing: 1px;
}

/* 标签页包装器：白色卡片样式 */
.tabs-wrapper {
  background: #ffffff;
  border-radius: 16px;
  box-shadow: 0 8px 20px rgba(0, 77, 64, 0.08); /* 轻柔的绿色阴影 */
  overflow: hidden; /* 保持圆角整洁 */
  border: 1px solid rgba(255, 255, 255, 0.8);
  margin-bottom: 20px;
}

/* 覆盖Element Plus标签页默认样式 */
:deep(.fresh-tabs.el-tabs--border-card) {
  border: none;
  box-shadow: none;
  background: transparent;
}

/* 标签页头部样式 */
:deep(.fresh-tabs .el-tabs__header) {
  background-color: #f0f9f8; /* 浅青色背景 */
  border-bottom: 1px solid #e0f2f1;
  margin: 0;
}

/* 标签项样式 */
:deep(.fresh-tabs .el-tabs__item) {
  color: #00796b; /* 中绿色文字 */
  font-weight: 500;
  transition: all 0.3s ease;
}

/* 激活的标签项样式 */
:deep(.fresh-tabs .el-tabs__item.is-active) {
  color: #004d40; /* 深绿色 */
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
  max-width: 600px;
  margin: 0 auto;
  padding-top: 10px;
}

/* 表单样式 */
.fresh-form {
  padding: 10px;
}

/* 按钮基础样式：统一圆角和过渡效果 */
:deep(.fresh-btn) {
  border-radius: 8px;
  padding: 10px 24px;
  font-weight: 500;
  transition: all 0.3s ease;
}

/* Primary按钮：薄荷绿主题（定投计算器） */
:deep(.fresh-btn-primary.el-button--primary) {
  background-color: #58c9b9; /* 薄荷绿主色 */
  border-color: #58c9b9;
}

:deep(.fresh-btn-primary.el-button--primary:hover) {
  background-color: #4ab8a8;
  border-color: #4ab8a8;
  transform: translateY(-2px); /* 轻微上浮 */
  box-shadow: 0 4px 12px rgba(88, 201, 185, 0.3); /* 增强的阴影 */
}

/* Success按钮：清新绿色（最终收益计算器） */
:deep(.fresh-btn-success.el-button--success) {
  background-color: #76d49c; /* 清新绿色 */
  border-color: #76d49c;
}

:deep(.fresh-btn-success.el-button--success:hover) {
  background-color: #65c48b;
  border-color: #65c48b;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(118, 212, 156, 0.3);
}

/* Warning按钮：柔和橙色（收益率计算器） */
:deep(.fresh-btn-warning.el-button--warning) {
  background-color: #f8c377; /* 柔和橙色 */
  border-color: #f8c377;
}

:deep(.fresh-btn-warning.el-button--warning:hover) {
  background-color: #e8b367;
  border-color: #e8b367;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(248, 195, 119, 0.3);
}

/* Info按钮：淡雅蓝色（财务自由计算器） */
:deep(.fresh-btn-info.el-button--info) {
  background-color: #a9b3d2; /* 淡雅蓝色 */
  border-color: #a9b3d2;
}

:deep(.fresh-btn-info.el-button--info:hover) {
  background-color: #99a3c2;
  border-color: #99a3c2;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(169, 179, 210, 0.3);
}

/* 输入框数字对齐优化 */
:deep(.el-input-number .el-input__inner) {
  text-align: left;
}

/* 结果显示区域样式 */
.result {
  margin-top: 20px;
  padding: 15px 20px;
  background: linear-gradient(135deg, #f0f9f8 0%, #e0f2f1 100%);
  border-radius: 8px;
  border-left: 4px solid #58c9b9;
}

.result p {
  margin: 8px 0;
  font-size: 15px;
  color: #004d40;
  line-height: 1.6;
}

.result strong {
  font-size: 18px;
  color: #00796b;
}

/* 成功和错误状态样式 */
.success {
  color: #67c23a !important;
}

.error {
  color: #f56c6c !important;
}
</style>
