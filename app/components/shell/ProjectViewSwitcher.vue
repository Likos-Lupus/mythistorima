<template>
  <nav aria-label="项目视图" class="project-view-switcher">
    <TransitionGroup class="project-view-row" name="project-view-tab" tag="div">
      <div
          v-for="entry in navigationEntries"
          :key="entry.key"
          :class="[
            'project-view-entry',
            {
              'is-subviews': entry.kind === 'subviews',
              'is-active-view': entry.kind === 'view' && entry.view.id === activePrimaryView
            }
          ]"
      >
        <UTooltip v-if="entry.kind === 'view'" :text="entry.view.label" :disabled="entry.view.id === activePrimaryView">
          <UButton
              :aria-current="entry.view.id === activePrimaryView ? 'page' : undefined"
              :aria-label="entry.view.label"
              :class="['project-view-main-tab', { 'is-active': entry.view.id === activePrimaryView }]"
              :icon="entry.view.icon"
              :label="entry.view.id === activePrimaryView ? entry.view.label : undefined"
              color="neutral"
              size="sm"
              variant="ghost"
              @click="selectPrimary(entry.view.id)"
          />
        </UTooltip>

        <div v-else class="project-subview-rail">
          <UTabs
              :content="false"
              :items="secondaryItems"
              :model-value="activeMode"
              :ui="secondaryTabsUi"
              class="project-view-tabs project-view-subtabs"
              color="primary"
              size="sm"
              variant="link"
              @update:model-value="selectWorkspace"
          />
        </div>
      </div>
    </TransitionGroup>
  </nav>
</template>

<script lang="ts" setup>
import {
  getSecondaryWorkspaces,
  type ProjectPrimaryView,
  projectPrimaryViews,
  type ProjectWorkspaceMode
} from '~/constants/projectViews'

const shellStore = useProjectShellStore()

const activePrimaryView = computed(() => shellStore.activePrimaryView)
const activeMode = computed(() => shellStore.workspaceMode)

const secondaryItems = computed(() => getSecondaryWorkspaces(activePrimaryView.value).map(workspace => ({
  label: workspace.label,
  value: workspace.mode,
  icon: workspace.icon
})))

type ProjectPrimaryViewDefinition = typeof projectPrimaryViews[number]

type ProjectViewEntry =
  | { kind: 'view', key: string, view: ProjectPrimaryViewDefinition }
  | { kind: 'subviews', key: string }

const navigationEntries = computed<ProjectViewEntry[]>(() => projectPrimaryViews.flatMap(view => {
  const entries: ProjectViewEntry[] = [{ kind: 'view', key: `view-${view.id}`, view }]

  if (view.id === activePrimaryView.value && secondaryItems.value.length > 1) {
    entries.push({ kind: 'subviews', key: `subviews-${view.id}` })
  }

  return entries
}))

const secondaryTabsUi = {
  root: 'project-secondary-tabs-root',
  list: 'project-secondary-tabs-list',
  trigger: 'project-secondary-tabs-trigger',
  indicator: 'project-secondary-tabs-indicator',
  leadingIcon: 'project-secondary-tabs-icon',
  label: 'project-secondary-tabs-label'
}

function selectPrimary(value: ProjectPrimaryView) {
  shellStore.selectPrimaryView(value)
}

function selectWorkspace(value: string | number) {
  shellStore.selectWorkspaceMode(value as ProjectWorkspaceMode)
}
</script>
