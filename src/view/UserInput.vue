<template>
    <div class="home">
        <h1>输入框示例</h1>
        <FancyInput v-model="username" label="用户名" placeholder="请输入用户名" icon="fas fa-user" />
        <FancyInput v-model="password" label="密码" type="password" placeholder="请输入密码" icon="fas fa-lock" />
        <FancyInput v-model="email" label="邮箱" placeholder="请输入邮箱" :error="emailError" />
        <FancyInput v-model="disabledInput" label="禁用输入框" placeholder="此输入框已禁用" :disabled="true" />
    </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed } from 'vue';
import FancyInput from '@/components/FancyInput.vue';

export default defineComponent({
    name: 'HomeView',
    components: {
        FancyInput,
    },
    setup() {
        const username = ref('');
        const password = ref('');
        const email = ref('');
        const disabledInput = ref('');

        const emailError = computed(() => {
            if (email.value && !/^\S+@\S+\.\S+$/.test(email.value)) {
                return '请输入有效的邮箱地址';
            }
            return '';
        });

        return {
            username,
            password,
            email,
            disabledInput,
            emailError,
        };
    },
});
</script>

<style scoped>
.home {
    max-width: 400px;
    margin: 0 auto;
    padding: 2rem;
}
</style>