<script setup lang="ts">
  import { monaco } from '@/utils/customMonaco'
  import { newEditor } from '@/utils/monacoFactory'
  import { onMounted, onUnmounted, ref } from 'vue'
  const editorDom = ref<HTMLDivElement>()
  let editor: monaco.editor.IStandaloneCodeEditor | null = null
  onMounted(() => {
    editor = newEditor(editorDom.value!)
  })
  onUnmounted(() => {
    editor?.dispose()
  })
  defineExpose({
    getCode() {
      return editor?.getModel()?.getValue()
    },
    getSelectedCode() {
      const selection = editor?.getSelection()
      if (!selection)
        return ''
      return editor?.getModel()?.getValueInRange(selection)
    }
  })
</script>

<template>
  <div class="editor-container">
    <div id="editor" ref="editorDom"></div>
  </div>
</template>

<style scoped>
  .editor-container {
    width: 100%;
    height: 100%;
    background: #ffffff;
  }

  #editor {
    width: 100%;
    height: 100%;
  }
</style>
