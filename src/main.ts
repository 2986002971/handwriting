import { createApp } from 'vue'
import PrimeVue from 'primevue/config'
import Aura from '@primevue/themes/aura'
import Slider from 'primevue/slider'
import InputNumber from 'primevue/inputnumber'

import App from './App.vue'

const app = createApp(App)

// 配置 PrimeVue
app.use(PrimeVue, {
    theme: {
        preset: Aura,
        options: {
            darkModeSelector: false, // 或者使用 'none'
        }
    }
})
app.component('Slider', Slider)
app.component('InputNumber', InputNumber)

app.mount('#app')
