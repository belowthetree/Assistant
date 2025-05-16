<template>
    <div class="dashboard-container">
        <div class="content-container">
            <div class="header">
                <h1>MCP 服务</h1>
                <button @click="showModal" class="add-button">
                    <i class="fas fa-plus"></i>添加 MCP 服务
                </button>
            </div>

            <div class="grid-container">
                <!-- Service Cards -->
                <div v-for="(service, index) in services" :key="index" class="service-card">
                    <div class="card-content">
                        <div class="card-header">
                            <div>
                                <h3>{{ service.config.name }}</h3>
                                <div class="status-container">
                                    <span class="status-badge" :class="{'status-online': service.connected, 'status-offline': !service.connected}"></span>
                                    <span class="status-text">
                                        {{ capitalizeFirstLetter(service.connected ? "已连接" : "未连接" ) }}
                                    </span>
                                </div>
                            </div>
                            <div class="action-buttons">
                                <button @click="editService(index)" class="edit-button">
                                    <i class="fas fa-edit"></i>
                                </button>
                                <label class="toggle-switch">
                                    <input type="checkbox" v-model="service.config.enable" @change="toggleServiceStatus(index)"
                                        class="toggle-input">
                                    <div class="toggle-slider"></div>
                                </label>
                            </div>
                        </div>

                        <div class="command-section">
                            <p class="command-label">Start Command</p>
                            <div class="command-box">
                                <code class="command-text">{{ service.config.command }}</code>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Empty Card Slot -->
                <div @click="showModal" class="add-card">
                    <div class="add-card-content">
                        <i class="fas fa-plus-circle"></i>
                        <p>添加 MCP 服务</p>
                    </div>
                </div>
            </div>
        </div>

        <!-- Add Service Modal -->
        <div v-if="showAddModal" class="modal-overlay">
            <div class="modal-container">
                <div class="modal-header">
                    <h3>
                        {{ editingIndex === null ? '添加 MCP 服务' : '编辑 MCP 服务' }}
                    </h3>
                </div>
                <div class="modal-body">
                    <div class="form-group">
                        <label>服务名字</label>
                        <input type="text" v-model="newService.name">
                    </div>
                    <div class="form-group">
                        <label>描述</label>
                        <input type="text" v-model="newService.desc">
                    </div>
                    <div class="form-group">
                        <label>命令</label>
                        <input type="text" v-model="newService.command">
                    </div>
                    <div class="form-group">
                        <label>参数</label>
                        <input type="text" v-model="newService.args">
                    </div>
                    <div class="form-group">
                        <label>环境变量</label>
                        <input type="text" v-model="newService.env" placeholder="">
                    </div>
                </div>
                <div class="modal-footer">
                    <button @click="closeModal" class="cancel-button">
                        取消
                    </button>
                    <button @click="confirmAdd" class="confirm-button">
                        {{ editingIndex === null ? '添加服务' : '更新服务' }}
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
.dashboard-container {
    background-color: #f3f4f6;
    min-height: 100vh;
    padding: 1.5rem;
}

.content-container {
    max-width: 56rem;
    margin-left: auto;
    margin-right: auto;
}

.header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
}

.header h1 {
    font-size: 1.5rem;
    font-weight: 700;
    color: #1f2937;
}

.add-button {
    background-color: #3b82f6;
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    transition: background-color 0.3s ease;
    display: flex;
    align-items: center;
}

.add-button:hover {
    background-color: #2563eb;
}

.add-button i {
    margin-right: 0.5rem;
}

.grid-container {
    display: grid;
    grid-template-columns: 1fr;
    gap: 1rem;
}

@media (min-width: 640px) {
    .grid-container {
        grid-template-columns: repeat(2, 1fr);
    }
}

.service-card {
    background-color: white;
    border-radius: 0.5rem;
    overflow: hidden;
    transition: all 0.3s ease;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

.service-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
}

.card-content {
    padding: 1rem;
}

.card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
}

.card-header h3 {
    font-size: 1.125rem;
    font-weight: 600;
    color: #1f2937;
}

.status-container {
    display: flex;
    align-items: center;
    margin-top: 0.25rem;
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

.status-text {
    font-size: 0.875rem;
    color: #4b5563;
}

.action-buttons {
    display: flex;
    gap: 0.5rem;
}

.edit-button {
    color: #9ca3af;
    transition: color 0.3s ease;
}

.edit-button:hover {
    color: #3b82f6;
}

.toggle-switch {
    position: relative;
    display: inline-flex;
    align-items: center;
    cursor: pointer;
}

.toggle-input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
}

.toggle-slider {
    width: 2.75rem;
    height: 1.5rem;
    background-color: #e5e7eb;
    border-radius: 9999px;
    position: relative;
    transition: background-color 0.3s ease;
}

.toggle-slider:after {
    content: '';
    position: absolute;
    top: 2px;
    left: 2px;
    width: 1.25rem;
    height: 1.25rem;
    background-color: white;
    border: 1px solid #d1d5db;
    border-radius: 50%;
    transition: transform 0.3s ease;
}

.toggle-input:checked + .toggle-slider {
    background-color: #3b82f6;
}

.toggle-input:checked + .toggle-slider:after {
    transform: translateX(1.25rem);
    border-color: white;
}

.command-section {
    margin-top: 0.5rem;
}

.command-label {
    font-size: 0.875rem;
    color: #6b7280;
    margin-bottom: 0.25rem;
    text-align: left;
}

.command-box {
    background-color: #f9fafb;
    padding: 0.5rem;
    border-radius: 0.375rem;
    overflow-x: hidden;
}

.command-text {
    font-size: 0.875rem;
    color: #374151;
    text-align: left;
    white-space: nowrap;
    overflow-x: hidden;
}

.add-card {
    border: 2px dashed #cbd5e0;
    border-radius: 0.5rem;
    padding: 1.25rem;
    min-height: 200px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s ease;
}

.add-card:hover {
    border-color: #3b82f6;
    background-color: #ebf8ff;
}

.add-card-content {
    text-align: center;
}

.add-card-content i {
    font-size: 2.25rem;
    color: #9ca3af;
    margin-bottom: 0.5rem;
}

.add-card-content p {
    color: #6b7280;
}

.modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
}

.modal-container {
    background-color: white;
    border-radius: 0.5rem;
    padding: 1.5rem;
    width: 100%;
    max-width: 28rem;
}

.modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
}

.modal-header h3 {
    font-size: 1.125rem;
    font-weight: 600;
}

.modal-body {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.form-group {
    display: flex;
    flex-direction: column;
}

.form-group label {
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
    margin-bottom: 0.25rem;
    text-align: left;
}

.form-group input {
    width: 100%;
    padding: 0.5rem 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
}

.modal-footer {
    margin-top: 1.5rem;
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
}

.cancel-button {
    padding: 0.5rem 1rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    color: #374151;
    transition: background-color 0.3s ease;
}

.cancel-button:hover {
    background-color: #f9fafb;
}

.confirm-button {
    padding: 0.5rem 1rem;
    background-color: #3b82f6;
    color: white;
    border-radius: 0.375rem;
    transition: background-color 0.3s ease;
}

.confirm-button:hover {
    background-color: #2563eb;
}
</style>