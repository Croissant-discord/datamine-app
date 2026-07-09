import { createRouter, createWebHashHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Settings from '../views/Settings.vue'
import About from '../views/About.vue'

const routes = [
  { path: '/', name: 'home', component: Home },
  { path: '/settings', name: 'settings', component: Settings },
  { path: '/about', name: 'about', component: About },
]

const router = createRouter({
  history: createWebHashHistory(), // important for Tauri — no server routing
  routes,
})

export default router