<template>
  <header class="project-context-toolbar">
    <div class="project-context-heading">
      <UIcon :name="primary.icon"/>
      <div>
        <strong>{{ primary.label }}</strong>
        <span>{{ workspace.description }}</span>
      </div>
    </div>

    <UButtonGroup v-if="workspaces.length > 1" size="sm">
      <UButton
          v-for="item in workspaces"
          :key="item.mode"
          :color="activeMode === item.mode ? 'primary' : 'neutral'"
          :icon="item.icon"
          :label="item.label"
          :variant="activeMode === item.mode ? 'soft' : 'ghost'"
          @click="$emit('select', item.mode)"
      />
    </UButtonGroup>

    <div class="project-context-actions">
      <UTooltip v-if="primary.id === 'writing'" text="切换专注模式">
        <UButton
            :aria-label="focusMode ? '退出专注模式' : '进入专注模式'"
            :color="focusMode ? 'primary' : 'neutral'"
            :variant="focusMode ? 'soft' : 'ghost'"
            icon="i-lucide-focus"
            size="sm"
            @click="$emit('toggle-focus')"
        />
      </UTooltip>
      <UTooltip text="打开命令面板">
        <UButton
            aria-label="打开命令面板"
            color="neutral"
            icon="i-lucide-search"
            size="sm"
            variant="ghost"
            @click="$emit('command')"
        />
      </UTooltip>
    </div>
  </header>
</template>

<script lang="ts" setup>
import type {
  ProjectPrimaryViewDefinition,
  ProjectWorkspaceDefinition,
  ProjectWorkspaceMode
} from '~/constants/projectViews'

defineProps<{
  primary: ProjectPrimaryViewDefinition
  workspace: ProjectWorkspaceDefinition
  workspaces: ProjectWorkspaceDefinition[]
  activeMode: ProjectWorkspaceMode
  focusMode: boolean
}>()

defineEmits<{
  select: [mode: ProjectWorkspaceMode]
  'toggle-focus': []
  command: []
}>()
</script>
