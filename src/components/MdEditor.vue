<template>
  <div class="md-editor">
    <!-- 切换工具栏 -->
    <div class="editor-toolbar">
      <el-radio-group v-model="viewMode" size="small" button-style="solid">
        <el-radio-button value="edit"> 编辑 </el-radio-button>
        <el-radio-button value="split"> 混合 </el-radio-button>
        <el-radio-button value="preview"> 预览 </el-radio-button>
      </el-radio-group>
    </div>

    <!-- 编辑区域 -->
    <div class="editor-body">
      <!-- 编辑框 -->
      <codemirror
        ref="codemirrorRef"
        v-model="value"
        :extensions="extensions"
        class="editor-content"
        v-show="viewMode === 'edit' || viewMode === 'split'"
        :style="{ width: viewMode === 'split' ? '58%' : '100%' }"
        @ready="handleReady"
      />

      <!-- 预览框 -->
      <div
        ref="previewRef"
        class="preview-content"
        v-html="preview"
        v-show="viewMode === 'preview' || viewMode === 'split'"
        :style="{ 
          width: viewMode === 'split' ? '42%' : '100%',
          left: viewMode === 'split' ? '58%' : '0'
        }"
        @scroll="handlePreviewScroll"
      ></div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, shallowRef } from 'vue'
import { Codemirror } from 'vue-codemirror'
import { basicSetup, EditorView } from 'codemirror'
import { markdown } from '@codemirror/lang-markdown'
import { marked } from 'marked'

const props = defineProps({ modelValue: String })
const emit = defineEmits(['update:modelValue'])

// 编辑器实例
const editorView = shallowRef(null)

// DOM 元素引用
const codemirrorRef = ref(null)
const previewRef = ref(null)

// 是否正在同步滚动（防止循环触发）
let isSyncing = false

// 默认预览
const viewMode = ref('preview')

const value = computed({
  get: () => props.modelValue || '',
  set: v => emit('update:modelValue', v)
})

const lightTheme = EditorView.theme({
  '&': { backgroundColor: '#ffffff', color: '#222' },
  '.cm-gutters': { backgroundColor: '#f0f9f8', color: '#00796b', border: 'none' },
})

const extensions = [basicSetup, markdown(), lightTheme]
const preview = computed(() => {
  const result = marked(value.value || '')
  return result
})

// 编辑器准备就绪时获取实例
function handleReady(payload) {
  editorView.value = payload.view
  
  // 添加编辑器滚动监听
  const scroller = payload.view.scrollDOM
  scroller.addEventListener('scroll', () => {
    if (viewMode.value === 'split' && !isSyncing) {
      syncScrollFromEditor()
    }
  })
}

// 从编辑器滚动同步到预览
function syncScrollFromEditor() {
  if (!previewRef.value || !editorView.value) return
  
  isSyncing = true
  const editorScroller = editorView.value.scrollDOM
  const previewEl = previewRef.value
  
  // 计算编辑器的滚动比例
  const scrollTop = editorScroller.scrollTop
  const scrollHeight = editorScroller.scrollHeight - editorScroller.clientHeight
  const ratio = scrollHeight > 0 ? scrollTop / scrollHeight : 0
  
  // 应用到预览区域
  const previewScrollHeight = previewEl.scrollHeight - previewEl.clientHeight
  previewEl.scrollTop = ratio * previewScrollHeight
  
  setTimeout(() => { isSyncing = false }, 50)
}

// 从预览滚动同步到编辑器
function syncScrollFromPreview() {
  if (!previewRef.value || !editorView.value) return
  
  isSyncing = true
  const editorScroller = editorView.value.scrollDOM
  const previewEl = previewRef.value
  
  // 计算预览的滚动比例
  const scrollTop = previewEl.scrollTop
  const scrollHeight = previewEl.scrollHeight - previewEl.clientHeight
  const ratio = scrollHeight > 0 ? scrollTop / scrollHeight : 0
  
  // 应用到编辑器
  const editorScrollHeight = editorScroller.scrollHeight - editorScroller.clientHeight
  editorScroller.scrollTop = ratio * editorScrollHeight
  
  setTimeout(() => { isSyncing = false }, 50)
}

