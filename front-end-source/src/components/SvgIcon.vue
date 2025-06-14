<script setup lang="ts">
  const { prefix = "#icon-", name, color = "#000", width = "1em", height = "1em" } = defineProps<{
    //前缀
    prefix?: string,
    //图标名称
    name: string,
    //颜色
    color?: string,
    width?: string,
    height?: string,
  }>()
  // {
  //   //前缀
  //   prefix: {
  //     type: String,
  //     default: "#icon-"
  //   },
  //   //图标名称
  //   name: String,
  //     //颜色
  //     color: {
  //     type: String,
  //     default: "#fff"
  //   },
  //   width: {
  //     type: String,
  //     default: "20px"
  //   },
  //   height: {
  //     type: String,
  //     default: "20px"
  //   },
  // }
</script>
<template>
  <svg :style="{ width, height }">
    <use :xlink:href="prefix + name"></use>
  </svg>
</template>
