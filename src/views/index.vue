<template>
    <div class="page-container">
        <header class="page-header">
            <h1>笺影</h1>
            <p>保持热爱，奔赴山海</p>
        </header>
        <div class="menu-main">
            <el-row :gutter="10">
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
    </div>
</template>

<script setup>
import { useRouter } from 'vue-router' // 引入路由
// 1. 先把 assets 里的图片通过 import 引入进来
import markImg from '@/assets/images/mark.png'
import cipherImg from '@/assets/images/cipher.png'
import saveImg from '@/assets/images/save.png'
import calcImg from '@/assets/images/calc.png'

const router = useRouter() // 获取路由实例

// 定义你的菜单数据
const menuList = [
  { name: "印记", url: "/mark", image: markImg },
  { name: "密语", url: "/cipher", image: cipherImg },
  { name: "锦投", url: "/save", image: saveImg },
  { name: "简财", url: "/calc", image: calcImg },
]

// 定义跳转方法，接收点击项的 url
const goToPage = (url) => {
  router.push(url) // 编程式导航跳转到对应路径
}
</script>

<style>
/* 只隐藏垂直（右侧）滚动条，保留水平（底部）滚动条 */
body, html {
  /* 确保页面可以正常滚动 */
  overflow: auto !important;

  /* 针对 Chrome, Safari, Edge 等 Webkit 内核浏览器 */
  &::-webkit-scrollbar {
    width: 0 !important;      /* 将垂直滚动条的宽度压缩为0，彻底隐藏右侧 */
    height: 12px !important;  /* 明确保留水平滚动条的高度，显示底部 */
  }
  /* 给水平滚动条的滑块加个颜色，防止看不见 */
  &::-webkit-scrollbar-thumb {
    background-color: #ccc;
    border-radius: 6px;
  }
  /* 给水平滚动条的轨道加个底色 */
  &::-webkit-scrollbar-track {
    background-color: #f1f1f1;
  }

  /* 针对 Firefox 浏览器（Firefox 只能整体隐藏，无法单独保留横向） */
  scrollbar-width: none !important;
  /* 针对 IE 和旧版 Edge */
  -ms-overflow-style: none !important;
}
</style>
<style scoped>
/* 1. 修改页面容器，加上渐变背景 */
.page-container {
  min-height: 100vh;
  /* 从极淡的薄荷绿（带了一点透明度）平滑过渡到纯白 */
  background: linear-gradient(to bottom, rgba(216, 243, 220, 0.4), #ffffff);
  padding-bottom: 40px;
}

/* 2. 全局隐藏右侧滚动条（但保留滚动功能） */
/* 兼容 Chrome, Safari, Edge 等 Webkit 内核浏览器 */
.page-container::-webkit-scrollbar {
  display: none; 
}
/* 兼容 Firefox 浏览器 */
.page-container {
  scrollbar-width: none; 
}
/* 兼容 IE 和旧版 Edge */
.page-container {
  -ms-overflow-style: none; 
}

/* 4. .page-header 恢复成普通的头部样式，去掉背景渐变 */
.page-header {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 60px 0 40px;
  text-align: center;
}

/* 标题样式 */
.page-header h1 {
  margin: 0 0 12px;
  font-size: 36px;
  color: #1b4332;
  font-weight: 600;
  letter-spacing: 4px;
}

/* 副标题/标语样式 */
.page-header p {
  margin: 0;
  font-size: 15px;
  color: #52b788;
  letter-spacing: 2px;
  font-weight: 400;
}

/* 卡片基础样式 */
.menu-card {
  cursor: pointer;
  margin-bottom: 20px;
  border-radius: 16px !important;
  border: none !important;
  background-color: #ffffff; /* 纯白卡片在浅绿背景下色差非常明显 */
  box-shadow: 0 6px 16px rgba(27, 67, 50, 0.08);
  transition: all 0.4s ease;
}

/* 鼠标悬停时的卡片效果 */
.menu-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 15px 30px rgba(27, 67, 50, 0.2) !important;
}

/* 穿透修改 Element Plus 卡片默认内边距 */
:deep(.el-card__body) {
  padding: 30px 20px !important;
}

/* 让内容垂直排列并居中 */
.card-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

/* 圆形图片样式 */
.card-image {
  width: 70px;
  height: 70px;
  border-radius: 50%;
  margin-bottom: 16px;
  object-fit: cover;
  transition: transform 0.5s ease;
  background-color: #f0fdf4;
}

/* 鼠标悬停时图片旋转 */
.menu-card:hover .card-image {
  transform: rotate(20deg);
}

/* 文字样式 */
.card-name {
  font-size: 16px;
  color: #40916c;
  font-weight: 500;
  letter-spacing: 1px;
}
</style>