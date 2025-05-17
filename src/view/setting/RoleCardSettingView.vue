<template>
	<div class="role-card-setting">
		<div class="card-list">
			<div v-for="card in roleCards" :key="card.id" class="role-card">
				<div class="card-content">
					<h3 class="card-name">{{ card.name }}</h3>
					<p class="card-desc">{{ card.description }}</p>
				</div>
				<div class="card-actions">
					<button class="edit-btn" @click="openEditDialog(card)">
						编辑
					</button>
				</div>
			</div>
		</div>

		<!-- 编辑弹窗 -->
		<div v-if="showEditDialog" class="edit-dialog-overlay">
			<div class="edit-dialog">
				<h3>编辑角色卡片</h3>
				<div class="form-group">
					<label>角色名称</label>
					<input v-model="editingCard.name" type="text" />
				</div>
				<div class="form-group">
					<label>角色描述</label>
					<textarea v-model="editingCard.description"></textarea>
				</div>
				<div class="form-group">
					<label>提示词</label>
					<textarea v-model="editingCard.prompt"></textarea>
				</div>
				<div class="dialog-actions">
					<button class="cancel-btn" @click="closeEditDialog">取消</button>
					<button class="save-btn" @click="saveCard">保存</button>
				</div>
			</div>
		</div>
	</div>
</template>

<script lang="ts">
import { invoke } from "@tauri-apps/api/core";

interface RoleCard {
	id: string
	name: string
	description: string
	prompt: string
	enabled: boolean
}

export default {
	name: 'RoleCardSettingView',
	data() {
		return {
			roleCards: [{
				name: "nn", id:"id", description:"hhhhhh",
			prompt:"", enabled:true} as RoleCard] as RoleCard[],
			showEditDialog: false,
			editingCard: undefined,
		}
	},
	setup() {
	},
	methods: {
		async loadRoleCards() {
			this.roleCards = await invoke("get_rolecard")
		},
		openEditDialog(card: RoleCard) {
			this.editingCard = { ...card }
			this.showEditDialog = true
		},
		closeEditDialog() {
			this.showEditDialog = false
		},
		async saveCard() {
			this.closeEditDialog()
			await invoke("set_rolecard", {card: this.editingCard})
			await this.loadRoleCards()
		}
	},
	mounted() {
		this.loadRoleCards()
	}
}
</script>

<style scoped>
.role-card-setting {
	padding: 20px;
	max-width: 800px;
	margin: 0 auto;
}

.setting-title {
	color: #42b983;
	margin-bottom: 20px;
}

.card-list {
	display: grid;
	grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
	gap: 20px;
}

.role-card {
	background: #fff;
	border-radius: 8px;
	box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
	padding: 16px;
	display: flex;
	flex-direction: column;
	transition: all 0.3s ease;
	margin: 10px;
}

.role-card:hover {
	transform: translateY(-2px);
	box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.card-content {
	flex: 1;
}

.card-name {
	margin: 0 0 8px;
	color: #2c3e50;
}

.card-desc {
	margin: 0;
	color: #666;
	font-size: 14px;
}

.card-actions {
	display: flex;
	justify-content: flex-end;
	margin-top: 12px;
}

.edit-btn {
	background: #42b983;
	color: white;
	border: none;
	padding: 6px 12px;
	border-radius: 4px;
	cursor: pointer;
	transition: background 0.2s;
}

.edit-btn:hover {
	background: #3aa876;
}

.edit-dialog-overlay {
	position: fixed;
	top: 0;
	left: 0;
	right: 0;
	bottom: 0;
	background: rgba(0, 0, 0, 0.5);
	display: flex;
	align-items: center;
	justify-content: center;
	z-index: 1000;
}

.edit-dialog {
	background: white;
	padding: 20px;
	border-radius: 8px;
	width: 500px;
	max-width: 90%;
}

.form-group {
	margin-bottom: 16px;
}

.form-group label {
	display: block;
	margin-bottom: 6px;
	font-weight: bold;
}

.form-group input,
.form-group textarea {
	width: 100%;
	padding: 8px;
	border: 1px solid #ddd;
	border-radius: 4px;
}

.form-group textarea {
	min-height: 100px;
}

.dialog-actions {
	display: flex;
	justify-content: flex-end;
	gap: 10px;
}

.cancel-btn,
.save-btn {
	padding: 8px 16px;
	border-radius: 4px;
	cursor: pointer;
}

.cancel-btn {
	background: #f0f0f0;
	border: 1px solid #ddd;
}

.save-btn {
	background: #42b983;
	color: white;
	border: none;
}

.save-btn:hover {
	background: #3aa876;
}
</style>
