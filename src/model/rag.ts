import ollama from 'ollama'

export async function test() {
    const response = await ollama.chat({
        model: 'qwen2.5-coder:7b',
        messages: [{ role: 'user', content: '你好' }],
    })
    console.log(response.message.content)
}