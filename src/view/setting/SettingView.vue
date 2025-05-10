<script lang="ts">
import ModelSettingView from "~/src/view/setting/ModelSettingView.vue";
import MCPServerListView from "./MCPServerListView.vue";

export default {
    components: {
        'modelsetting': ModelSettingView,
        'serversetting': MCPServerListView
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
    <div id="mainbody" class="bg-white overflow-hidden w-full max-w-4xl flex flex-col md:flex-row">
        <!-- Left Tabs -->
        <div class="bg-gray-50 w-full md:w-48 border-b md:border-b-0 md:border-r border-gray-200 flex flex-row md:flex-col">
            <button @click="onClickTab('modeltab')" :class="['tab-btn p-4 flex items-center gap-3 text-left border-r md:border-r-0 md:border-b border-gray-200 hover:bg-gray-100 transition-colors duration-200',
                { 'active': activeTab === 'modeltab' }]">
                <i class="fa-solid fa-globe"></i>
                <span class="hidden md:inline">模型</span>
            </button>
            <button @click="onClickTab('rolecardtab')" :class="['tab-btn p-4 flex items-center gap-3 text-left border-r md:border-r-0 md:border-b border-gray-200 hover:bg-gray-100 transition-colors duration-200',
                { 'active': activeTab === 'rolecardtab' }]">
                <i class="fa-solid fa-server"></i>
                <span class="hidden md:inline">服务</span>
            </button>
        </div>

        <!-- Right Content Area -->
        <div class="flex-1 p-6">
            <!-- Tab 1 Content -->
            <div v-show="activeTab === 'modeltab'" class="tab-content" :class="{ 'active': activeTab === 'modeltab' }">
                <h2 class="text-2xl font-bold text-gray-800 mb-4">模型</h2>
                <modelsetting></modelsetting>
            </div>

            <!-- Tab 2 Content -->
            <div v-show="activeTab === 'rolecardtab'" class="tab-content" :class="{ 'active': activeTab === 'rolecardtab' }">
                <h2 class="text-2xl font-bold text-gray-800 mb-4">MCP 服务</h2>
                <serversetting></serversetting>
            </div>
        </div>
    </div>
</template>

<style>
#mainbody {
    margin: 0;
    height: 100%;
    width: 100%;
}

.tab-content {
    display: none;
    animation: fadeIn 0.3s ease-in-out;
}

.tab-content.active {
    display: block;
}

.tab-btn.active {
    background-color: #3b82f6;
    color: white;
    border-color: #3b82f6;
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

.window {
    box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
}

/* 暗黑模式样式 */
.dark {
    --tw-bg-opacity: 1;
    background-color: rgb(17 24 39 / var(--tw-bg-opacity));
}

.dark .window {
    background-color: rgb(31 41 55 / var(--tw-bg-opacity));
    border-color: rgb(55 65 81 / var(--tw-border-opacity));
}

.dark .bg-gray-50 {
    background-color: rgb(31 41 55 / var(--tw-bg-opacity));
}

.dark .text-gray-800 {
    color: rgb(229 231 235 / var(--tw-text-opacity));
}

.dark .border-gray-200 {
    border-color: rgb(55 65 81 / var(--tw-border-opacity));
}

/* 响应式调整 */
@media (max-width: 767px) {
    .tab-btn {
        flex: 1;
        justify-content: center;
    }

    .tab-btn span {
        display: none;
    }
}
</style>