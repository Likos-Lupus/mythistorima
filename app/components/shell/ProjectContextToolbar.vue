<template>
  <header class="project-context-toolbar">
    <div class="project-context-heading">
      <UIcon :name="primary.icon"/>
      <div>
        <strong>{{ primary.label }}</strong>
        <span>{{ workspace.description }}</span>
      </div>
    </div>

    <UTabs
        v-if="workspaces.length > 1"
        :content="false"
        :items="workspaceTabs"
        :model-value="activeMode"
        color="neutral"
        size="sm"
        variant="link"
        @update:model-value="selectWorkspace"
    />

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

const props = defineProps<{
  primary: ProjectPrimaryViewDefinition
  workspace: ProjectWorkspaceDefinition
  workspaces: ProjectWorkspaceDefinition[]
  activeMode: ProjectWorkspaceMode
  focusMode: boolean
}>()

const emit = defineEmits<{
  select: [mode: ProjectWorkspaceMode]
  'toggle-focus': []
  command: []
}>()

const workspaceTabs = computed(() => props.workspaces.map(item => ({
  label: item.label,
  value: item.mode,
  icon: item.icon
})))

function selectWorkspace(value: string | number) {
  emit('select', value as ProjectWorkspaceMode)
}
</script>
