<template>
  <div class="editor-status-bar">
    <div class="editor-status-items">
      <span>{{ characterCount }} 字</span>
      <span>·</span>
      <span>{{ saveText }}</span>
      <span v-if="lastSavedAt">· {{ formatDate(lastSavedAt) }}</span>
    </div>
    <p v-if="errorMessage" class="editor-error">{{ errorMessage }}</p>
  </div>
</template>

<script lang="ts" setup>
import type {SaveState} from '~/composables/useAutoSave'

const props = defineProps<{
  saveState: SaveState
  characterCount: number
  lastSavedAt?: number | null
  errorMessage?: string | null
}>()

const saveText = computed(() => {
  switch (props.saveState) {
    case 'dirty':
      return '有未保存修改'
    case 'saving':
      return '保存中…'
    case 'saved':
      return '已保存'
    case 'failed':
      return '保存失败'
    default:
      return '等待输入'
  }
})

function formatDate(timestamp: number) {
  return new Intl.DateTimeFormat('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  }).format(new Date(timestamp))
}
</script>
