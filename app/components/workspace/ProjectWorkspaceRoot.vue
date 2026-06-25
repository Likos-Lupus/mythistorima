<template>
  <div
      :class="{ 'is-focus-mode': controller.focusMode.value }"
      class="project-workspace-root"
  >
    <ProjectWorkspaceContent/>

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
import ProjectWorkspaceContent from '~/components/workspace/ProjectWorkspaceContent.vue'

const controller = provideProjectWorkspace(useProjectWorkspaceController())
</script>
