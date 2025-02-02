<template>
    <div class="upload-container">
        <div class="drop-zone" @dragover.prevent="dragover" @dragleave.prevent="dragleave" @drop.prevent="drop"
            :class="{ 'dragover': isDragging }" @click="selectFile">
            <input type="file" ref="fileInput" multiple @change="handleFileSelect" style="display: none"
                :accept="accept" />
            <div class="upload-prompt">
                <p>点击选择文件 或 拖放文件到这里</p>
                <p v-if="maxSize">最大文件大小: {{ maxSize }}MB</p>
            </div>
        </div>

        <div class="file-list">
            <div v-for="(file, index) in files" :key="index" class="file-item">
                <div class="file-info">
                    <span class="file-name">{{ file.name }}</span>
                    <span class="file-size">{{ formatSize(file.size) }}</span>
                </div>
                <div class="upload-progress">
                    <div class="progress-bar" :style="{ width: file.progress + '%' }"></div>
                    <span class="progress-text">
                        {{ file.status === 'uploading' ? file.progress + '%' : file.status }}
                    </span>
                </div>
                <button class="cancel-btn" @click="removeFile(index)" v-if="file.status === 'uploading'">
                    ×
                </button>
            </div>
        </div>

        <button class="upload-btn" @click="uploadFiles" :disabled="!files.length || isUploading">
            {{ isUploading ? '上传中...' : '开始上传' }}
        </button>
    </div>
</template>

<script>
import axios from 'axios';

export default {
    props: {
        apiUrl: {
            type: String,
            required: true
        },
        maxSize: {
            type: Number,
            default: 10
        },
        accept: {
            type: String,
            default: '*/*'
        }
    },

    data() {
        return {
            isDragging: false,
            files: [],
            isUploading: false
        };
    },

    methods: {
        selectFile() {
            this.$refs.fileInput.click();
        },

        handleFileSelect(e) {
            this.addFiles(e.target.files);
            e.target.value = '';
        },

        dragover() {
            this.isDragging = true;
        },

        dragleave() {
            this.isDragging = false;
        },

        drop(e) {
            this.isDragging = false;
            this.addFiles(e.dataTransfer.files);
        },

        addFiles(fileList) {
            for (let file of fileList) {
                if (file.size > this.maxSize * 1024 * 1024) {
                    alert(`文件 ${file.name} 超过大小限制`);
                    continue;
                }

                this.files.push({
                    file,
                    name: file.name,
                    size: file.size,
                    progress: 0,
                    status: 'pending'
                });
            }
        },

        formatSize(bytes) {
            if (bytes === 0) return '0 B';
            const k = 1024;
            const sizes = ['B', 'KB', 'MB', 'GB'];
            const i = Math.floor(Math.log(bytes) / Math.log(k));
            return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
        },

        async uploadFiles() {
            this.isUploading = true;
            const uploadPromises = this.files.map(async (item, index) => {
                if (item.status !== 'pending') return;

                const formData = new FormData();
                formData.append('file', item.file);

                try {
                    item.status = 'uploading';
                    const response = await axios.post(this.apiUrl, formData, {
                        onUploadProgress: (progressEvent) => {
                            item.progress = Math.round(
                                (progressEvent.loaded * 100) / progressEvent.total
                            );
                        }
                    });

                    item.status = 'success';
                    this.$emit('upload-success', response.data, item.file);
                } catch (error) {
                    item.status = 'error';
                    this.$emit('upload-error', error, item.file);
                }
            });

            await Promise.all(uploadPromises);
            this.isUploading = false;
        },

        removeFile(index) {
            this.files.splice(index, 1);
        }
    }
};
</script>

<style scoped>
.upload-container {
    max-width: 600px;
    margin: 20px auto;
    font-family: Arial, sans-serif;
}

.drop-zone {
    border: 2px dashed #ccc;
    border-radius: 8px;
    padding: 30px;
    text-align: center;
    cursor: pointer;
    transition: all 0.3s;
    background-color: #f8f9fa;
}

.drop-zone.dragover {
    border-color: #2196F3;
    background-color: #e3f2fd;
}

.upload-prompt p {
    margin: 5px 0;
    color: #666;
}

.file-list {
    margin-top: 20px;
}

.file-item {
    display: flex;
    align-items: center;
    padding: 10px;
    border: 1px solid #eee;
    border-radius: 4px;
    margin-bottom: 8px;
    background-color: #fff;
}

.file-info {
    flex: 1;
    min-width: 0;
}

.file-name {
    display: block;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.file-size {
    color: #666;
    font-size: 0.9em;
}

.upload-progress {
    width: 200px;
    margin-left: 20px;
    position: relative;
}

.progress-bar {
    height: 20px;
    background-color: #4CAF50;
    transition: width 0.3s;
}

.progress-text {
    position: absolute;
    right: 5px;
    top: 50%;
    transform: translateY(-50%);
    color: #333;
    font-size: 0.9em;
}

.cancel-btn {
    margin-left: 10px;
    background: none;
    border: none;
    cursor: pointer;
    color: #ff4444;
    font-size: 1.2em;
    padding: 0 5px;
}

.upload-btn {
    margin-top: 20px;
    padding: 10px 20px;
    background-color: #2196F3;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.3s;
}

.upload-btn:disabled {
    background-color: #90caf9;
    cursor: not-allowed;
}

.upload-btn:hover:not(:disabled) {
    background-color: #1976D2;
}
</style>