import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'
import monacoEditorPlugin from 'vite-plugin-monaco-editor'
import { createSvgIconsPlugin } from 'vite-plugin-svg-icons'
import path from 'node:path'
// https://vite.dev/config/
export default defineConfig({
  base: './',
  plugins: [
    (monacoEditorPlugin as any).default({
      languageWorkers: ['editorWorkerService']
    }),
    // monacoEditorPlugin({
    //   languageWorkers:['']
    // }),
    vue(),
    vueDevTools(),
    createSvgIconsPlugin({
      iconDirs: [path.resolve(process.cwd(), 'src/assets/icons')],
      symbolId: 'icon-[name]'
    }),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
  server: {
    proxy: {
      '/api1': {
        target: 'http://192.168.200.130:8000',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api1/, '')
      },
      // 新增 AI 服务代理
      '/api2': {
        target: 'http://192.168.200.130:8001',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api2/, '') // 保留原始路径请删除该行
      }
    }
  }
})
