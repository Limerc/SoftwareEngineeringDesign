<script setup lang="ts">
  import CodeEditor from '@/components/CodeEditor.vue'
  import { ref } from 'vue';
  import { ElMenu, ElMenuItem, ElMessage } from 'element-plus'
  import { submitProblem } from '@/apis/problems';
  import ResultModal from '@/components/ResultModal.vue'
  import { useRoute } from 'vue-router';
  import eventEmitter from '@/utils/eventEmitter';
  import router from '@/router';

  const route = useRoute()

  const id = +route.query.id!


  const editor = ref<InstanceType<typeof CodeEditor>>()

  const resultVisible = ref(false)
  const resultData = ref<{
    success: boolean;
    message: string;
    stdout?: string;
    stderr?: string;
  }>({
    success: false,
    message: '',
  });

  const handleSubmit = async () => {
    if (!editor.value) return;

    try {
      const code = editor.value.getCode() ?? '';
      const resp = await submitProblem(id, { code });

      resultData.value = {
        success: resp.success,
        message: resp.message,
        stdout: resp.stdout,
        stderr: resp.stderr
      };
      resultVisible.value = true;
    } catch (error: any) {
      ElMessage.error(error.message || '提交失败，请重试');
    }
  }

  function changeMenu(sth: any) {
    router.push({ name: sth.index, query: { id } })
  }

  eventEmitter.listen('submitCode', handleSubmit)
  eventEmitter.listen('reqGetCode', () => {
    eventEmitter.emit('resGetCode', editor.value?.getCode() ?? '')
  })
</script>

<template>
  <div class="container">
    <ElMenu style="width: 4%;" :mode="'vertical'" default-active="pratice-problem-detail">
      <ElMenuItem @click="changeMenu" index="pratice-problem-detail">
        题目
      </ElMenuItem>
      <ElMenuItem @click="changeMenu" index="pratice-blog-detail">
        博客
      </ElMenuItem>
      <ElMenuItem @click="changeMenu" index="pratice-ai-detail">
        AI
      </ElMenuItem>
    </ElMenu>
    <div class="sider">
      <RouterView />
    </div>
    <CodeEditor style="width: 65%;" ref="editor" />
  </div>
  <ResultModal v-model:visible="resultVisible" :success="resultData.success" :message="resultData.message"
    :stdout="resultData.stdout" :stderr="resultData.stderr" />
</template>

<style lang="scss" scoped>
  .container {
    width: 100%;
    height: 100%;
    display: flex;
    overflow: hidden;

    .sider {
      display: flex;
      flex-direction: column;
      width: 30%;
      height: 100%;
      border-right: 1px solid var(--el-menu-border-color);


    }
  }
</style>
