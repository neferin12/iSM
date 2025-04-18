import {createRouter, createWebHashHistory, createWebHistory} from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
    history: createWebHashHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'home',
            component: HomeView
        },
        {
            path: '/results',
            name: 'results',
            component: () => import('../views/Results.vue')
        },
        {
            path: '/run/normal',
            name: 'run-normal',
            component: () => import('../views/NormalRun.vue')
        },
        {
            path: '/run/model-checking',
            name: 'run-model-checking',
            component: () => import('../views/ModelCheckingRun.vue')
        },
        {
            path: '/about',
            name: 'about',
            // route level code-splitting
            // this generates a separate chunk (About.[hash].js) for this route
            // which is lazy-loaded when the route is visited.
            component: () => import('../views/AboutView.vue')
        }
    ]
})

export default router
