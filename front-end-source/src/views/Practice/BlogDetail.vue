<script setup lang="ts">
  import { ref, onMounted } from 'vue'
  import { useRoute } from 'vue-router'
  import { getBlogs, getComments, addComment, type BlogPost, type Comment, addBlog } from '@/apis/blog'
  import { ElMessage, ElLoading, ElPagination, ElMessageBox } from 'element-plus'
  import MarkdownIt from 'markdown-it'
  import hljs from 'highlight.js'
  import 'highlight.js/styles/github.css'

  const route = useRoute()
  let id = route.query.id as string

  // 博客列表数据
  const blogList = ref<BlogPost[]>([])
  // 评论列表映射 { blogId: Comment[] }
  const commentsMap = ref<Record<number, Comment[]>>({})
  // 新增评论内容映射 { blogId: string }
  const newCommentContent = ref<Record<number, string>>({})
  // 加载状态
  const loading = ref(false)
  // 分页相关
  const currentPage = ref(1)
  const pageSize = ref(10)
  const total = ref(0)

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
          return '' // 使用额外的默认转义
        }
      }
      return '' // 使用额外的默认转义
    }
  })

  // 获取博客列表
  const fetchBlogs = async () => {
    try {
      loading.value = true
      const { code, posts } = await getBlogs({
        problem: id,
        page: currentPage.value,
        per_page: pageSize.value
      })
      if (code === 0) {
        blogList.value = posts
        total.value = posts.length // 假设返回的是全部数量

        // 预加载评论
        posts.forEach(blog => {
          if (blog.id) fetchComments(blog.id)
        })
      } else {
        ElMessage.error('获取博客列表失败')
      }
    } catch (error) {
      console.error('获取博客列表失败', error)
      ElMessage.error('获取博客列表失败')
    } finally {
      loading.value = false
    }
  }

  // 获取评论列表
  const fetchComments = async (blogId: number) => {
    try {
      const { comments } = await getComments(blogId)
      commentsMap.value[blogId] = comments
    } catch (error) {
      console.error(`获取博客${blogId}评论失败`, error)
      commentsMap.value[blogId] = []
    }
  }

  // 添加评论
  const handleAddComment = async (blogId: number) => {
    const content = newCommentContent.value[blogId]?.trim()
    if (!content) {
      ElMessage.warning('评论内容不能为空')
      return
    }

    let loadingInstance: any
    try {
      loadingInstance = ElLoading.service({
        lock: true,
        text: '提交评论中...',
        background: 'rgba(0, 0, 0, 0.1)'
      })

      await addComment(blogId, { content })
      ElMessage.success('评论提交成功')

      // 刷新评论列表
      await fetchComments(blogId)

      // 清空输入框
      newCommentContent.value[blogId] = ''
    } catch (error) {
      console.error(`添加评论失败:`, error)
      ElMessage.error('评论提交失败，请稍后再试')
    } finally {
      loadingInstance.close()
    }
  }

  // 格式化日期
  const formatDate = (date: string | Date) => {
    if (!date) return ''
    return new Date(date).toLocaleString()
  }

  // 添加博客
  // 新增博客表单数据
  const newBlogForm = ref({
    title: '',
    content: '',
  })

  // 表单验证状态
  const formErrors = ref({
    title: '',
    content: '',
  })

  // 提交状态
  const submitting = ref(false)
  const handleAddBlog = async () => {
    // 简单验证
    let isValid = true
    formErrors.value = { title: '', content: '' }

    if (!newBlogForm.value.title.trim()) {
      formErrors.value.title = '标题不能为空'
      isValid = false
    }

    if (!newBlogForm.value.content.trim()) {
      formErrors.value.content = '内容不能为空'
      isValid = false
    }


    if (!isValid) return

    try {
      submitting.value = true

      // 调用API提交博客
      await addBlog({
        title: newBlogForm.value.title,
        content: newBlogForm.value.content,
        related_problem: id
      })

      ElMessageBox.alert('博客提交成功！', '成功', {
        type: 'success'
      })

      // 清空表单
      newBlogForm.value = {
        title: '',
        content: '',
      }

      // 刷新博客列表
      currentPage.value = 1
      await fetchBlogs()
    } catch (error) {
      console.error('提交博客失败', error)
      ElMessageBox.alert('博客提交失败，请稍后再试', '错误', {
        type: 'error'
      })
    } finally {
      submitting.value = false
    }
  }
  onMounted(() => {
    fetchBlogs()
    id = route.query.id as string
  })
</script>

