<template>
  <DocumentTree
      :active-document-id="activeDocumentId"
      :items="items"
      @select="documentId => $emit('open-preview', documentId)"
      @open-pinned="documentId => $emit('open-pinned', documentId)"
      @create-document="$emit('create-document', $event)"
      @delete-document="$emit('delete-document', $event)"
      @move-document="$emit('move-document', $event)"
      @rename-document="(documentId, title) => $emit('rename-document', documentId, title)"
      @update-status="(documentId, status) => $emit('update-status', documentId, status)"
  />
</template>

<script lang="ts" setup>
import DocumentTree from '~/components/project/DocumentTree.vue'
import type {DocumentCreatePayload, DocumentStatus, DocumentTreeNode, MoveDocumentInput} from '~/types/document'

withDefaults(defineProps<{
  items: DocumentTreeNode[]
  activeDocumentId?: string | null
}>(), {
  activeDocumentId: null
})

defineEmits<{
  'open-preview': [documentId: string]
  'open-pinned': [documentId: string]
  'create-document': [payload: DocumentCreatePayload]
  'rename-document': [documentId: string, title: string]
  'delete-document': [documentId: string]
  'move-document': [payload: MoveDocumentInput]
  'update-status': [documentId: string, status: DocumentStatus]
}>()
</script>
