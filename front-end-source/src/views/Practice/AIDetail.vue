<script setup lang="ts">
  import { nextTick, onMounted, ref } from 'vue'
  import { askAI, getAIHistory } from '@/apis/ai'  // 新增导入历史记录接口
  import { ElMessage, ElButton, ElInput, ElEmpty } from 'element-plus'
  import MarkdownIt from 'markdown-it'
  import hljs from 'highlight.js'
  import 'highlight.js/styles/github.css'
  import eventEmitter from '@/utils/eventEmitter'

  function visualizeWhitespace(str: string) {
    return str
      .replace(/\r/g, '[回车]\r')
      .replace(/\n/g, '[换行]\n')
      .replace(/\t/g, '→→→')         // 显示 tab 为箭头
      .replace(/  /g, '␣␣')          // 可视化连续空格（更清晰）
      .replace(/ /g, '␣');           // 单个空格也显示为 ␣
  }

  // 初始化Markdown解析器
  const md = new MarkdownIt({
    html: true,
    linkify: true,
    typographer: true,
    highlight: (str, lang) => {
      if (lang && hljs.getLanguage(lang)) {
        try {
          return hljs.highlight(str, { language: lang }).value
        } catch {
          return ''
        }
      }
      return ''
    }
  })

  // 问题输入内容
  const question = ref('')
  // 代码输入内容
  const codeContent = ref('')
  // 聊天记录
  const chatHistory = ref<Array<{ content: string; isAI: boolean }>>([])
  // 加载状态
  const isLoading = ref(false)
  // 滚动容器
  const historyScrollRef = ref<HTMLDivElement>()

  const handleAsk = async () => {
    const q = question.value.trim()
    const c = codeContent.value.trim()

    if (!q || !c) {
      ElMessage.warning('问题或代码内容不能为空')
      return
    }

    try {
      isLoading.value = true
      // 添加用户消息
      chatHistory.value.push({
        content: `**你的问题**\n\`\`\`\n${q}\n\`\`\`\n**相关代码**\n\`\`\`rust\n${c}\n\`\`\``,
        isAI: false
      })

      // 流式接收回答
      await askAI(q, c, (chunk) => {
        chunk = chunk === '' ? ' ' : chunk
        if (!chatHistory.value[chatHistory.value.length - 1]?.isAI) {
          chatHistory.value.push({ content: '', isAI: true })
        }
        console.log(visualizeWhitespace(chunk))
        chatHistory.value[chatHistory.value.length - 1].content += chunk
        historyScrollRef.value?.scrollTo({ top: historyScrollRef.value.scrollHeight })
      })

      ElMessage.success('AI 回答完成')
      console.log(visualizeWhitespace(chatHistory.value[chatHistory.value.length - 1].content))
    } catch (error) {
      ElMessage.error(`请求失败: ${error instanceof Error ? error.message : '未知错误'}`)
      if (chatHistory.value.length > 0 && chatHistory.value[chatHistory.value.length - 1].isAI) {
        chatHistory.value.pop()
      }
    } finally {
      isLoading.value = false
      question.value = ''
      codeContent.value = ''
    }
  }

  // 新增分页相关状态
  const currentPage = ref(1)
  const perPage = 10
  const hasMore = ref(true)
  const isLoadingHistory = ref(false)

  // 监听滚动事件
  const handleScroll = (e: Event) => {
    const target = e.target as HTMLDivElement
    if (target.scrollTop < 100 && !isLoadingHistory.value && hasMore.value) {
      loadMoreHistory()
    }
  }

  // 加载历史记录
  const loadMoreHistory = async () => {
    try {
      isLoadingHistory.value = true
      const prevHeight = historyScrollRef.value?.scrollHeight || 0

      const res = await getAIHistory({ page: currentPage.value, per_page: perPage })

      // const newHistories = res.histories.map(h => ({
      //   content: `**历史问题**\n${h.question}\n\n**AI回答**\n${h.response}`,
      //   isAI: true,
      //   createdAt: h.created_at
      // }))
      const newHistories: Array<{ content: string; isAI: boolean }> = []
      res.histories.forEach(h => {
        newHistories.push({
          content: h.question,
          isAI: false
        },
          {
            content: h.response,
            isAI: true
          }
        )
      })

      // 保持滚动位置
      if (newHistories.length) {
        chatHistory.value.unshift(...newHistories)
        await nextTick()
        if (historyScrollRef.value) {
          historyScrollRef.value.scrollTop = historyScrollRef.value.scrollHeight - prevHeight
        }
      }

      hasMore.value = newHistories.length >= perPage
      currentPage.value += hasMore.value ? 1 : currentPage.value
    } catch {
      ElMessage.error('加载历史记录失败')
    } finally {
      isLoadingHistory.value = false
    }
  }

  function handleFill() {
    eventEmitter.emit('reqGetCode')
  }

  eventEmitter.listen('resGetCode', (code: string) => {
    codeContent.value = code
  })

  // 初始化加载第一页
  onMounted(() => {
    loadMoreHistory()
  })
