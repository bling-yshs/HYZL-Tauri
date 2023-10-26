import {createRouter, createWebHistory} from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      redirect: "/start"
    },
    {
      path: "/start",
      component: () => import('../views/StartPage.vue')
    },
    {
      path: "/download",
      component: () => import('../views/DownloadPage.vue')
    },
    {
      path: "/issue-fix",
      component: () => import('../views/IssueFix.vue')
    },
    {
      path: "/debug",
      component: () => import('../views/DebugPage.vue')
    },
    {
      path: "/settings",
      component: () => import('../views/SettingsPage.vue')
    },
    {
      path: "/about",
      component: () => import('../views/AboutPage.vue')
    },
  ]
})

export default router
