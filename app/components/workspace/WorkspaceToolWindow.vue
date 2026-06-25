<template>
  <section
      :aria-label="title"
      :class="[
        'workspace-tool-window',
        `is-${state.side}`,
        `at-${state.position}`,
        dock === 'bottom' ? 'is-bottom-dock' : 'is-side-dock'
      ]"
      :style="windowStyle"
  >
    <header class="workspace-tool-window-header">
      <div class="workspace-tool-window-title">
        <UIcon :name="icon"/>
        <span>{{ title }}</span>
      </div>

      <div class="workspace-tool-window-actions">
        <UDropdownMenu :items="slotItems">
          <UButton
              aria-label="移动工具窗口"
              color="neutral"
              icon="i-lucide-layout-dashboard"
              size="xs"
              variant="ghost"
          />
        </UDropdownMenu>
        <UTooltip text="隐藏窗口">
          <UButton
              aria-label="隐藏工具窗口"
              color="neutral"
              icon="i-lucide-minus"
              size="xs"
              variant="ghost"
              @click="$emit('close', state.id)"
          />
        </UTooltip>
      </div>
    </header>

    <div class="workspace-tool-window-body">
      <slot/>
    </div>

    <button
        :aria-label="resizeLabel"
        :class="['workspace-tool-resize-handle', dock === 'bottom' ? 'is-vertical' : 'is-horizontal']"
        type="button"
        @pointerdown="startResize"
    />
  </section>
</template>

<script lang="ts" setup>
import type {ToolWindowId, ToolWindowPosition, ToolWindowSide, ToolWindowState} from '~/stores/workspaceLayout.store'

const props = withDefaults(defineProps<{
  state: ToolWindowState
  title: string
  icon: string
  dock?: 'side' | 'bottom'
}>(), {
  dock: 'side'
})

const emit = defineEmits<{
  close: [id: ToolWindowId]
  resize: [id: ToolWindowId, size: number]
  'set-slot': [id: ToolWindowId, side: ToolWindowSide, position: ToolWindowPosition]
}>()

const windowStyle = computed(() => ({'--tool-window-size': `${props.state.size}px`}))
const resizeLabel = computed(() => props.dock === 'bottom' ? `调整${props.title}高度` : `调整${props.title}宽度`)

const slotItems = computed(() => [
  [
    {label: '左侧上方', icon: 'i-lucide-panel-left', onSelect: () => emit('set-slot', props.state.id, 'left', 'top')},
    {
      label: '左侧中部',
      icon: 'i-lucide-panel-left-dashed',
      onSelect: () => emit('set-slot', props.state.id, 'left', 'center')
    },
    {
      label: '左侧下方',
      icon: 'i-lucide-panel-bottom',
      onSelect: () => emit('set-slot', props.state.id, 'left', 'bottom')
    }
  ],
  [
    {label: '右侧上方', icon: 'i-lucide-panel-right', onSelect: () => emit('set-slot', props.state.id, 'right', 'top')},
    {
      label: '右侧中部',
      icon: 'i-lucide-panel-right-dashed',
      onSelect: () => emit('set-slot', props.state.id, 'right', 'center')
    },
    {
      label: '右侧下方',
      icon: 'i-lucide-panel-bottom',
      onSelect: () => emit('set-slot', props.state.id, 'right', 'bottom')
    }
  ]
])

function startResize(event: PointerEvent) {
  event.preventDefault()
  const startX = event.clientX
  const startY = event.clientY
  const startSize = props.state.size
  const side = props.state.side
  const dock = props.dock

  const move = (nextEvent: PointerEvent) => {
    if (dock === 'bottom') {
      emit('resize', props.state.id, startSize + startY - nextEvent.clientY)
      return
    }

    const delta = side === 'right'
        ? startX - nextEvent.clientX
        : nextEvent.clientX - startX
    emit('resize', props.state.id, startSize + delta)
  }

  const stop = () => {
    window.removeEventListener('pointermove', move)
    window.removeEventListener('pointerup', stop)
    document.body.style.removeProperty('cursor')
    document.body.style.removeProperty('user-select')
  }

  document.body.style.cursor = dock === 'bottom' ? 'row-resize' : 'col-resize'
  document.body.style.userSelect = 'none'
  window.addEventListener('pointermove', move)
  window.addEventListener('pointerup', stop, {once: true})
}
</script>
