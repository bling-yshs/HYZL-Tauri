import {createRouter, createWebHistory} from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "NormalPage",
      children: [
        {
          path: "/",
          redirect: "/start"
        },
        {
          path: "/start",
          component: () => import('../views/StartPage.vue')
        },
        {
          path: "/issue-fix",
          name: "问题修复",
          component: () => import('../views/IssueFix.vue')
        },
      ]
    },
  ]
})

export default router
