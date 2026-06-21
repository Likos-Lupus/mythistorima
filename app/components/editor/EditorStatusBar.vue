<template>
  <div class="editor-status-bar">
    <div class="editor-status-items">
      <span>{{ characterCount }} 字</span>
      <span v-if="targetCharacterCount">/ {{ targetCharacterCount }} 目标</span>
      <span v-if="targetCharacterCount">· {{ progressText }}</span>
      <span>·</span>
      <span>本次 +{{ sessionDelta }} 字</span>
      <span>·</span>
      <span>计时 {{ formatDuration(sessionElapsedMs) }}</span>
      <span>·</span>
      <span>{{ saveText }}</span>
      <span v-if="lastSavedAt">· {{ formatDate(lastSavedAt) }}</span>
    </div>
    <p v-if="errorMessage" class="editor-error">{{ errorMessage }}</p>
  </div>
</template>

<script lang="ts" setup>
import type {SaveState} from '~/composables/useAutoSave'

const props = withDefaults(defineProps<{
  saveState: SaveState
  characterCount: number
  targetCharacterCount?: number | null
  sessionElapsedMs?: number
  sessionDelta?: number
  lastSavedAt?: number | null
  errorMessage?: string | null
}>(), {
  targetCharacterCount: null,
  sessionElapsedMs: 0,
  sessionDelta: 0,
  lastSavedAt: null,
  errorMessage: null
})

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

const progressText = computed(() => {
  if (!props.targetCharacterCount) return ''
  const progress = Math.min(999, Math.round((props.characterCount / props.targetCharacterCount) * 100))
  return `${progress}%`
})

function formatDuration(ms: number) {
  const totalSeconds = Math.floor(ms / 1000)
  const minutes = Math.floor(totalSeconds / 60)
  const seconds = totalSeconds % 60
  return `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`
}

function formatDate(timestamp: number) {
  return new Intl.DateTimeFormat('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  }).format(new Date(timestamp))
}
</script>
