<template>
    <div class="bg-gray-100 min-h-screen p-6">
        <div class="max-w-4xl mx-auto">
            <div class="flex justify-between items-center mb-8">
                <h1 class="text-2xl font-bold text-gray-800">MCP 服务</h1>
                <button @click="showModal"
                    class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-md transition">
                    <i class="fas fa-plus mr-2"></i>添加 MCP 服务
                </button>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <!-- Service Cards -->
                <div v-for="(service, index) in services" :key="index" class="card bg-white rounded-lg overflow-hidden">
                    <div class="p-4">
                        <div class="flex justify-between items-start">
                            <div>
                                <h3 class="text-lg font-semibold text-gray-800">{{ service.config.name }}</h3>
                                <div class="flex items-center mt-1">
                                    <span class="status-badge" :class="{'status-online': service.connected, 'status-offline': !service.connected}"></span>
                                    <span class="text-sm text-gray-600">
                                        {{ capitalizeFirstLetter(service.connected ? "已连接" : "未连接" ) }}
                                    </span>
                                </div>
                            </div>
                            <div class="flex space-x-2">
                                <button @click="editService(index)"
                                    class="text-gray-400 hover:text-blue-500 transition">
                                    <i class="fas fa-edit"></i>
                                </button>
                                <label class="relative inline-flex items-center cursor-pointer">
                                    <input type="checkbox" v-model="service.config.enable" @change="toggleServiceStatus(index)"
                                        class="sr-only peer">
                                    <div
                                        class="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-500">
                                    </div>
                                </label>
                            </div>
                        </div>

                        <div class="mt-2">
                            <p class="text-sm text-gray-500 mb-1 text-left">Start Command</p>
                            <div class="bg-gray-50 p-2 rounded-md " style="overflow-x: hidden;">
                                <code class="text-sm text-gray-700 text-left text-nowrap" style="overflow-x: hidden;">{{ service.config.command }}</code>
                            </div>
                        </div>

                        <!-- <div class="mt-4 flex space-x-2">
                            <button @click="startService(index)"
                                class="flex-1 bg-green-100 hover:bg-green-200 text-green-700 py-2 px-4 rounded-md text-sm transition">
                                <i class="fas fa-play mr-2"></i>Start
                            </button>
                            <button @click="stopService(index)"
                                class="flex-1 bg-red-100 hover:bg-red-200 text-red-700 py-2 px-4 rounded-md text-sm transition">
                                <i class="fas fa-stop mr-2"></i>Stop
                            </button>
                            <button @click="restartService(index)"
                                class="flex-1 bg-blue-100 hover:bg-blue-200 text-blue-700 py-2 px-4 rounded-md text-sm transition">
                                <i class="fas fa-sync-alt mr-2"></i>Restart
                            </button>
                        </div> -->
                    </div>
                </div>

                <!-- Empty Card Slot -->
                <div @click="showModal" class="card add-card rounded-lg p-5 min-h-[200px] cursor-pointer">
                    <div class="text-center">
                        <i class="fas fa-plus-circle text-4xl text-gray-400 mb-2"></i>
                        <p class="text-gray-500">添加 MCP 服务</p>
                    </div>
                </div>
            </div>
        </div>

        <!-- Add Service Modal -->
        <div v-if="showAddModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
            <div class="bg-white rounded-lg p-6 w-full max-w-md">
                <div class="flex justify-between items-center mb-4">
                    <h3 class="text-lg font-semibold">
                        {{ editingIndex === null ? '添加 MCP 服务' : '编辑 MCP 服务' }}
                    </h3>
                    <button @click="closeModal" class="text-gray-500 hover:text-gray-700">
                        <i class="fas fa-times"></i>
                    </button>
                </div>
                <div class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1 text-left">服务名字</label>
                        <input type="text" v-model="newService.name"
                            class="w-full px-3 py-2 border border-gray-300 rounded-md">
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1 text-left">描述</label>
                        <input type="text" v-model="newService.desc"
                            class="w-full px-3 py-2 border border-gray-300 rounded-md">
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1 text-left">命令</label>
                        <input type="text" v-model="newService.command"
                            class="w-full px-3 py-2 border border-gray-300 rounded-md">
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1 text-left">参数</label>
                        <input type="text" v-model="newService.args"
                            class="w-full px-3 py-2 border border-gray-300 rounded-md">
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1 text-left">环境变量</label>
                        <input type="text" v-model="newService.env"
                            class="w-full px-3 py-2 border border-gray-300 rounded-md text-wrap" placeholder="">
                    </div>
                    <!-- <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">Initial Status</label>
                        <select v-model="newService.status" class="w-full px-3 py-2 border border-gray-300 rounded-md">
                            <option value="online">Online</option>
                            <option value="offline">Offline</option>
                            <option value="pending">Pending</option>
                        </select>
                    </div> -->
                </div>
                <div class="mt-6 flex justify-end space-x-3">
                    <button @click="closeModal"
                        class="px-4 py-2 border border-gray-300 rounded-md text-gray-700 hover:bg-gray-50">
                        Cancel
                    </button>
                    <button @click="confirmAdd" class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600">
                        {{ editingIndex === null ? 'Add Service' : 'Update Service' }}
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';


