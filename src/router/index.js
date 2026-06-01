// src/router/index.js
import { createRouter, createWebHistory } from 'vue-router'

// 引入你的两个独立页面
import IndexPage from '../views/index.vue'      // 首页/导航页
import CalcPage from '../views/calc/index.vue'   // 简财
import MarkPage from '../views/mark/index.vue'   // 印记 
import CipherPage from '../views/cipher/index.vue'   // 密语 

const router = createRouter({
  history: createWebHistory(),
  // 新增：定义滚动行为
  scrollBehavior(to, from, savedPosition) {
    // 始终返回顶部
    return { top: 0 }
  },
  routes: [
    {
      path: '/',
      name: 'Index',
      component: IndexPage
    },
    {
      path: '/calc',
      name: 'Calc',
      component: CalcPage
    },
    {
      path: '/mark',
      name: 'Mark',
      component: MarkPage
    }
    ,
    {
      path: '/cipher',
      name: 'Cipher',
      component: CipherPage
    }
  ]
})

export default router