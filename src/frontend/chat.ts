import { LLMBase } from '../model/llm_base';

interface ChatMessage {
    role: 'user' | 'assistant';
    content: string;
}

export class Chat {
    private messages: ChatMessage[] = [];
    private llmBase: LLMBase;

    constructor(llmBase: LLMBase) {
        this.llmBase = llmBase;
    }

    async talk(content: string): Promise<string> {
        this.messages.push({
            role: 'user',
            content
        });

        // 转为字符串处理
        const messagesString = this.messages
            .map(msg => `${msg.role}: ${msg.content}`)
            .join('\n');

        const response = await this.llmBase.chat(messagesString);

        this.messages.push({
            role: 'assistant',
            content: response
        });

        return response;
    }

    clear(): void {
        this.messages = [];
    }

    getHistory(): ChatMessage[] {
        return [...this.messages];
    }
}
