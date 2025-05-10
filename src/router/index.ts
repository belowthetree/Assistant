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
        component: ()=> import('@/view/setting/SettingView.vue')
    },
    {
        path: '/setting',
        name: 'mcpServerListView',
        component: ()=> import('@/view/setting/MCPServerListView.vue')
    },
    {
        path: '/modelSetting',
        name: 'modelSetting',
        component: ()=> import('@/view/setting/ModelSettingView.vue')
    },
    {
        path: '/talk',
        name: 'talk',
        component: () => import("@/view/Talk/TalkView.vue")
    },
    {
        path: '/rolecard',
        name: 'rolecard',
        component: () => import("@/view/setting/RoleCardSettingView.vue")
    },
    {
        path: '/knowledge',
        name: 'knowledge',
        component: () => import("@/view/Knowledge/KnowledgeView.vue")
    }
]

const router = createRouter({
    history: createWebHistory(BASE_URL),
    routes,
});

export default router;