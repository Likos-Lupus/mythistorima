<template>
  <footer class="project-status-bar">
    <div class="project-status-section">
      <span :class="`is-${saveState}`" class="project-status-indicator"/>
      <span>{{ saveLabel }}</span>
      <span v-if="documentTitle" class="project-status-document">{{ documentTitle }}</span>
    </div>

    <div class="project-status-section project-status-metrics">
      <span>{{ currentCharacterCount }} 字</span>
      <span v-if="targetCharacterCount">/ {{ targetCharacterCount }} 目标</span>
      <span>今日 +{{ todayCharacterCount }}</span>
      <span>{{ formatDuration(todayElapsedMs) }}</span>
      <span>项目 {{ projectCharacterCount }} 字</span>
    </div>
  </footer>
</template>

<script lang="ts" setup>
import type {SaveState} from '~/composables/useAutoSave'

const props = withDefaults(defineProps<{
  saveState?: SaveState
  documentTitle?: string | null
  currentCharacterCount?: number
  targetCharacterCount?: number | null
  todayCharacterCount?: number
  todayElapsedMs?: number
  projectCharacterCount?: number
}>(), {
  saveState: 'idle',
  documentTitle: null,
  currentCharacterCount: 0,
  targetCharacterCount: null,
  todayCharacterCount: 0,
  todayElapsedMs: 0,
  projectCharacterCount: 0
})

const saveLabel = computed(() => {
  if (props.saveState === 'dirty') return '未保存'
  if (props.saveState === 'saving') return '保存中'
  if (props.saveState === 'saved') return '已保存'
  if (props.saveState === 'failed') return '保存失败'
  return '就绪'
})

function formatDuration(durationMs: number) {
  const totalSeconds = Math.max(0, Math.floor(durationMs / 1000))
  const hours = Math.floor(totalSeconds / 3600)
  const minutes = Math.floor((totalSeconds % 3600) / 60)
  const seconds = totalSeconds % 60
  return [hours, minutes, seconds].map(value => String(value).padStart(2, '0')).join(':')
}
</script>
