import { createApp } from 'vue'
import PrimeVue from 'primevue/config'
// 使用 Aura 主题
import Aura from '@primevue/themes/aura'

import App from './App.vue'

const app = createApp(App)

// 配置 PrimeVue
app.use(PrimeVue, {
    theme: {
        preset: Aura
    }
})

app.mount('#app')
