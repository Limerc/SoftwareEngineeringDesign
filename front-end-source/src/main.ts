import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'

// svgs
import 'virtual:svg-icons-register'
// mockjs
// import '@/mock/'

const app = createApp(App)

app.use(createPinia())
  .use(router)
  .use(ElementPlus)

app.mount('#app')
