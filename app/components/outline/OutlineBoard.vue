<template>
  <section class="outline-board">
    <OutlineBoardColumn
        v-for="column in columns"
        :key="column.status"
        :active-outline-node-id="activeOutlineNodeId"
        :description="column.description"
        :items="itemsFor(column.status)"
        :status="column.status"
        :title="column.label"
        @select="$emit('select', $event)"
        @change-status="forwardStatus"
        @open-document="$emit('openDocument', $event)"
    />
  </section>
</template>

<script lang="ts" setup>
import OutlineBoardColumn from '~/components/outline/OutlineBoardColumn.vue'
import type {OutlineNodeStatus, OutlineTreeNode} from '~/types/outline'
import type {OutlineBoardStatus} from '~/utils/outlinePresentation'
import {OUTLINE_BOARD_COLUMNS} from '~/utils/outlinePresentation'

const props = defineProps<{
  items: OutlineTreeNode[]
  activeOutlineNodeId?: string | null
}>()

const emit = defineEmits<{
  select: [outlineNodeId: string]
  changeStatus: [outlineNodeId: string, status: OutlineNodeStatus]
  openDocument: [documentId: string]
}>()

const columns = OUTLINE_BOARD_COLUMNS

function itemsFor(status: OutlineBoardStatus) {
  return props.items.filter(item => item.status === status)
}

function forwardStatus(outlineNodeId: string, status: OutlineNodeStatus) {
  emit('changeStatus', outlineNodeId, status)
}
</script>
