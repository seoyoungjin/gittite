import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('../views/AboutView.vue')
    },
    {
      path: '/gitcmd/init',
      name: 'git-init',
      component: () => import('../views/gitcmd/Init.vue')
    },
    {
      path: '/gitcmd/clone',
      name: 'git-clone',
      component: () => import('../views/gitcmd/Clone.vue')
    },
    {
      path: '/gitcmd/status',
      name: 'git-tatus',
      component: () => import('../views/gitcmd/Status.vue')
    },
    {
      path: '/gitcmd/logs',
      name: 'git-logs',
      component: () => import('../views/gitcmd/Logs.vue')
    },
    {
      path: '/gitcmd/branch',
      name: 'git-branch',
      component: () => import('../views/gitcmd/Branch.vue')
    },
    {
      path: '/gitcmd/tags',
      name: 'git-tags',
      component: () => import('../views/gitcmd/Tags.vue')
    },
    {
      path: '/gitcmd/stash',
      name: 'git-stash',
      component: () => import('../views/gitcmd/Stash.vue')
    },
    {
      path: '/gitcmd/addremove',
      name: 'git-add-remove',
      component: () => import('../views/gitcmd/AddRemove.vue')
    },
    {
      path: '/gitcmd/remote',
      name: 'git-remote',
      component: () => import('../views/gitcmd/Remote.vue')
    },
    {
      path: '/gitcmd/settings',
      name: 'git-settings',
      component: () => import('../views/gitcmd/Settings.vue')
    }
  ]
})

export default router
