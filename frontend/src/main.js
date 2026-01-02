// 1. 确保从 'vue' 导入 createApp
import { createApp } from 'vue'

// 2. 确保从 'pinia' 导入 createPinia
import { createPinia } from 'pinia'

// 3. 导入你的根组件、路由和全局样式
import App from './App.vue'
import router from './router'
import './assets/main.css'

// 4. 创建 Vue 应用实例
const app = createApp(App)

// 5. 【关键步骤】在 use(router) 之前，先 use(createPinia())
//    createPinia() 会返回一个 pinia 实例
app.use(createPinia()) 
app.use(router)

// 6. 最后挂载应用
app.mount('#app')