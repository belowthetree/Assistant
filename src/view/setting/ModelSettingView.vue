<template>
    <div class="settings-container">
        <div class="settings-header">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path
                    d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z">
                </path>
                <circle cx="12" cy="12" r="3"></circle>
            </svg>
            <h1>AI 模型设置</h1>
        </div>

        <div class="settings-body">
            <div class="settings-section">
                <div class="section-header">
                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none"
                        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
                        <circle cx="12" cy="7" r="4"></circle>
                    </svg>
                    <h2>基本设置</h2>
                </div>

                <div class="flex-row">
                    <div class="form-group">
                        <label for="model-name">接口类型</label>
                        <select id="model-name" class="form-control form-select" v-model="modelconfig.modelType">
                            <option v-for="item in apiTypeOptions" :value="item">{{ item }}</option>
                        </select>
                    </div>
                </div>

                <div class="flex-row">
                    <div class="form-group">
                        <label for="model-name">模型名称</label>
                        <select id="model-name" class="form-control form-select" v-model="modelconfig.modelName">
                            <option v-for="item in modelNameOptions" :value="item">{{ item }}</option>
                        </select>
                    </div>
                </div>

                <div class="form-group">
                    <label for="api-key">API 密钥</label>
                    <input type="password" id="api-key" class="form-control" @change="modify" v-model="modelconfig.apiKey" placeholder="输入API密钥">
                    <div class="model-status">
                        <span class="connection-status">
                            <span class="status-indicator" :class="{ connected: isConnected }"></span>
                            <span>{{ isConnected ? '已连接' : '未连接' }}</span>
                        </span>
                        <span class="badge" :class="apiKeyValid ? 'badge-success' : 'badge-warning'">
                            {{ apiKeyValid ? '有效' : '无效' }}
                        </span>
                    </div>
                </div>

                <div class="form-group">
                    <label for="api-url">API URL</label>
                    <input type="url" id="api-url" class="form-control" v-model="modelconfig.baseUrl" placeholder="输入API端点URL">
                </div>
            </div>

            <div class="settings-section">
                <div class="section-header">
                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none"
                        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"></path>
                    </svg>
                    <h2>提示词设置</h2>
                </div>

                <div class="form-group">
                    <label for="system-prompt">系统提示词</label>
                    <textarea id="system-prompt" class="form-control form-textarea" v-model="systemPrompt"
                        placeholder="输入系统提示词" @change="modify"></textarea>
                </div>
            </div>

            <div class="settings-section">
                <div class="advanced-toggle" @click="showAdvanced = !showAdvanced">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none"
                        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <polyline points="6 9 12 15 18 9"></polyline>
                    </svg>
                    <span>高级选项</span>
                </div>

                <div class="advanced-options" v-if="showAdvanced">
                    <div class="flex-row">
                        <div class="form-group">
                            <label for="temperature">温度 (Temperature)</label>
                            <input type="range" id="temperature" min="0" max="2" step="0.1" v-model="modelconfig.temperature"
                                class="form-control" @change="modify">
                            <div style="text-align: center; font-size: 12px; color: var(--text-light);">{{ modelconfig.temperature
                                }}</div>
                        </div>
                        <!-- <div class="form-group">
                            <label for="max-tokens">最大令牌数</label>
                            <input type="number" id="max-tokens" class="form-control" v-model="maxTokens" min="1"
                                max="4096">
                        </div> -->
                    </div>

                    <div class="form-group">
                        <label>启用流式响应</label>
                        <div style="display: flex; align-items: center; gap: 8px;">
                            <label class="toggle-switch" style="flex-direction: row;display: flex; ">
                                <input type="checkbox" v-model="modelconfig.stream" @change="modify">
                                <span class="slider"></span>
                            </label>
                            <span>启用</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div class="settings-footer">
            <button class="btn btn-secondary" @click="cancel">
                <svg class="btn-icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
                    fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="18" y1="6" x2="6" y2="18"></line>
                    <line x1="6" y1="6" x2="18" y2="18"></line>
                </svg>
                取消
            </button>
            <button class="btn btn-primary" @click="saveSettings">
                <svg class="btn-icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
                    fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
                    <polyline points="17 21 17 13 7 13 7 21"></polyline>
                    <polyline points="7 3 7 8 15 8"></polyline>
                </svg>
                保存设置
            </button>
        </div>
    </div>
</template>

<script>
import { OpenAIModel } from "@/model/openai"
import { getRoleCard, loadConfig, ModelConfig, ModelList, saveConfig } from "~/src/config";
import { EModelType } from "~/src/data";
import { MCPClient } from "~/src/frontend/MCPClient";
import { generateModelFromConfig } from "~/src/model/global";
import { Ollama } from "~/src/model/ollama";

