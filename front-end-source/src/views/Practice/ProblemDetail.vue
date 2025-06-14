<script setup lang="ts">
  import { getProblemDetail, type Problem } from '@/apis/problems';
  import eventEmitter from '@/utils/eventEmitter';
  import { ref } from 'vue';
  import { useRoute } from 'vue-router';
  import { ElButton } from 'element-plus'

  const route = useRoute()
  const problem = ref<Problem>()
  const id = +route.query.id!
  async function init() {
    problem.value = await getProblemDetail(id)
  }
  init()
</script>

<template>
  <div class="problem-desc">
    <div class="title">
      {{ problem?.title }}
    </div>
    <div class="description">
      {{ problem?.description }}
    </div>
    <div class="examples">
      {{ problem?.examples }}
    </div>
  </div>
  <div class="action">
    <ElButton @click="eventEmitter.emit('submitCode')" type="primary">提交</ElButton>
  </div>
</template>

<style lang="scss" scoped>
  .problem-desc {
    .title {
      font-size: 24px;
      font-weight: bold;
      margin: 20px 0 0 20px;
    }

    .description {
      font-size: 16px;
      margin: 20px 0 0 20px;
    }

    .examples {
      margin: 20px;
      border: 1px solid var(--el-menu-border-color);
    }

    .problem-desc {
      height: calc(100% - 50px);
    }
  }

  .action {
    border-top: 1px solid var(--el-menu-border-color);
    height: 50px;
    display: flex;
    align-items: center;
    position: fixed;
    /* 新增：固定定位 */
    bottom: 0;
    /* 新增：底部对齐 */
    left: 0;
    /* 新增：左侧对齐 */
    width: 100%;
    /* 新增：占满宽度 */
    background: white;
    /* 新增：设置背景色避免内容被遮挡 */

    &>* {
      margin-left: 10px;
    }
  }
</style>
