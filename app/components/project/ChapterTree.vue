<template>
  <div class="chapter-tree">
    <div class="chapter-tree-header">
      <div>
        <h2 class="chapter-title">章节</h2>
        <p class="chapter-subtitle">章节</p>
      </div>
      <button class="primary-button" type="button" @click="$emit('create')">
        新建
      </button>
    </div>

    <div class="chapter-list">
      <button
          v-for="document in documents"
          :key="document.id"
          :class="{ 'is-active': document.id === activeDocumentId }"
          class="chapter-button"
          type="button"
          @click="$emit('select', document.id)"
      >
        <span class="chapter-button-title">{{ document.title }}</span>
        <span class="chapter-button-meta">
          {{ document.characterCount }} 字 · {{ formatDate(document.updatedAt) }}
        </span>
      </button>

      <div v-if="!documents.length" class="empty-panel">
        暂无章节。
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import type {NovelDocument} from '~/types/document'

defineProps<{
  documents: NovelDocument[]
  activeDocumentId?: string | null
}>()

defineEmits<{
  create: []
  select: [documentId: string]
}>()

function formatDate(timestamp: number) {
  return new Intl.DateTimeFormat('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(timestamp))
}
</script>