// 预览区域滚动事件处理
function handlePreviewScroll() {
  if (viewMode.value === 'split' && !isSyncing) {
    syncScrollFromPreview()
  }
}

// 滚动到指定行
function scrollToLine(lineNumber, forceEditMode = true) {
  if (!editorView.value) return
  
  const view = editorView.value
  
  // 获取指定行的信息
  const line = view.state.doc.line(Math.min(lineNumber, view.state.doc.lines))
  const pos = line.from
  const headingText = line.text.replace(/^#+\s*/, '').trim()
  
  // 预览模式下，先尝试在预览区域滚动
  if (viewMode.value === 'preview' && !forceEditMode && headingText) {
    const previewEl = document.querySelector('.preview-content')
    if (previewEl) {
      const headings = previewEl.querySelectorAll('h1,h2,h3,h4,h5,h6')
      for (const h of headings) {
        if (h.textContent.trim() === headingText) {
          h.scrollIntoView({ behavior: 'smooth', block: 'start' })
          return
        }
      }
    }
  }
  
  // 切换到编辑模式
  if (forceEditMode) {
    viewMode.value = 'edit'
  }
  
  // 聚焦编辑器并平滑滚动到指定行
  view.focus()
  setTimeout(() => {
    view.dispatch({ selection: { anchor: pos, head: pos } })
    
    // 使用原生方法实现平滑滚动
    const scroller = view.scrollDOM
    const targetLine = view.state.doc.line(Math.min(lineNumber, view.state.doc.lines))
    const targetPos = targetLine.from
    
    // 计算目标行的 DOM 位置
    const coords = view.coordsAtPos(targetPos)
    if (coords && scroller) {
      const start = performance.now()
      const duration = 600
      const initialScrollTop = scroller.scrollTop
      const targetScrollTop = coords.top + scroller.scrollTop - scroller.getBoundingClientRect().top
      
      function animate(currentTime) {
        const elapsed = currentTime - start
        const progress = Math.min(elapsed / duration, 1)
        // easeOutCubic 缓动函数
        const easeProgress = 1 - Math.pow(1 - progress, 3)
        scroller.scrollTop = initialScrollTop + (targetScrollTop - initialScrollTop) * easeProgress
        
        if (progress < 1) {
          requestAnimationFrame(animate)
        }
      }
      requestAnimationFrame(animate)
    }
  }, 100)
}

// 暴露方法给父组件
defineExpose({
  scrollToLine
})
</script>

<style scoped>
.md-editor {
  width: 100%;
  height: 100%;
  background: #fff;
  border: 1px solid #e0f2f1;
  border-radius: 8px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.editor-toolbar {
  padding: 6px 12px;
  border-bottom: 1px solid #e0f2f1;
  background: #fafbfc;
}

.editor-body {
  position: relative;
  flex: 1;
  overflow: hidden;
}

.editor-content {
  height: 100%;
  position: absolute;
  top: 0;
  left: 0;
}

.editor-content :deep(.cm-editor) {
  height: 100% !important;
  outline: none;
}

.editor-content :deep(.cm-scroller) {
  overflow-y: auto !important;
}

.preview-content {
  box-sizing: border-box;
  position: absolute;
  top: 0;
  bottom: 0;
  padding: 14px;
  background: #ffffff;
  color: #333;
  font-size: 14px;
  line-height: 1.7;
  overflow-y: auto;
  word-break: break-all;
  border-left: 1px solid #e0f2f1;
}

/* 行内代码样式 */
.preview-content :deep(code) {
  background: #f3f4f6;
  color: #374151;
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'Consolas', monospace;
  font-size: 0.9em;
}

/* 代码块样式 */
.preview-content :deep(pre) {
  background: #f9fafb;
  color: #374151;
  padding: 12px 16px;
  border-radius: 8px;
  border: 1px solid #e5e7eb;
  overflow-x: auto;
  margin: 12px 0;
  font-family: 'Consolas', monospace;
  font-size: 13px;
  line-height: 1.6;
}

.preview-content :deep(pre code) {
  background: transparent;
  color: inherit;
  padding: 0;
  font-size: inherit;
}
</style>