<template>
  <div :aria-label="`${sideLabel}工具窗口栏`" :class="['workspace-tool-strip', `is-${side}`]">
    <div
        v-for="position in positions"
        :key="position"
        :class="['workspace-tool-strip-group', `at-${position}`]"
    >
      <UTooltip
          v-for="window in windowsFor(position)"
          :key="window.id"
          :text="labelFor(window.id)"
      >
        <UButton
            :aria-label="`${window.visible ? '隐藏' : '打开'}${labelFor(window.id)}`"
            :color="window.visible ? 'primary' : 'neutral'"
            :icon="iconFor(window.id)"
            :variant="window.visible ? 'soft' : 'ghost'"
            class="workspace-tool-strip-button"
            size="md"
            @click="$emit('toggle', window.id)"
        />
      </UTooltip>
    </div>
  </div>
</template>

<script lang="ts" setup>
import type {ToolWindowId, ToolWindowPosition, ToolWindowSide, ToolWindowState} from '~/stores/workspaceLayout.store'

const props = defineProps<{
  side: ToolWindowSide
  windows: ToolWindowState[]
}>()

defineEmits<{
  toggle: [id: ToolWindowId]
}>()

const positions: ToolWindowPosition[] = ['top', 'center', 'bottom']
const sideLabel = computed(() => props.side === 'left' ? '左侧' : '右侧')

function windowsFor(position: ToolWindowPosition) {
  return props.windows
      .filter(window => window.side === props.side && window.position === position)
      .sort((a, b) => a.order - b.order)
}

function labelFor(id: ToolWindowId) {
  switch (id) {
    case 'project':
      return '项目树'
    case 'inspector':
      return '章节信息'
    case 'notes':
      return '事项'
    case 'foreshadow':
      return '伏笔'
    case 'proofreading':
      return '校对'
    case 'references':
      return '引用'
    case 'stats':
      return '统计'
    case 'search':
      return '搜索'
  }
}

function iconFor(id: ToolWindowId) {
  switch (id) {
    case 'project':
      return 'i-lucide-list-tree'
    case 'inspector':
      return 'i-lucide-panel-right'
    case 'notes':
      return 'i-lucide-list-checks'
    case 'foreshadow':
      return 'i-lucide-sparkles'
    case 'proofreading':
      return 'i-lucide-spell-check-2'
    case 'references':
      return 'i-lucide-at-sign'
    case 'stats':
      return 'i-lucide-bar-chart-3'
    case 'search':
      return 'i-lucide-search'
  }
}
</script>
