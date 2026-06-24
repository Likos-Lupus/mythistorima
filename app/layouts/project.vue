<template>
  <div :class="{ 'is-focus-mode': shellStore.focusMode }" class="project-layout">
    <ProjectTitlebar
        :command-shortcut="formattedCommandShortcut"
        :document-title="documentStore.activeDocument?.title"
        :project-title="projectStore.currentProject?.title || 'Mythistorima'"
        :save-state="editorStore.saveState"
        :workspace-label="shellStore.activeWorkspaceDefinition.label"
        @command="commandStore.openPalette"
        @home="router.push('/')"
        @settings="shellStore.openSettings"
    />

    <div class="project-layout-body">
      <ProjectActivityBar
          :active-view="shellStore.activePrimaryView"
          @select="shellStore.selectPrimaryView"
      />

      <section class="project-layout-stage">
        <ProjectContextToolbar
            :active-mode="shellStore.workspaceMode"
            :focus-mode="shellStore.focusMode"
            :primary="shellStore.activePrimaryDefinition"
            :workspace="shellStore.activeWorkspaceDefinition"
            :workspaces="shellStore.secondaryWorkspaces"
            @command="commandStore.openPalette"
            @select="shellStore.selectWorkspaceMode"
            @toggle-focus="shellStore.toggleFocusMode"
        />

        <main class="project-layout-content">
          <slot/>
        </main>
      </section>
    </div>

    <ProjectStatusBar
        :current-character-count="editorStore.currentCharacterCount"
        :document-title="documentStore.activeDocument?.title"
        :project-character-count="shellStore.projectCharacterCount"
        :save-state="editorStore.saveState"
        :target-character-count="activeDocumentTarget"
        :today-character-count="timerStore.todayCharacterCount"
        :today-elapsed-ms="timerStore.todayElapsedMs"
    />

    <div v-if="shellStore.feedback" class="command-feedback-toast">
      {{ shellStore.feedback }}
    </div>

    <CommandPalette
        :items="shellStore.commandItems"
        :open="commandStore.isPaletteOpen"
        :open-shortcut="commandStore.shortcutFor('commandPalette.open')"
        @close="commandStore.closePalette"
        @execute="shellStore.executeCommandItem"
    />

    <AppSettingsModal v-model:open="shellStore.settingsOpen"/>
  </div>
</template>

<script lang="ts" setup>
import ProjectTitlebar from '~/components/shell/ProjectTitlebar.vue'
import ProjectActivityBar from '~/components/shell/ProjectActivityBar.vue'
import ProjectContextToolbar from '~/components/shell/ProjectContextToolbar.vue'
import ProjectStatusBar from '~/components/shell/ProjectStatusBar.vue'
import CommandPalette from '~/components/command/CommandPalette.vue'
import AppSettingsModal from '~/components/settings/AppSettingsModal.vue'

const router = useRouter()
const projectStore = useProjectStore()
const documentStore = useDocumentStore()
const editorStore = useEditorStore()
const timerStore = useTimerStore()
const commandStore = useCommandStore()
const shellStore = useProjectShellStore()

const formattedCommandShortcut = computed(() => {
  const shortcut = commandStore.shortcutFor('commandPalette.open')
  const isMac = typeof navigator !== 'undefined' && navigator.platform.includes('Mac')
  return shortcut.replace('Mod', isMac ? '⌘' : 'Ctrl')
})

const activeDocumentTarget = computed(() => {
  const metadataJson = documentStore.activeDocument?.metadataJson
  if (!metadataJson) return null
  try {
    const parsed = JSON.parse(metadataJson) as { targetCharacterCount?: unknown }
    return typeof parsed.targetCharacterCount === 'number' && parsed.targetCharacterCount > 0
        ? parsed.targetCharacterCount
        : null
  } catch {
    return null
  }
})
</script>
