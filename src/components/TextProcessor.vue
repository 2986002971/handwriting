<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
// 改用 Panel 替代 Card
import Panel from 'primevue/panel'
import Textarea from 'primevue/textarea'
import Button from 'primevue/button'

const inputText = ref('')
const processedHtml = ref('')
const settings = ref({
    font_sizes: [18.5, 18.5, 18.5, 19, 18],
    font_names: ['萌妹子体', '张维镜手写楷书', '手写大象体', '陈静的字完整版', '汉仪晨妹子W'],
    line_spacing: 25,
    position_offset: 1,
    spacing_offset: 0
})

const processText = async () => {
    processedHtml.value = await invoke('process_text', {
        text: inputText.value,
        settings: settings.value
    })
}

const copyToClipboard = async () => {
    try {
        // 创建一个临时的div来保存格式化的内容
        const tempDiv = document.createElement('div')
        tempDiv.innerHTML = processedHtml.value
        
        // 将内容添加到剪贴板
        await navigator.clipboard.write([
            new ClipboardItem({
                'text/html': new Blob([tempDiv.innerHTML], { type: 'text/html' }),
                'text/plain': new Blob([tempDiv.textContent], { type: 'text/plain' })
            })
        ])
    } catch (err) {
        console.error('复制失败:', err)
    }
}
</script>

<template>
    <div class="text-processor">
        <div class="section-header mb-2">
            <div class="flex align-items-center gap-2">
                <i class="pi pi-file-edit"></i>
                <span>文本处理器</span>
            </div>
        </div>
        
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
                label="复制到剪贴板"
                icon="pi pi-copy"
                @click="copyToClipboard"
                severity="secondary"
                size="large"
            />
        </div>

        <Panel 
            v-if="processedHtml"
            class="preview-panel"
        >
            <template #header>
                <div class="flex align-items-center gap-2">
                    <i class="pi pi-eye"></i>
                    <span>预览结果</span>
                </div>
            </template>
            <div v-html="processedHtml" class="preview-content" />
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