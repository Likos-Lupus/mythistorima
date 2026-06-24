<template>
  <section class="project-workspace-content">
    <ProjectDashboard
        v-if="controller.workspaceMode.value === 'dashboard' && controller.projectStore.currentProject"
        :backups="controller.exportStore.backups"
        :project="controller.projectStore.currentProject"
        :saving="controller.projectSaving.value"
        :stats="controller.projectStats.value"
        @backup="controller.createManualBackup"
        @delete-project="controller.deleteCurrentProject"
        @open-mode="controller.selectWorkspaceMode"
        @update-project="controller.updateProjectInfo"
    />

    <template v-else-if="controller.workspaceMode.value === 'writing'">
      <EditorFocusOverlay
          v-if="controller.focusMode.value"
          @exit="controller.toggleFocusMode"
      />

      <NovelEditor
          v-if="controller.documentStore.activeDocumentId"
          :key="controller.documentStore.activeDocumentId"
          :document-id="controller.documentStore.activeDocumentId"
          :focus-mode="controller.focusMode.value"
          :project-id="controller.projectId.value"
          :settings="controller.settingsStore.editorSettings"
          :show-status-bar="false"
          :target-character-count="controller.activeDocumentTarget.value"
          @saved="controller.handleSaved"
          @session="controller.handleEditorSession"
          @status="controller.handleEditorStatus"
          @paragraph-note="controller.handleParagraphNote"
          @toggle-focus-mode="controller.toggleFocusMode"
      />

      <UEmpty
          v-else
          class="project-workspace-empty"
          description="从项目管理器选择文档，或创建新的卷和章节。"
          icon="i-lucide-file-plus-2"
          title="请选择写作文档"
      />
    </template>

    <OutlineWorkspace
        v-else-if="controller.workspaceMode.value === 'outline'"
        :documents="controller.documentStore.documents"
        :project-id="controller.projectId.value"
        @open-document="targetId => controller.openTarget({type: 'document', targetId, source: 'outline'})"
    />

    <OutlineBoardWorkspace
        v-else-if="controller.workspaceMode.value === 'board'"
        :documents="controller.documentStore.documents"
        :project-id="controller.projectId.value"
        @open-document="targetId => controller.openTarget({type: 'document', targetId, source: 'outline'})"
    />

    <TimelineWorkspace
        v-else-if="controller.workspaceMode.value === 'timeline'"
        :documents="controller.documentStore.documents"
        :project-id="controller.projectId.value"
        @open-target="controller.openTarget"
    />

    <CardWorkspace
        v-else-if="controller.workspaceMode.value === 'cards'"
        :project-id="controller.projectId.value"
    />

    <RelationWorkspace
        v-else-if="controller.workspaceMode.value === 'relations'"
        :project-id="controller.projectId.value"
        @open-target="controller.openTarget"
    />

    <StatsWorkspace
        v-else-if="controller.workspaceMode.value === 'stats'"
        :documents="controller.documentStore.documents"
        :project-id="controller.projectId.value"
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
import NovelEditor from '~/components/editor/NovelEditor.vue'
import EditorFocusOverlay from '~/components/editor/EditorFocusOverlay.vue'
import CardWorkspace from '~/components/cards/CardWorkspace.vue'
import NoteWorkspace from '~/components/notes/NoteWorkspace.vue'
import SearchWorkspace from '~/components/search/SearchWorkspace.vue'
import ExportWorkspace from '~/components/export/ExportWorkspace.vue'
import OutlineWorkspace from '~/components/outline/OutlineWorkspace.vue'
import OutlineBoardWorkspace from '~/components/outline/OutlineBoardWorkspace.vue'
import TimelineWorkspace from '~/components/timeline/TimelineWorkspace.vue'
import RelationWorkspace from '~/components/relations/RelationWorkspace.vue'
import ForeshadowWorkspace from '~/components/foreshadow/ForeshadowWorkspace.vue'
import StatsWorkspace from '~/components/stats/StatsWorkspace.vue'
import ProofreadingWorkspace from '~/components/proofreading/ProofreadingWorkspace.vue'

const controller = useProjectWorkspaceContext()
</script>
