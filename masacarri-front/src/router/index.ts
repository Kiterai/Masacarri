import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: () => import('../views/HomeView.vue')
    },
    {
      path: '/admin',
      name: 'admin',
      component: () => import('../views/AdminView.vue')
    },
    {
      path: '/pages/:page_id',
      name: 'comment',
      component: () => import('../views/CommentView.vue'),
      props: true,
    },
    {
      path: '/admin/pages/:page_id',
      name: 'admin_commentpage',
      component: () => import('../views/CommentView.vue'),
      props: (route) => {
        return {
          page_id: route.params.page_id,
          is_admin: true,
        }
      }
    },
  ]
})

export default router
