import { createRouter, createWebHistory, createWebHashHistory } from 'vue-router'
import Main from '@/views/Main/index.vue'
import Login from '@/views/Login.vue'
import { useUserStore } from '@/stores/userStore'

const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: '/login',
    },
    {
      path: '/login',
      name: 'login',
      component: Login,
      meta: {
        noAuth: true
      }
    },
    {
      path: '/main',
      name: 'main',
      component: Main,
      redirect: '/main/problems',
      children: [
        {
          path: '/main/problems',
          name: 'main-problems',
          component: () => import('@/views/Main/Problems.vue')
        },
        {
          path: '/main/videos',
          name: 'main-videos',
          component: () => import('@/views/Main/Videos.vue')
        },
      ]

    },
    {
      path: '/pratice',
      name: 'pratice',
      component: () => import('@/views/Practice/index.vue'),
      redirect: '/pratice/problem-detail',
      children: [
        {
          path: 'problem-detail',
          name: 'pratice-problem-detail',
          component: () => import('@/views/Practice/ProblemDetail.vue')
        },
        {
          path: 'blog-detail',
          name: 'pratice-blog-detail',
          component: () => import('@/views/Practice/BlogDetail.vue')
        },
        // 新增AI问答路由
        {
          path: 'ai-detail',
          name: 'pratice-ai-detail',
          component: () => import('@/views/Practice/AIDetail.vue')
        },
      ]
    },
  ],
})

router.beforeEach((to, _from, next) => {
  if (to.meta.noAuth) {
    if (useUserStore().getToken())
      next('/main/problems')
    else
      next()
  } else {
    if (useUserStore().getToken())
      next()
    else
      next('/login')
  }
})

export default router