</script>

<template>
  <div class="ai-detail-container">
    <!-- 聊天记录区域 -->
    <el-card class="history-card" shadow="hover">
      <div class="chat-history" ref="historyScrollRef" @scroll="handleScroll">
        <ElEmpty v-if="chatHistory.length === 0" description="开始你的第一次AI问答" :image-size="120" />

        <div v-for="(msg, index) in chatHistory" :key="index" :class="['chat-item', msg.isAI ? 'ai-msg' : 'user-msg']">
          <div class="msg-content" v-html="msg.isAI ? md.render(msg.content) : msg.content" />
        </div>
      </div>
    </el-card>

    <!-- 固定底部的输入区域 -->
    <div class="input-footer">
      <el-card shadow="hover">
        <div class="input-section">
          <el-input v-model="question" placeholder="输入技术问题" class="mb-2" />
          <el-input v-model="codeContent" type="textarea" :rows="3" placeholder="粘贴需要分析的Rust代码" class="mb-2" />
          <el-button @click="handleAsk" type="primary" :loading="isLoading" :disabled="isLoading" class="w-full">
            发送
          </el-button>
          <el-button @click="handleFill" type="primary" :loading="isLoading" :disabled="isLoading" class="w-full">
            填充代码
          </el-button>
        </div>
      </el-card>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .ai-detail-container {
    height: 100vh;
    display: flex;
    flex-direction: column;
    position: relative;

    .history-card {
      flex: 1;
      margin-bottom: 0;
      overflow: hidden;

      .chat-history {
        height: calc(100vh - 220px);
        padding: 16px;
        overflow-y: auto;
      }
    }

    .input-footer {
      position: sticky;
      bottom: 0;
      background: white;
      z-index: 1;
      box-shadow: 0 -2px 12px rgba(0, 0, 0, 0.05);

      .el-card {
        border-radius: 0;
        border: none;
      }
    }

    .chat-item {
      max-width: 85%;
      margin: 12px 0;
      border-radius: 8px;
      padding: 12px;

      &.user-msg {
        margin-left: auto;
        background: var(--el-color-primary-light-9);
        border-radius: 8px 8px 0 8px;
      }

      &.ai-msg {
        margin-right: auto;
        background: #f0f2f5;
        border-radius: 8px 8px 8px 0;
      }

      .msg-content {
        font-size: 14px;
        line-height: 1.6;

        :deep() {
          pre {
            background: rgba(0, 0, 0, 0.05);
            padding: 12px;
            border-radius: 6px;
            overflow-x: auto;
          }

          code {
            font-family: Consolas, Monaco, monospace;
            padding: 0.2em 0.4em;
            background: rgba(0, 0, 0, 0.05);
            border-radius: 3px;
          }

          blockquote {
            border-left: 3px solid #ddd;
            padding-left: 1em;
            color: #666;
            margin: 1em 0;
          }

          table {
            border-collapse: collapse;
            margin: 1em 0;

            td,
            th {
              padding: 0.6em;
              border: 1px solid #ddd;
            }
          }
        }
      }
    }
  }

  .chat-history {

    /* 新增加载提示样式 */
    .loading-text {
      text-align: center;
      padding: 10px;
      color: #666;
    }

    /* 新增时间戳样式 */
    .time-stamp {
      font-size: 12px;
      color: #999;
      margin-top: 8px;
      text-align: right;
    }
  }
</style>