<template>
  <div class="blog-container">

    <!-- 博客列表 -->
    <div v-if="loading" class="loading-container">
      <el-skeleton animated count="3" />
    </div>

    <div v-else class="blog-list">
      <el-empty v-if="blogList.length === 0" description="暂无博客" />

      <el-card v-for="blog in blogList" :key="blog.id" class="blog-card mb-6" shadow="hover">
        <template #header>
          <div class="flex justify-between items-center">
            <h3 class="text-xl font-semibold">{{ blog.title }}</h3>
            <div class="text-sm text-gray-500">
              <span>{{ formatDate(blog.created_at!) }}</span>
            </div>
          </div>
        </template>

        <!-- 博客内容 (Markdown渲染) -->
        <div class="blog-content p-4" v-html="md.render(blog.content || '')" />

        <!-- 评论区域 -->
        <div class="comments-section mt-4 pt-4 border-t border-gray-100">
          <h3 class="text-lg font-medium mb-3">评论 ({{ commentsMap[blog.id]?.length || 0 }})</h3>

          <!-- 评论列表 -->
          <div class="comments-list">
            <el-empty v-if="!(commentsMap[blog.id]?.length)" description="暂无评论" />

            <div v-else class="space-y-4">
              <div style="margin-top: 10px;" v-for="comment in commentsMap[blog.id]" :key="comment.id"
                class="comment-item p-3 bg-gray-50 rounded-lg">
                <div class="flex justify-between text-sm text-gray-500 mb-1">
                  <span>评论 #{{ comment.id }}</span>
                  <span>{{ formatDate(comment.created_at!) }}</span>
                </div>
                <div class="comment-content">{{ comment.content }}</div>
              </div>
            </div>
          </div>

          <!-- 发表评论 -->
          <div class="comment-form mt-4">
            <el-input v-model="newCommentContent[blog.id]" type="textarea" :rows="3" placeholder="发表你的评论..."
              class="mb-2" />
            <el-button @click="handleAddComment(blog.id)" type="primary" size="small">
              提交评论
            </el-button>
          </div>
        </div>
      </el-card>
    </div>

    <!-- 分页 -->
    <div class="pagination-container mt-6">
      <el-pagination v-model:current-page="currentPage" v-model:page-size="pageSize" :total="total"
        layout="total, prev, pager, next, sizes" @size-change="fetchBlogs" @current-change="fetchBlogs" />
    </div>

    <!-- 提交博客区域 -->
    <el-card style="margin-top: 32px;" class="mt-8 submit-blog-card" shadow="none">

      <el-form label-width="100px" class="space-y-4">
        <el-form-item label="标题" :error="formErrors.title">
          <el-input v-model="newBlogForm.title" placeholder="请输入博客标题" class="w-full" />
        </el-form-item>


        <el-form-item label="内容" :error="formErrors.content">
          <el-input v-model="newBlogForm.content" type="textarea" :rows="10" placeholder="请输入博客内容（支持Markdown格式）"
            class="w-full" />
        </el-form-item>

        <el-form-item>
          <el-button @click="handleAddBlog" type="primary" :loading="submitting">
            提交博客
          </el-button>
        </el-form-item>
      </el-form>
    </el-card>
  </div>
</template>

<style lang="scss" scoped>
  .blog-container {
    max-width: 100%;
    margin: 0 auto;
    padding: 20px;
    height: 100%;
    overflow-y: auto;

    .blog-card {
      max-width: 100%;
      transition: transform 0.2s ease;

      &:hover {
        transform: translateY(-2px);
      }

      .comments-section {
        border-top: 1px solid #ddd;

        .comment-item {
          transition: background-color 0.2s;

          &:hover {
            background-color: #f0f2f5;
          }
        }

        .comment-form {
          .el-input__inner {
            border-radius: 6px;
          }
        }
      }
    }

    .pagination-container {
      display: flex;
      justify-content: center;
    }
  }

  :deep(.el-card) {
    border: none;
  }
</style>
<style lang="scss">
  .blog-content {

    // 为Markdown内容添加样式
    h1,
    h2,
    h3,
    h4,
    h5,
    h6 {
      margin-top: 1em;
      margin-bottom: 0.5em;
    }

    p {
      font-size: 1.2em !important;
      margin-bottom: 1em;
    }

    ul,
    ol {
      margin-left: 1.5em;
      margin-bottom: 1em;
    }

    pre {
      background-color: #f8f9fa;
      padding: 1em;
      border-radius: 4px;
      overflow-x: auto;
      margin-bottom: 1em;
    }

    code {
      font-family: Consolas, Monaco, 'Andale Mono', monospace;
      font-size: 0.9em;
      background-color: #f1f1f1;
      padding: 0.2em 0.4em;
      border-radius: 3px;
    }

    pre code {
      background-color: transparent;
      padding: 0;
    }

    blockquote {
      border-left: 4px solid #ddd;
      padding-left: 1em;
      margin-left: 0;
      margin-bottom: 1em;
      color: #666;
    }

    img {
      max-width: 100%;
      height: auto;
      margin: 1em 0;
      border-radius: 4px;
    }
  }

  .submit-blog-card {
    .el-form-item {
      margin-bottom: 16px;
    }
  }
</style>
