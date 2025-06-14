<script setup lang="ts">
  import SvgIcon from '@/components/SvgIcon.vue';
  import { useUserStore } from '@/stores/userStore';
  import { ElButton, ElIcon, ElMenu, ElMenuItem } from 'element-plus';

  const userStore = useUserStore()

</script>

<template>
  <ElMenu style="width: 100%;" mode="horizontal" :ellipsis="false" default-active="/main/problems" router>
    <ElMenuItem>
      <SvgIcon style="width: 50px;
      height: 50px;" name="logo" />
    </ElMenuItem>
    <ElMenuItem index="/main/problems">刷题题单</ElMenuItem>
    <ElMenuItem index="/main/videos">学习视频</ElMenuItem>
    <ElMenuItem v-if="userStore.getToken()">
      欢迎！{{ userStore.getUser()?.username }}
    </ElMenuItem>
    <ElMenuItem v-else>
      <ElButton>登录</ElButton>
    </ElMenuItem>

  </ElMenu>
  <RouterView />
</template>

<style lang="scss" scoped>
  .el-menu--horizontal>.el-menu-item:last-child {
    margin-left: auto;
  }
</style>
