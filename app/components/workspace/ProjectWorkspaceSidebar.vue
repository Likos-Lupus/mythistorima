<template>
  <aside class="project-workspace-sidebar">
    <DocumentTree
        :active-document-id="controller.documentStore.activeDocumentId"
        :items="controller.documentStore.documentTree"
        @select="controller.selectDocument"
        @create-document="controller.createDocument"
        @delete-document="controller.deleteDocument"
        @move-document="controller.moveDocument"
        @rename-document="controller.renameDocument"
        @update-status="controller.updateDocumentStatus"
    />
  </aside>
</template>

<script lang="ts" setup>
import DocumentTree from '~/components/project/DocumentTree.vue'

const controller = useProjectWorkspaceContext()
</script>
