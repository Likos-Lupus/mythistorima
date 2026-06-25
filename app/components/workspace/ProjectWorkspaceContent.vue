<template>
  <section class="project-workspace-content">
    <ProjectDashboard
        v-if="controller.workspaceMode.value === 'dashboard' && controller.projectStore.currentProject"
        :backups="controller.exportStore.backups"
        :overview="controller.projectOverview.value"
        :project="controller.projectStore.currentProject"
        :saving="controller.projectSaving.value"
        :stats="controller.projectStats.value"
        @backup="controller.createManualBackup"
        @delete-project="controller.deleteCurrentProject"
        @open-mode="controller.selectWorkspaceMode"
        @open-target="controller.openTarget"
        @update-project="controller.updateProjectInfo"
    />

    <WritingWorkspace v-else-if="controller.workspaceMode.value === 'writing'"/>

    <OutlinePlanningWorkspace
        v-else-if="['board', 'outline', 'timeline'].includes(controller.workspaceMode.value)"
        :documents="controller.documentStore.documents"
        :mode="controller.workspaceMode.value"
        :project-id="controller.projectId.value"
        @select-mode="controller.selectWorkspaceMode"
        @open-document="targetId => controller.openTarget({type: 'document', targetId, source: 'outline'})"
    />

    <ResourcesWorkspace
        v-else-if="['cards', 'relations'].includes(controller.workspaceMode.value)"
        :mode="controller.workspaceMode.value"
        :project-id="controller.projectId.value"
        @select-mode="controller.selectWorkspaceMode"
        @open-target="controller.openTarget"
    />


    <NoteWorkspace
        v-else-if="controller.workspaceMode.value === 'notes'"
        :active-document-id="controller.documentStore.activeDocumentId"
        :documents="controller.documentStore.documents"
        :project-id="controller.projectId.value"
    />

    <ForeshadowWorkspace
        v-else-if="controller.workspaceMode.value === 'foreshadow'"
        :documents="controller.documentStore.documents"
        :project-id="controller.projectId.value"
        @open-target="controller.openTarget"
    />

    <ProofreadingWorkspace
        v-else-if="controller.workspaceMode.value === 'proofreading'"
        :active-document-id="controller.documentStore.activeDocumentId"
        :documents="controller.documentStore.documents"
        :project-id="controller.projectId.value"
        @open-target="controller.openTarget"
    />

    <SearchWorkspace
        v-else-if="controller.workspaceMode.value === 'search'"
        :project-id="controller.projectId.value"
        @open-target="controller.openTarget"
    />

    <ExportWorkspace
        v-else-if="controller.workspaceMode.value === 'export'"
        :active-document-id="controller.documentStore.activeDocumentId"
        :documents="controller.documentStore.documents"
        :project-id="controller.projectId.value"
        @imported="controller.handleImportedDocument"
    />

    <UEmpty
        v-else
        class="project-workspace-empty"
        description="请从活动栏选择工作视图。"
        icon="i-lucide-panels-top-left"
        title="未选择工作区"
    />
  </section>
</template>

<script lang="ts" setup>
import ProjectDashboard from '~/components/project/ProjectDashboard.vue'
import NoteWorkspace from '~/components/notes/NoteWorkspace.vue'
import SearchWorkspace from '~/components/search/SearchWorkspace.vue'
import ExportWorkspace from '~/components/export/ExportWorkspace.vue'
import ForeshadowWorkspace from '~/components/foreshadow/ForeshadowWorkspace.vue'
import ProofreadingWorkspace from '~/components/proofreading/ProofreadingWorkspace.vue'
import WritingWorkspace from '~/components/writing/WritingWorkspace.vue'
import OutlinePlanningWorkspace from '~/components/outline/OutlinePlanningWorkspace.vue'
import ResourcesWorkspace from '~/components/resources/ResourcesWorkspace.vue'

const controller = useProjectWorkspaceContext()
</script>
