<script lang="ts">
import ModelSettingView from "~/src/view/setting/ModelSettingView.vue";
import MCPServerListView from "./MCPServerListView.vue";
import RoleCardSettingView from "./RoleCardSettingView.vue";

export default {
    components: {
        'modelsetting': ModelSettingView,
        'serversetting': MCPServerListView,
        'rolecardsetting': RoleCardSettingView,
    },
    setup() {
    },
    data() {
        return {
            activeTab: "modeltab"
        }
    },
    mounted() {
    },
    methods: {
        onClickTab(tab) {
            this.activeTab = tab
        }
    }
}
</script>

<template>
    <div id="mainbody" class="settings-container">
        <!-- Left Tabs -->
        <div class="tabs-sidebar">
            <button 
                @click="onClickTab('modeltab')" 
                :class="['tab-btn', { 'active': activeTab === 'modeltab' }]">
                <i class="fa-solid fa-globe" style="font-size: 20px;"></i>
                <span class="tab-text">模型</span>
            </button>
            <button 
                @click="onClickTab('servertab')" 
                :class="['tab-btn', { 'active': activeTab === 'servertab' }]">
                <i class="fa-solid fa-server" style="font-size: 20px;"></i>
                <span class="tab-text">服务</span>
            </button>
            <button 
                @click="onClickTab('rolecardtab')" 
                :class="['tab-btn', { 'active': activeTab === 'rolecardtab' }]">
                <i class="fa-solid fa-server" style="font-size: 20px;"></i>
                <span class="tab-text">角色</span>
            </button>
        </div>

        <!-- Right Content Area -->
        <div class="content-area">
            <div v-show="activeTab === 'modeltab'" class="tab-content" :class="{ 'active': activeTab === 'modeltab' }">
                <h2 class="content-title">模型</h2>
                <modelsetting></modelsetting>
            </div>

            <div v-show="activeTab === 'servertab'" class="tab-content" :class="{ 'active': activeTab === 'servertab' }">
                <h2 class="content-title">MCP 服务</h2>
                <serversetting></serversetting>
            </div>

            <div v-show="activeTab === 'rolecardtab'" class="tab-content" :class="{ 'active': activeTab === 'rolecardtab' }">
                <h2 class="content-title">角色</h2>
                <rolecardsetting></rolecardsetting>
            </div>
        </div>
    </div>
</template>

<style scoped>
* {
    
    /* 隐藏滚动条箭头 - Windows */
    scrollbar-width: thin; /* Firefox */
    scrollbar-color: transparent transparent; /* Firefox */
}

.settings-container {
    background-color: white;
    overflow: hidden;
    width: 100%;
    margin: 0;
    height: 100%;
    display: flex;
    flex-direction: column;
}

.tabs-sidebar {
    background-color: #f9fafb; /* bg-gray-50 */
    width: 100%;
    border-bottom: 1px solid #e5e7eb; /* border-gray-200 */
    display: flex;
    flex-direction: row;
}

.tab-btn {
    padding: 1rem;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    text-align: left;
    border-right: 1px solid #e5e7eb;
    transition: background-color 0.2s ease-in-out;
    cursor: pointer;
    border: none;
    background: none;
    width: 100%;
}

.tab-btn:hover {
    background-color: #f3f4f6; /* hover:bg-gray-100 */
}

.tab-btn.active {
    background-color: #3b82f6; /* bg-blue-500 */
    color: white;
    border-color: #3b82f6;
}

.tab-text {
    display: inline;
    font-size: 19px;
}

.content-area {
    padding: 1.5rem;
    width: 100%;
}

.content-title {
    font-size: 1.5rem;
    line-height: 2rem;
    font-weight: 700;
    color: #1f2937; /* text-gray-800 */
    margin-bottom: 1rem;
}

.tab-content {
    display: none;
    animation: fadeIn 0.3s ease-in-out;
}

.tab-content.active {
    display: block;
}

@keyframes fadeIn {
    from {
        opacity: 0;
        transform: translateY(10px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

/* 响应式设计 - 桌面端 */
@media (min-width: 768px) {
    .settings-container {
        flex-direction: row;
    }
    
    .tabs-sidebar {
        width: 12rem; /* w-48 */
        border-bottom: none;
        border-right: 1px solid #e5e7eb;
        flex-direction: column;
    }
    
    .tab-btn {
        border-right: none;
        border-bottom: 1px solid #e5e7eb;
    }
}

/* 暗黑模式 */
.dark .settings-container {
    background-color: #111827; /* dark:bg-gray-900 */
}

.dark .tabs-sidebar {
    background-color: #1f2937; /* dark:bg-gray-800 */
    border-color: #374151; /* dark:border-gray-700 */
}

.dark .content-title {
    color: #e5e7eb; /* dark:text-gray-200 */
}

.dark .tab-btn:hover {
    background-color: #374151; /* dark:hover:bg-gray-700 */
}

.dark .tab-btn.active {
    background-color: #3b82f6; /* dark:bg-blue-500 */
    border-color: #3b82f6;
}
</style>