export default {
    name: 'AIModelSettings',
    data() {
        return {
            modelconfig: new ModelConfig("默认配置"),
            apiTypeOptions: Object.values(EModelType),
            modelNameOptions: [],
            isConnected: true,
            apiKeyValid: true,
            systemPrompt: '你是一个有帮助的AI助手。回答要简洁专业。',
            showAdvanced: false,
            maxTokens: 2048,
            timeout: 30
        }
    },
    methods: {
        saveSettings() {
            ModelList.saveModel("默认配置", this.modelconfig)
            saveConfig()
        },
        cancel() {
        },
        async modify() {
            this.saveSettings()
            this.apiKeyValid = false
            this.isConnected = false
            // 检查 api 连接
            if (this.modelconfig.modelName.length <= 0)
                return
            if (this.modelconfig.modelType.length <= 0)
                return
            if (this.modelconfig.modelType !== EModelType.Ollama && this.modelconfig.baseUrl <= 0)
                return
            let model = generateModelFromConfig(this.modelconfig)
            const val = await model.checkApiKeyValid()
            this.apiKeyValid = val
            if (this.apiKeyValid) {
                model.getModels().then((models)=>{
                    this.isConnected = val
                    this.modelNameOptions = models
                    if (this.modelName in this.modelNameOptions === false) {
                        this.modelName = this.modelNameOptions[0] || ""
                    }
                    this.modelconfig.modelName = this.modelName
                })
            }
        }
    },
    mounted() {
        // const client = new MCPClient({})
        // client.addServers([{name: "fetch", command: "node", args: ["/home/zgg/文档/Cline/MCP/fetch-mcp/dist/index.js"]}])
        loadConfig().finally(()=>{
            this.modelconfig = ModelList.getCurrentModelConfig()
            this.modify()
            let card = getRoleCard(this.modelconfig.roleCard)
            this.systemPrompt = card.systemPrompt
            let model = generateModelFromConfig(this.modelconfig)
            model.getModels().then((vals)=>{
                this.modelNameOptions = vals
            })
        })
    }
}
</script>

<style>
:root {
    --primary: #6366f1;
    --primary-hover: #4f46e5;
    --secondary: #f3f4f6;
    --secondary-hover: #e5e7eb;
    --text: #111827;
    --text-light: #6b7280;
    --bg: #f9fafb;
    --card-bg: #ffffff;
    --border: #e5e7eb;
    --success: #10b981;
    --warning: #f59e0b;
    --danger: #ef4444;
}

* {
    overflow-y: scroll;
    margin: 0;
    padding: 0;
    box-sizing: border-box;
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

body {
    background-color: var(--bg);
    color: var(--text);
    min-height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 20px;
}

.settings-container {
    width: 100%;
    background-color: var(--card-bg);
    border-radius: 12px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
    overflow: hidden;
}

.settings-header {
    padding: 20px 24px;
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    gap: 12px;
}

.settings-header h1 {
    font-size: 20px;
    font-weight: 600;
}

.settings-body {
    padding: 24px;
}

.settings-section {
    margin-bottom: 32px;
}

.settings-section:last-child {
    margin-bottom: 0;
}

.section-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 16px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border);
}

.section-header h2 {
    font-size: 16px;
    font-weight: 600;
    color: var(--text);
}

.form-group {
    margin-bottom: 20px;
}

.form-group label {
    display: block;
    margin-bottom: 8px;
    font-size: 14px;
    font-weight: 500;
    color: var(--text-light);
}

.form-control {
    width: 100%;
    padding: 10px 12px;
    border: 1px solid var(--border);
    border-radius: 6px;
    font-size: 14px;
    transition: border-color 0.2s;
}

.form-control:focus {
    outline: none;
    border-color: var(--primary);
    box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.2);
}

.form-textarea {
    min-height: 120px;
    resize: vertical;
}

.form-select {
    appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' fill='%236b7280' viewBox='0 0 16 16'%3E%3Cpath d='M7.247 11.14 2.451 5.658C1.885 5.013 2.345 4 3.204 4h9.592a1 1 0 0 1 .753 1.659l-4.796 5.48a1 1 0 0 1-1.506 0z'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 12px center;
    background-size: 16px;
}

.btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 10px 16px;
    border-radius: 6px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    border: none;
}

.btn-primary {
    background-color: var(--primary);
    color: white;
}

.btn-primary:hover {
    background-color: var(--primary-hover);
}

.btn-secondary {
    background-color: var(--secondary);
    color: var(--text);
}

.btn-secondary:hover {
    background-color: var(--secondary-hover);
}

.btn-icon {
    margin-right: 8px;
}

.settings-footer {
    padding: 16px 24px;
    border-top: 1px solid var(--border);
    display: flex;
    justify-content: flex-end;
    gap: 12px;
}

.toggle-switch {
    position: relative;
    display: inline-block;
    width: 48px;
    height: 24px;
}

.toggle-switch input {
    opacity: 0;
    width: 0;
    height: 0;
}

.slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: #ccc;
    transition: .4s;
    border-radius: 24px;
}

.slider:before {
    position: absolute;
    content: "";
    height: 16px;
    width: 16px;
    left: 4px;
    bottom: 4px;
    background-color: white;
    transition: .4s;
    border-radius: 50%;
}

input:checked+.slider {
    background-color: var(--primary);
}

input:checked+.slider:before {
    transform: translateX(24px);
}

.badge {
    display: inline-flex;
    align-items: center;
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 12px;
    font-weight: 500;
}

.badge-success {
    background-color: rgba(16, 185, 129, 0.1);
    color: var(--success);
}

.badge-warning {
    background-color: rgba(245, 158, 11, 0.1);
    color: var(--warning);
}

.flex-row {
    display: flex;
    gap: 16px;
}

.flex-row .form-group {
    flex: 1;
}

.model-status {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 8px;
}

.connection-status {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
}

.status-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: var(--danger);
}

.status-indicator.connected {
    background-color: var(--success);
}

.advanced-options {
    margin-top: 24px;
    padding-top: 16px;
    border-top: 1px dashed var(--border);
}

.advanced-toggle {
    display: flex;
    align-items: center;
    gap: 8px;
    color: var(--primary);
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    margin-bottom: 16px;
}
</style>