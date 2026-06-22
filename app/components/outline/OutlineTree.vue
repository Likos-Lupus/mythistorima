<template>
  <div class="outline-tree">
    <div class="outline-tree-toolbar">
      <label>
        状态筛选
        <select :value="statusFilter" class="outline-select" @change="onFilterChange">
          <option value="all">全部</option>
          <option value="planned">计划</option>
          <option value="drafting">推进中</option>
          <option value="done">完成</option>
          <option value="archived">归档</option>
        </select>
      </label>
    </div>

    <div class="outline-tree-list">
      <OutlineTreeItem
          v-for="item in flatItems"
          :key="item.id"
          :active-outline-node-id="activeOutlineNodeId"
          :can-move-down="canMoveDown(item)"
          :can-move-up="canMoveUp(item)"
          :item="item"
          @delete="$emit('delete', $event)"
          @move="forwardMove"
          @rename="$emit('rename', $event)"
          @select="$emit('select', $event)"
          @create-child="$emit('createChild', $event)"
          @create-sibling="$emit('createSibling', $event)"
          @update-status="forwardStatus"
          @open-document="$emit('openDocument', $event)"
      />

      <div v-if="!flatItems.length" class="empty-panel">
        暂无大纲节点。请创建剧情、冲突或转折节点开始规划。
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import OutlineTreeItem from '~/components/outline/OutlineTreeItem.vue'
import type {OutlineNodeStatus, OutlineTreeNode} from '~/types/outline'

const props = defineProps<{
  items: OutlineTreeNode[]
  activeOutlineNodeId?: string | null
  statusFilter: OutlineNodeStatus | 'all'
}>()

const emit = defineEmits<{
  select: [outlineNodeId: string]
  createChild: [node: OutlineTreeNode]
  createSibling: [node: OutlineTreeNode]
  rename: [node: OutlineTreeNode]
  delete: [node: OutlineTreeNode]
  move: [node: OutlineTreeNode, direction: -1 | 1]
  updateStatus: [outlineNodeId: string, status: OutlineNodeStatus]
  updateStatusFilter: [status: OutlineNodeStatus | 'all']
  openDocument: [documentId: string]
}>()

const flatItems = computed(() => {
  const result: OutlineTreeNode[] = []
  const walk = (items: OutlineTreeNode[]) => {
    for (const item of items) {
      result.push(item)
      walk(item.children)
    }
  }
  walk(props.items)
  return result
})

function normalizeParentId(parentId?: string | null) {
  return parentId && parentId.length > 0 ? parentId : null
}

function siblingsFor(item: OutlineTreeNode) {
  const parentId = normalizeParentId(item.parentId)
  return flatItems.value
      .filter(candidate => normalizeParentId(candidate.parentId) === parentId)
      .sort((a, b) => {
        if (a.sortOrder !== b.sortOrder) return a.sortOrder - b.sortOrder
        return a.createdAt - b.createdAt
      })
}

function canMoveUp(item: OutlineTreeNode) {
  return siblingsFor(item).findIndex(candidate => candidate.id === item.id) > 0
}

function canMoveDown(item: OutlineTreeNode) {
  const siblings = siblingsFor(item)
  const index = siblings.findIndex(candidate => candidate.id === item.id)
  return index >= 0 && index < siblings.length - 1
}

function forwardMove(node: OutlineTreeNode, direction: -1 | 1) {
  emit('move', node, direction)
}

function forwardStatus(outlineNodeId: string, status: OutlineNodeStatus) {
  emit('updateStatus', outlineNodeId, status)
}

function onFilterChange(event: Event) {
  const target = event.target as HTMLSelectElement | null
  if (!target) return
  emit('updateStatusFilter', target.value as OutlineNodeStatus | 'all')
}
</script>
