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
        v-model="value"
        :extensions="extensions"
        class="editor-content"
        v-show="viewMode === 'edit' || viewMode === 'split'"
        :style="{ width: viewMode === 'split' ? '58%' : '100%' }"
      />

      <!-- 预览框 -->
      <div
        class="preview-content"
        v-html="preview"
        v-show="viewMode === 'preview' || viewMode === 'split'"
        :style="{ 
          width: viewMode === 'split' ? '42%' : '100%',
          left: viewMode === 'split' ? '58%' : '0'
        }"
      ></div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { Codemirror } from 'vue-codemirror'
import { basicSetup, EditorView } from 'codemirror'
import { markdown } from '@codemirror/lang-markdown'
import { marked } from 'marked'

const props = defineProps({ modelValue: String })
const emit = defineEmits(['update:modelValue'])

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
const preview = computed(() => marked.parse(value.value || ''))
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
</style>