<template>
  <section
      :class="['outline-board-column', { 'is-drag-over': isDragOver }]"
      @dragleave="isDragOver = false"
      @dragenter.prevent="isDragOver = true"
      @dragover.prevent="isDragOver = true"
      @drop.prevent="handleDrop"
  >
    <header class="outline-board-column-header">
      <div>
        <h2>{{ title }}</h2>
        <p>{{ description }}</p>
      </div>
      <span>{{ items.length }}</span>
    </header>

    <div class="outline-board-column-list">
      <article
          v-for="item in items"
          :key="item.id"
          :class="['outline-board-card', { 'is-active': item.id === activeOutlineNodeId }]"
          draggable="true"
          @click="$emit('select', item.id)"
          @dragstart="handleDragStart($event, item.id)"
      >
        <div class="outline-board-card-header">
          <span class="outline-tree-type">{{ outlineTypeLabel(item.type) }}</span>
          <strong>{{ item.title }}</strong>
        </div>
        <p v-if="item.summary">{{ item.summary }}</p>
        <small>
          {{ outlineStatusLabel(item.status) }}
          <template v-if="item.linkedDocument"> · {{
              documentTypeLabel(item.linkedDocument.type)
            }}《{{ item.linkedDocument.title }}》
          </template>
          <template v-if="item.depth > 0"> · 层级 {{ item.depth + 1 }}</template>
        </small>
        <div class="outline-board-card-actions" @click.stop>
          <button
              v-for="target in quickStatuses"
              :key="target"
              :disabled="target === item.status"
              class="tree-action-button"
              type="button"
              @click="$emit('changeStatus', item.id, target)"
          >
            {{ outlineStatusLabel(target) }}
          </button>
          <button
              v-if="item.linkedDocumentId"
              class="tree-action-button"
              type="button"
              @click="$emit('openDocument', item.linkedDocumentId)"
          >
            跳转章节
          </button>
        </div>
      </article>

      <div v-if="!items.length" class="outline-board-empty">
        将大纲节点拖到这里，或在节点卡片上点击状态按钮。
      </div>
    </div>
  </section>
</template>

<script lang="ts" setup>
import type {OutlineNodeStatus, OutlineTreeNode} from '~/types/outline'
import type {OutlineBoardStatus} from '~/utils/outlinePresentation'
import {
  documentTypeLabel,
  OUTLINE_BOARD_STATUSES,
  outlineStatusLabel,
  outlineTypeLabel
} from '~/utils/outlinePresentation'

const props = defineProps<{
  status: OutlineBoardStatus
  title: string
  description: string
  items: OutlineTreeNode[]
  activeOutlineNodeId?: string | null
}>()

const emit = defineEmits<{
  select: [outlineNodeId: string]
  changeStatus: [outlineNodeId: string, status: OutlineNodeStatus]
  openDocument: [documentId: string]
}>()

const isDragOver = ref(false)
const quickStatuses = OUTLINE_BOARD_STATUSES

function handleDragStart(event: DragEvent, outlineNodeId: string) {
  event.dataTransfer?.setData('application/x-mythistorima-outline-node', outlineNodeId)
  event.dataTransfer?.setData('text/plain', outlineNodeId)
  if (event.dataTransfer) event.dataTransfer.effectAllowed = 'move'
}

function handleDrop(event: DragEvent) {
  isDragOver.value = false
  const outlineNodeId = event.dataTransfer?.getData('application/x-mythistorima-outline-node') || event.dataTransfer?.getData('text/plain')
  if (!outlineNodeId) return
  emit('changeStatus', outlineNodeId, props.status)
}
</script>
