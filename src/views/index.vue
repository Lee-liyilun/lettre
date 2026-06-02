<template>
    <div class="page-container">
        <header class="page-header">
            <h1>笺影</h1>
            <p>保持热爱，奔赴山海</p>
        </header>
        <div class="menu-main">
            <el-row :gutter="20">
                <el-col 
                    v-for="(item, index) in menuList"
                    :key="index"
                    :xs="24" 
                    :sm="12" 
                    :md="8" 
                    :lg="6" 
                    :xl="2"
                    class="menu-col"
                >
                    <el-card 
                        shadow="hover" 
                        class="menu-card"
                        @click="goToPage(item.url)"
                    >
                        <!-- 显示图片和名称 -->
                        <div class="card-content">
                            <img :src="item.image" :alt="item.name" class="card-image" />
                            <span class="card-name">{{ item.name }}</span>
                        </div>
                    </el-card>
                </el-col>
            </el-row>
        </div>

        <!-- 底部页脚 -->
        <footer class="page-footer">
          <p>
            <span>© 2026 {{ author }}</span>
            <span class="divider">|</span>
            <span class="version">v{{ appVersion }}</span>
          </p>
        </footer>
    </div>
</template>

<script setup>
import { useRouter } from 'vue-router' 
import markImg from '@/assets/images/mark.png'
import cipherImg from '@/assets/images/cipher.png'
import saveImg from '@/assets/images/save.png'
import calcImg from '@/assets/images/calc.png'

// 引入 package.json 获取版本号
import packageJson from '@/../package.json'

const router = useRouter() 

const menuList = [
  { name: "印记", url: "/mark", image: markImg },
  { name: "密语", url: "/cipher", image: cipherImg },
  { name: "锦投", url: "/save", image: saveImg },
  { name: "简财", url: "/calc", image: calcImg },
]

const goToPage = (url) => {
  router.push(url) 
}

// 定义作者和获取版本号
const author = "1072966772@qq.com" // 可以在这里修改作者名
const appVersion = packageJson.version
</script>

<style scoped>
/* 1. 页面容器：换成了更有“小清新”感的浅青绿色背景 */
.page-container {
  min-height: 100vh;
  /* 从清新的薄荷绿渐变到稍微深一点的青绿色，更有层次感 */
  background: linear-gradient(135deg, #e0f2f1 0%, #b2dfdb 100%);
  padding-bottom: 40px;
  /* 仅针对当前页面隐藏滚动条 */
  overflow-y: auto;
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE/Edge */
}
.page-container::-webkit-scrollbar {
  display: none; /* Chrome/Safari */
}

/* 2. 头部样式：加深了文字颜色，让它在浅色背景上更突出 */
.page-header {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 70px 0 50px;
  text-align: center;
}

.page-header h1 {
  margin: 0 0 12px;
  font-size: 38px;
  color: #004d40; /* 深墨绿色，色差明显 */
  font-weight: 700;
  letter-spacing: 6px;
}

.page-header p {
  margin: 0;
  font-size: 15px;
  color: #00796b; /* 中绿色 */
  letter-spacing: 3px;
  font-weight: 500;
}
 
.menu-main {
  padding: 10px 40px; /* 增加两侧留白 */
  max-width: 1400px;
  margin: 0 auto;
}

/* 3. 卡片基础样式：纯白背景，与青绿色背景形成鲜明对比 */
.menu-card {
  cursor: pointer;
  margin-bottom: 25px;
  border-radius: 20px !important; /* 更大的圆角，更柔和 */
  border: 1px solid rgba(255, 255, 255, 0.6) !important; /* 加一点半透明白边框，增加精致感 */
  background-color: #ffffff; /* 纯白卡片 */
  box-shadow: 0 8px 20px rgba(0, 77, 64, 0.08); /* 带一点绿色的阴影 */
  transition: all 0.4s cubic-bezier(0.25, 0.8, 0.25, 1);
}

/* 鼠标悬停效果 */
.menu-card:hover {
  transform: translateY(-8px) scale(1.02); /* 上浮并轻微放大 */
  box-shadow: 0 18px 35px rgba(0, 77, 64, 0.15) !important;
}

/* 修改 Element Plus 卡片内边距 */
:deep(.el-card__body) {
  padding: 35px 20px !important;
}

.card-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

/* 4. 图片样式：加了个浅绿色的底托，更有层次 */
.card-image {
  width: 75px;
  height: 75px;
  border-radius: 50%;
  margin-bottom: 18px;
  object-fit: cover;
  transition: transform 0.5s ease;
  background-color: #e0f2f1; /* 图片底色与背景呼应 */
  padding: 5px; /* 增加一点内边距，像相框一样 */
  box-sizing: border-box;
}

.menu-card:hover .card-image {
  transform: rotate(15deg) scale(1.1);
}

.card-name {
  font-size: 17px;
  color: #00695c; /* 深青色文字 */
  font-weight: 600;
  letter-spacing: 2px;
}
/* 底部页脚样式 */
.page-footer {
  text-align: center;
  padding: 20px 0;
  margin-top: 20px;
  color: #00796b; /* 与你页面副标题颜色保持一致 */
  font-size: 13px;
  letter-spacing: 1px;
  opacity: 0.8; /* 轻微透明，更有质感 */
}

.page-footer .divider {
  margin: 0 10px;
  opacity: 0.5;
}

.page-footer .version {
  font-family: monospace; /* 版本号用等宽字体，更有科技感 */
  background-color: rgba(0, 121, 107, 0.1); /* 给版本号加个极淡的背景色 */
  padding: 2px 8px;
  border-radius: 10px;
}
</style>

<!-- 全局滚动条样式（建议单独放在一个全局 CSS 文件里，如果必须放这里请保持这样） -->
<style>
body, html {
  margin: 0;
  padding: 0;
}
/* 仅针对 Webkit 浏览器隐藏垂直滚动条但保留水平 */
body::-webkit-scrollbar {
  width: 0 !important;
  height: 8px !important;
}
body::-webkit-scrollbar-thumb {
  background-color: #b2dfdb;
  border-radius: 4px;
}
body::-webkit-scrollbar-track {
  background-color: #e0f2f1;
}
</style>