export default {
    name: 'ServiceDashboard',
    data() {
        return {
            showAddModal: false,
            editingIndex: null,
            newService: {
                name: '',
                desc: '',
                command: '',
                args: '',
                env:'',
                active: true
            },
            services: []
        }
    },
    methods: {
        showModal() {
            this.newService = {
                name: '',
                command: '',
                status: 'online',
                active: true
            };
            this.editingIndex = null;
            this.showAddModal = true;
        },
        closeModal() {
            this.showAddModal = false;
        },
        confirmAdd() {
            if (!this.newService.name || !this.newService.command) return;

            console.log("add ", this.newService)
            if (this.editingIndex !== null) {
                // Update existing service
                this.services[this.editingIndex] = { ...this.newService };
            } else {
                invoke("set_server", {server: this.newService}).then(res=>{
                    console.log("res ", res)
                    this.refreshServices()
                })
            }

            this.closeModal();
        },
        editService(index) {
            this.editingIndex = index;
            this.newService = { ...this.services[index].config };
            this.showAddModal = true;
        },
        toggleServiceStatus(index) {
            const service = this.services[index];
            service.status = service.active ? 'online' : 'offline';
        },
        startService(index) {
            console.log('Starting service:', this.services[index].name);
            this.services[index].status = 'online';
            this.services[index].active = true;
        },
        stopService(index) {
            console.log('Stopping service:', this.services[index].name);
            this.services[index].status = 'offline';
            this.services[index].active = false;
        },
        restartService(index) {
            console.log('Restarting service:', this.services[index].name);
            this.services[index].status = 'pending';
            setTimeout(() => {
                this.services[index].status = 'online';
            }, 1000);
        },
        capitalizeFirstLetter(string) {
            return string.charAt(0).toUpperCase() + string.slice(1);
        },
        refreshServices() {
            invoke("get_servers").then(res=>{
                console.log("get ", res)
                this.services = res
            })
        }
    },
    mounted() {
        this.refreshServices()
    },
}
</script>

<style scoped>
.card {
    transition: all 0.3s ease;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

.card:hover {
    transform: translateY(-2px);
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
}

.status-badge {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    display: inline-block;
    margin-right: 6px;
}

.status-online {
    background-color: #10B981;
}

.status-offline {
    background-color: #EF4444;
}

.status-pending {
    background-color: #F59E0B;
}

.add-card {
    border: 2px dashed #CBD5E0;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.3s ease;
}

.add-card:hover {
    border-color: #4299E1;
    background-color: #EBF8FF;
}
</style>