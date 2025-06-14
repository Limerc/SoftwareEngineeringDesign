<script setup lang="ts">
  import { getVideos } from '@/apis/videos';
  import { ElButton, ElInput, ElLink } from 'element-plus';
  import { onMounted, ref } from 'vue';

  const data = ref<{ title: string, url: string }[]>()
  const kw = ref('')

  async function init() {
    data.value = (await getVideos(kw.value)).data

  }

  onMounted(() => {
    init()
  })
</script>

<template>
  <div class="container">
    <div class="mid">
      <div class="op">
        <ElInput v-model="kw" placeholder="请输入关键词" />
        <ElButton @click="init">搜索</ElButton>
      </div>
      <div class="video-list">
        <div v-for="v in data" class="video">
          <h3>{{ v.title }}</h3>
          <ElLink :href="v.url" target="_blank">{{ v.url }}</ElLink>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .container {
    width: 100%;
    height: calc(100vh - 60px);
    display: flex;
    justify-content: center;
    align-items: flex-start;
    background-color: #eee;
    overflow-y: auto;

    .mid {
      min-height: calc(100% - 20px);
      width: 1200px;
      margin: 10px 0;
      background-color: #fff;
      border-radius: 8px;
      padding: 24px;

      .op {
        display: flex;
      }

      .video-list {
        margin-top: 20px;
      }
    }
  }
</style>
