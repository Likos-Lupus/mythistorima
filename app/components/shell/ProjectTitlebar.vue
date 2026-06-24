<template>
  <header class="project-titlebar">
    <div class="project-titlebar-leading">
      <UTooltip text="返回项目列表">
        <UButton
            aria-label="返回项目列表"
            color="neutral"
            icon="i-lucide-panel-left-close"
            size="sm"
            variant="ghost"
            @click="$emit('home')"
        />
      </UTooltip>

      <UBreadcrumb :items="breadcrumbItems" class="min-w-0"/>
    </div>

    <div class="project-titlebar-actions">
      <UBadge :color="saveBadge.color" size="sm" variant="subtle">
        {{ saveBadge.label }}
      </UBadge>

      <UTooltip text="命令面板">
        <UButton
            aria-label="打开命令面板"
            color="neutral"
            icon="i-lucide-command"
            size="sm"
            variant="ghost"
            @click="$emit('command')"
        >
          <template #trailing>
            <UKbd size="sm">{{ commandShortcut }}</UKbd>
          </template>
        </UButton>
      </UTooltip>

      <UTooltip text="设置">
        <UButton
            aria-label="打开设置"
            color="neutral"
            icon="i-lucide-settings"
            size="sm"
            variant="ghost"
            @click="$emit('settings')"
        />
      </UTooltip>
    </div>
  </header>
</template>

<script lang="ts" setup>
import type {SaveState} from '~/composables/useAutoSave'

const props = withDefaults(defineProps<{
  projectTitle?: string
  workspaceLabel?: string
  documentTitle?: string | null
  saveState?: SaveState
  commandShortcut?: string
}>(), {
  projectTitle: 'Mythistorima',
  workspaceLabel: '',
  documentTitle: null,
  saveState: 'idle',
  commandShortcut: 'Ctrl+K'
})

defineEmits<{
  home: []
  command: []
  settings: []
}>()

const breadcrumbItems = computed(() => [
  {label: props.projectTitle},
  ...(props.workspaceLabel ? [{label: props.workspaceLabel}] : []),
  ...(props.documentTitle ? [{label: props.documentTitle}] : [])
])

const saveBadge = computed(() => {
  switch (props.saveState) {
    case 'dirty':
      return {label: '未保存', color: 'warning' as const}
    case 'saving':
      return {label: '保存中', color: 'info' as const}
    case 'saved':
      return {label: '已保存', color: 'success' as const}
    case 'failed':
      return {label: '保存失败', color: 'error' as const}
    default:
      return {label: '就绪', color: 'neutral' as const}
  }
})
</script>
