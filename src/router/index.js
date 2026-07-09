import { createRouter, createWebHashHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Settings from '../views/Settings.vue'

const routes = [
  { path: '/', name: 'home', component: Home },
  { path: '/settings', name: 'settings', component: Settings },
]

const router = createRouter({
  history: createWebHashHistory(), // important for Tauri — no server routing
  routes,
})

export default router