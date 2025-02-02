<script lang="ts">
import FileUpload from '~/src/components/FileUpload.vue';

export default {
    components: {
        FileUpload
    },
    methods: {
        handleSuccess(response, file) {
            console.log('上传成功:', file.name, response);
        },
        handleError(error, file) {
            console.error('上传失败:', file.name, error);
        }
    }
}
</script>

<template>
    <FileUpload api-url="/api/upload" :max-size="20" accept="image/*, .pdf" @upload-success="handleSuccess"
        @upload-error="handleError" />
</template>