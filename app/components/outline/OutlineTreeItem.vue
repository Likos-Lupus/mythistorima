<template>
  <article
      :class="['outline-tree-row', { 'is-active': item.id === activeOutlineNodeId }]"
      :style="{ '--outline-depth': item.depth }"
  >
    <button class="outline-tree-main" type="button" @click="$emit('select', item.id)">
      <span class="outline-tree-type">{{ typeLabel(item.type) }}</span>
      <span class="outline-tree-name">{{ item.title }}</span>
      <span class="outline-tree-meta">
        {{ statusLabel(item.status) }}
        <template v-if="item.linkedDocument"> · 绑定 {{
            documentTypeLabel(item.linkedDocument.type)
          }}《{{ item.linkedDocument.title }}》</template>
      </span>
    </button>

    <div class="outline-tree-actions">
      <button class="tree-action-button" type="button" @click="$emit('createChild', item)">+子级</button>
      <button class="tree-action-button" type="button" @click="$emit('createSibling', item)">+同级</button>
      <button class="tree-action-button" type="button" @click="$emit('rename', item)">改名</button>
      <button :disabled="!canMoveUp" class="tree-action-button" type="button" @click="$emit('move', item, -1)">↑
      </button>
      <button :disabled="!canMoveDown" class="tree-action-button" type="button" @click="$emit('move', item, 1)">↓
      </button>
      <select :value="item.status" class="tree-status-select" @change="onStatusChange">
        <option value="planned">计划</option>
        <option value="drafting">推进中</option>
        <option value="done">完成</option>
        <option value="archived">归档</option>
      </select>
      <button v-if="item.linkedDocumentId" class="tree-action-button" type="button"
              @click="$emit('openDocument', item.linkedDocumentId)">跳转
      </button>
      <button class="tree-danger-button" type="button" @click="$emit('delete', item)">删除</button>
    </div>
  </article>
</template>

<script lang="ts" setup>
import type {OutlineNodeStatus, OutlineTreeNode} from '~/types/outline'

const props = defineProps<{
  item: OutlineTreeNode
  activeOutlineNodeId?: string | null
  canMoveUp: boolean
  canMoveDown: boolean
}>()

const emit = defineEmits<{
  select: [outlineNodeId: string]
  createChild: [node: OutlineTreeNode]
  createSibling: [node: OutlineTreeNode]
  rename: [node: OutlineTreeNode]
  delete: [node: OutlineTreeNode]
  move: [node: OutlineTreeNode, direction: -1 | 1]
  updateStatus: [outlineNodeId: string, status: OutlineNodeStatus]
  openDocument: [documentId: string]
}>()

function typeLabel(type: string) {
  switch (type) {
    case 'conflict':
      return '冲突'
    case 'twist':
      return '转折'
    case 'event':
      return '事件'
    case 'arc':
      return '支线'
    case 'theme':
      return '主题'
    case 'note':
      return '备注'
    default:
      return '剧情'
  }
}

function statusLabel(status: string) {
  switch (status) {
    case 'drafting':
      return '推进中'
    case 'done':
      return '完成'
    case 'archived':
      return '归档'
    default:
      return '计划'
  }
}

function documentTypeLabel(type: string) {
  switch (type) {
    case 'chapter':
      return '章'
    case 'scene':
      return '场景'
    case 'volume':
      return '卷'
    default:
      return '文档'
  }
}

function onStatusChange(event: Event) {
  const target = event.target as HTMLSelectElement | null
  if (!target) return
  emit('updateStatus', props.item.id, target.value as OutlineNodeStatus)
}
</script>
