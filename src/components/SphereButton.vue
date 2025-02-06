<template>
    <div class="spherical-chat" data->
        <div class="sphere" @click="toggleChat">
            <span v-if="!isOpen">💬</span>
        </div>
        <div v-if="isOpen" class="chat-box">
            <input v-model="message" placeholder="Type your message..." @keyup.enter="sendMessage" />
            <button @click="sendMessage">Send</button>
        </div>
    </div>
</template>

<script>
export default {
    data() {
        return {
            isOpen: false,
            message: ''
        };
    },
    methods: {
        toggleChat() {
            this.isOpen = !this.isOpen;
        },
        sendMessage() {
            if (this.message.trim() !== '') {
                alert(`Message sent: ${this.message}`);
                this.message = '';
            }
        }
    }
};
</script>

<style scoped>
.spherical-chat {
    position: fixed;
    bottom: 20px;
    right: 20px;
}

.sphere {
    width: 60px;
    height: 60px;
    background-color: #007bff;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    color: white;
    font-size: 24px;
}

.chat-box {
    position: absolute;
    bottom: 70px;
    right: 0;
    width: 250px;
    padding: 10px;
    background-color: white;
    border-radius: 8px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    display: flex;
    flex-direction: column;
}

.chat-box input {
    padding: 8px;
    border: 1px solid #ccc;
    border-radius: 4px;
    margin-bottom: 10px;
}

.chat-box button {
    padding: 8px;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.chat-box button:hover {
    background-color: #0056b3;
}
</style>