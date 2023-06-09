import { App } from 'vue';
import { createRouter,  createWebHistory } from 'vue-router';


const routes = [
  { path: '/', component: import('../components/Layout.vue') },
  { path: '/config', component: import('../components/Config.vue') }
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
