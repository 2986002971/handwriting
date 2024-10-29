<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import Panel from 'primevue/panel'
import Textarea from 'primevue/textarea'
import Button from 'primevue/button'
import { getCurrentWindow } from '@tauri-apps/api/window'
import html2canvas from 'html2canvas'

const inputText = ref('')
const processedHtml = ref('')
const previewRef = ref(null)
const settings = ref({
    font_sizes: [18.5, 18.5, 18.5, 19, 18],
    font_names: ['1', '2', '3', '4'],
    line_spacing: 25,
    position_offset: 1,
    spacing_offset: 0
})

const fontStyles = ref('')

const processText = async () => {
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
})

const isExporting = ref(false)

const exportPNG = async () => {
    if (!previewRef.value || isExporting.value) return
    
    isExporting.value = true
    try {
        const rect = previewRef.value.getBoundingClientRect()
        
        // 获取窗口的滚动位置
        const scrollX = window.scrollX || window.pageXOffset
        const scrollY = window.scrollY || window.pageYOffset
        
        // 获取设备像素比
        const scale = window.devicePixelRatio || 1
        
        // 获取视口（viewport）的位置和大小
        const viewportOffset = previewRef.value.ownerDocument.defaultView.pageYOffset
        
        // 计算实际的屏幕坐标，考虑视口偏移
        const screenshot = await invoke('capture_screenshot', {
            x: Math.round((rect.x + scrollX) * scale),
            // 使用相对于视口的位置，而不是文档的绝对位置
            y: Math.round((rect.top + viewportOffset) * scale),
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

        <div class="flex justify-content-between mb-3">
            <Button 
                label="处理文本" 
                icon="pi pi-refresh" 
                @click="processText"
                severity="primary"
                size="large"
            />
            
            <Button
                label="导出PNG"
                icon="pi pi-download"
                @click="exportPNG"
                severity="secondary"
                size="large"
                :loading="isExporting"
            />
        </div>

        <Panel 
            v-if="processedHtml"
            class="preview-panel"
        >
            <template #header>
                <div class="flex align-items-center gap-2">
                    <i class="pi pi-eye"></i>
                    <span>预览结果（可拖拽调整大小）</span>
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
</style> 