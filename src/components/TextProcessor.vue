<script setup>
import { ref, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import Panel from 'primevue/panel'
import Textarea from 'primevue/textarea'
import Button from 'primevue/button'
import { getCurrentWindow } from '@tauri-apps/api/window'
import Slider from 'primevue/slider'
import InputNumber from 'primevue/inputnumber'
import Message from 'primevue/message'

const inputText = ref('')
const processedHtml = ref('')
const previewRef = ref(null)
const settings = ref({
    font_sizes: [],
    font_names: ['1', '2', '3', '4'],
    line_spacing: 25,
    line_spacing_random: 5,
    spacing_offset: 0,
    spacing_offset_random: 2,
    italic_probability: 13,
    bold_probability: 27,
    indent_size: 2
})

const fontStyles = ref('')

const processText = async () => {
    updateFontSizes()
    processedHtml.value = await invoke('process_text', {
        text: inputText.value,
        settings: settings.value
    })
}

const copyToClipboard = async () => {
    try {
        await writeText(document.querySelector('.preview-content').textContent)
    } catch (err) {
        console.error('复制失败:', err)
    }
}

// 加载内嵌字体
const loadEmbeddedFonts = async () => {
    try {
        const fontCss = await invoke('get_embedded_fonts')
        const styleElement = document.createElement('style')
        styleElement.textContent = fontCss.join('\n')
        document.head.appendChild(styleElement)
        fontStyles.value = fontCss.join('\n')
    } catch (err) {
        console.error('加载字体失败:', err)
    }
}

// 在组件挂载时加载字体
onMounted(() => {
    loadEmbeddedFonts()
    updateFontSizes()
})

const isExporting = ref(false)

const exportPNG = async () => {
    if (!previewRef.value || isExporting.value) return
    
    isExporting.value = true
    try {
        const element = previewRef.value
        const rect = element.getBoundingClientRect()
        const scale = window.devicePixelRatio || 1
        
        // 获取窗口信息
        const appWindow = await getCurrentWindow()
        const position = await appWindow.innerPosition()
        
        // 考虑窗口标题栏高度（通常是 32px）
        const titleBarHeight = 32
        
        // 计算实际位置
        const actualX = Math.round(rect.left * scale)
        const actualY = Math.round((rect.top + titleBarHeight) * scale)

        const screenshot = await invoke('capture_screenshot', {
            x: actualX,
            y: actualY,
            width: Math.round(rect.width * scale),
            height: Math.round(rect.height * scale)
        })
        
        // 创建 Blob 并下载
        const blob = new Blob([new Uint8Array(screenshot)], { type: 'image/png' })
        const url = URL.createObjectURL(blob)
        const a = document.createElement('a')
        a.href = url
        a.download = 'handwriting.png'
        document.body.appendChild(a)
        a.click()
        document.body.removeChild(a)
        URL.revokeObjectURL(url)
    } catch (err) {
        console.error('导出PNG失败:', err)
    } finally {
        isExporting.value = false
    }
}

// 字号范围控制
const minFontSize = ref(16)
const maxFontSize = ref(20)
const fontSizeStep = 0.25

// 更新字号数组
const updateFontSizes = () => {
    const sizes = []
    for (let i = 0; i < 10; i++) {
        const size = minFontSize.value + 
            Math.random() * (maxFontSize.value - minFontSize.value)
        sizes.push(Number(size.toFixed(2)))
    }
    settings.value.font_sizes = [...sizes].sort((a, b) => a - b)
}

// 添加 watch 来监听字体大小的变化
watch([minFontSize, maxFontSize], () => {
    updateFontSizes()
}, { immediate: true })
</script>

<template>
    <div class="text-processor">
        <Textarea
            v-model="inputText"
            placeholder="请输入要处理的文本"
            :rows="5"
            class="w-full input-textarea"
            autoResize
        />

        <Panel header="文本设置" class="mb-3">
            <div class="grid">
                <!-- 字号范围设置 -->
                <div class="col-12 md:col-6 mb-3">
                    <label class="block mb-2">字号范围</label>
                    <div class="flex align-items-center gap-2">
                        <InputNumber 
                            v-model="minFontSize" 
                            :min="12" 
                            :max="maxFontSize"
                            :step="0.25"
                            @change="updateFontSizes"
                            :minFractionDigits="2"
                            :maxFractionDigits="2"
                        />
                        <span>至</span>
                        <InputNumber 
                            v-model="maxFontSize" 
                            :min="minFontSize" 
                            :max="30"
                            :step="0.25"
                            @change="updateFontSizes"
                            :minFractionDigits="2"
                            :maxFractionDigits="2"
                        />
                    </div>
                </div>

                <!-- 行距设置 -->
                <div class="col-12 md:col-6 mb-3">
                    <label class="block mb-2">行距 ({{ settings.line_spacing }}px)</label>
                    <Slider 
                        v-model="settings.line_spacing"
                        :min="15"
                        :max="35"
                        class="w-full"
                    />
                </div>

                <!-- 行距随机范围 -->
                <div class="col-12 md:col-6 mb-3">
                    <label class="block mb-2">行距随机范围 (±{{ settings.line_spacing_random }}px)</label>
                    <Slider 
                        v-model="settings.line_spacing_random"
                        :min="0"
                        :max="10"
                        :step="0.5"
                        class="w-full"
                    />
                </div>

                <!-- 间距偏移 -->
                <div class="col-12 md:col-6 mb-3">
                    <label class="block mb-2">间距偏移 ({{ settings.spacing_offset }}px)</label>
                    <Slider 
                        v-model="settings.spacing_offset"
                        :min="-10"
                        :max="10"
                        :step="0.1"
                        class="w-full"
                    />
                </div>

                <!-- 间距随机范围 -->
                <div class="col-12 md:col-6 mb-3">
                    <label class="block mb-2">间距随机范围 (±{{ settings.spacing_offset_random }}px)</label>
                    <Slider 
                        v-model="settings.spacing_offset_random"
                        :min="0"
                        :max="5"
                        :step="0.1"
                        class="w-full"
                    />
                </div>

                <!-- 斜体概率 -->
                <div class="col-12 md:col-6 mb-3">
                    <label class="block mb-2">斜体概率 (1/{{ settings.italic_probability }})</label>
                    <Slider 
                        v-model="settings.italic_probability"
                        :min="5"
                        :max="30"
                        class="w-full"
                    />
                </div>

                <!-- 粗体概率 -->
                <div class="col-12 md:col-6 mb-3">
                    <label class="block mb-2">粗体概率 (1/{{ settings.bold_probability }})</label>
                    <Slider 
                        v-model="settings.bold_probability"
                        :min="5"
                        :max="50"
                        class="w-full"
                    />
                </div>

                <!-- 首行缩进 -->
                <div class="col-12 md:col-6 mb-3">
                    <label class="block mb-2">首行缩进 ({{ settings.indent_size }}字符)</label>
                    <Slider 
                        v-model="settings.indent_size"
                        :min="0"
                        :max="4"
                        :step="0.5"
                        class="w-full"
                    />
                </div>
            </div>
        </Panel>

        <div class="flex justify-content-between mb-3">
            <Button 
                label="处理文本" 
                icon="pi pi-refresh" 
                @click="processText"
                severity="primary"
                size="large"
            />
            
            <div class="flex align-items-center gap-6">                
                <Button
                    label="导出PNG"
                    icon="pi pi-download"
                    @click="exportPNG"
                    severity="secondary"
                    size="large"
                    :loading="isExporting"
                />
                <Message 
                    severity="secondary" 
                    :closable="false"
                    class="inline-message"
                >
                    <i class="pi pi-info-circle mr-2"></i>
                    <span>窗口非最大化状态下导出png可能不准确，建议使用系统截图</span>
                </Message>
            </div>
        </div>

        <Panel 
            v-if="processedHtml"
            class="preview-panel"
        >
            <template #header>
                <div class="flex align-items-center gap-2">
                    <i class="pi pi-eye"></i>
                    <span>预览结果（可拖拽右下角调整大小）</span>
                </div>
            </template>
            <div 
                ref="previewRef"
                v-html="processedHtml" 
                class="preview-content resizable" 
            />
        </Panel>
    </div>
</template>

<style>
.text-processor {
    width: 100%;
    padding: 1rem;
}

.preview-panel {
    width: 100%;
}

.preview-content {
    min-height: 250px;
    padding: 1rem;
    resize: both; /* 允许双向调整大小 */
    overflow: auto; /* 必须的，使resize生效 */
    border: 1px solid #ddd;
    background: white;
}

.resizable {
    position: relative;
    max-width: 100%;
    max-height: 800px;
}

/* 添加一个小手柄的视觉提示 */
.resizable::after {
    content: '';
    position: absolute;
    bottom: 0;
    right: 0;
    width: 10px;
    height: 10px;
    cursor: se-resize;
    background: linear-gradient(
        135deg,
        transparent 0%,
        transparent 50%,
        #ccc 50%,
        #ccc 100%
    );
}

.flex.justify-content-between {
    width: 100%;
}

/* 修改 Textarea 的样式 */
.input-textarea {
    min-height: 250px;
    font-size: 1.1em;
    width: 100% !important;
    background: transparent;
}

/* 新增样式 */
.p-slider {
    margin-top: 0.5rem;
}

.p-inputnumber {
    width: 5rem;
}

/* 添加自定义样式使消息更紧凑 */
.inline-message {
    margin: 0 !important;
}

.inline-message .p-message-wrapper {
    padding: 0.5rem 1rem !important;
}

/* 确保消息组件在小屏幕上也能正常显示 */
@media screen and (max-width: 768px) {
    .flex.justify-content-between {
        flex-direction: column;
        gap: 1rem;
    }
    
    .flex.align-items-center {
        flex-direction: column;
        width: 100%;
    }
    
    .inline-message {
        width: 100%;
    }
}
</style> 