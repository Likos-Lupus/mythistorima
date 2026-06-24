<template>
  <div
      :class="{
        'is-focus-mode': controller.focusMode.value,
        'has-sidebar': showSidebar,
        'has-inspector': showInspector
      }"
      class="project-workspace-root"
  >
    <ProjectWorkspaceSidebar v-if="showSidebar"/>
    <ProjectWorkspaceContent/>
    <ProjectWorkspaceInspector v-if="showInspector"/>

    <UAlert
        v-if="controller.pageError.value"
        :description="controller.pageError.value"
        class="project-workspace-error"
        close
        color="error"
        icon="i-lucide-circle-alert"
        title="工作区操作失败"
        variant="subtle"
        @close="controller.pageError.value = null"
    />
  </div>
</template>

<script lang="ts" setup>
import ProjectWorkspaceSidebar from '~/components/workspace/ProjectWorkspaceSidebar.vue'
import ProjectWorkspaceContent from '~/components/workspace/ProjectWorkspaceContent.vue'
import ProjectWorkspaceInspector from '~/components/workspace/ProjectWorkspaceInspector.vue'

const controller = provideProjectWorkspace(useProjectWorkspaceController())

const showSidebar = computed(() =>
    controller.workspaceMode.value === 'writing' && !controller.focusMode.value
)
const showInspector = computed(() =>
    !controller.focusMode.value && controller.workspaceMode.value !== 'dashboard'
)
</script>
