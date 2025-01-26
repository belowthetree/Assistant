import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";

const { BASE_URL } = import.meta.env
const routes: Array<RouteRecordRaw> = [
    {
        path: "/",
        name: "Home",
        component: () => import('@/App.vue')
    },
    {
        path: "/home",
        name: "Home",
        component: () => import('@/App.vue')
    },
    {
        path: '/setting',
        name: 'setting',
        component: ()=> import('@/view/Setting/SettingView.vue')
    },
    {
        path: '/talk',
        name: 'talk',
        component: () => import("@/view/Talk/TalkView.vue")
    },
    {
        path: '/rolecard',
        name: 'rolecard',
        component: () => import("@/view/RoleCard/RoleCardView.vue")
    }
]

const router = createRouter({
    history: createWebHistory(BASE_URL),
    routes,
});

export default router;