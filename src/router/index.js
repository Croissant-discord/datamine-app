import { createRouter, createWebHashHistory } from 'vue-router'
import HomeView from '../views/Home.vue'
import ProfileView from '../views/Profile.vue'

const routes = [
  {
    path: '/',
    components: {
      default: HomeView,
      profile_info: ProfileView ,
    },
  }
]

const router = createRouter({
  history: createWebHashHistory(), // important for Tauri — no server routing
  routes,
})

export default router