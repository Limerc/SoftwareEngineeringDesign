import { useUserStore } from "@/stores/userStore";
import { fetchEventSource } from '@microsoft/fetch-event-source';
import { AUTH } from ".";
import { req } from "./req";

export const askAI = async (
  question: string,
  code: string,
  onChunk: (chunk: string) => void
): Promise<void> => new Promise((resolve, reject) => {
  const token = useUserStore().getToken();
  if (!token) {
    throw new Error("未登录，无法获取token");
  }

  const controller = new AbortController();

  fetchEventSource('/api2/api/ai/ask', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      [AUTH]: token,
      'Accept': 'text/event-stream',
      'Cache-Control': 'no-cache',
      'Pragma': 'no-cache',
      'Connection': 'keep-alive',
    },
    body: JSON.stringify({ question, code }), // POST 数据
    onmessage: (event) => {
      onChunk(event.data)
    },
    onerror: (err) => {
      console.error('SSE Error:', err);
      reject(err);
    },
    signal: controller.signal, // 可选：支持取消请求
  })
    .then(() => {
      resolve();
    })
})

export interface AskHistoryParams {
  page: number;
  per_page: number;
}

export interface History {
  question: string;
  response: string;
  created_at: string;
}

export interface ResponseHistory {
  histories: History[];
}

// ... existing code ...

export const getAIHistory = async (params: AskHistoryParams): Promise<ResponseHistory> => {
  const token = useUserStore().getToken();
  if (!token) {
    throw new Error("未登录，无法获取token");
  }

  const url = new URL('/api2/api/ai/history', window.location.origin);
  url.search = new URLSearchParams({
    page: params.page.toString(),
    per_page: params.per_page.toString()
  }).toString();

  try {
    const response = await fetch(url.toString(), {
      method: 'GET',
      headers: {
        [AUTH]: token,
        'Content-Type': 'application/json'
      }
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return await response.json() as ResponseHistory;
  } catch (error) {
    console.error('Fetch history failed:', error);
    throw error;
  }
}
