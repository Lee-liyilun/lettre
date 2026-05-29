import { createRouter, createWebHistory } from 'vue-router'

// 引入你的两个独立页面
import IndexPage from '../views/index.vue'      // 首页/导航页
import CalcPage from '../views/calc/index.vue'   // 简财
import MarkPage from '../views/mark/index.vue'   // 印记 

const router = createRouter({
  history: createWebHistory(),
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
  ]
})

export default router