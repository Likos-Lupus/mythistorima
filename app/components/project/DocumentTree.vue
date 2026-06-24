<template>
  <div class="document-tree">
    <div class="document-tree-header">
      <div>
        <h2 class="document-tree-title">文档树</h2>
        <p class="document-tree-subtitle">卷 / 章 / 场景</p>
      </div>
    </div>

    <div class="document-tree-create-row">
      <button class="primary-button document-create-button" type="button" @click="createRoot('volume')">
        新建卷
      </button>
      <button class="secondary-button document-create-button" type="button" @click="createRoot('chapter')">
        新建章
      </button>
    </div>

    <div class="document-tree-list">
      <article
          v-for="item in flatItems"
          :key="item.id"
          :class="{ 'is-active': item.id === activeDocumentId }"
          :style="{ '--tree-depth': item.depth }"
          class="document-tree-row"
      >
        <button class="document-tree-main" type="button" @click="$emit('select', item.id)">
          <span class="document-tree-type">{{ typeLabel(item.type) }}</span>
          <span class="document-tree-name">{{ item.title }}</span>
          <span class="document-tree-meta">
            {{ statusLabel(item.status) }} · {{ item.characterCount }} 字
          </span>
        </button>

        <div class="document-tree-actions">
          <button
              v-if="canCreateChild(item)"
              :title="`给《${item.title}》添加子级`"
              class="tree-action-button"
              type="button"
              @click="createChild(item)"
          >
            +子级
          </button>
          <button class="tree-action-button" title="新建同级" type="button" @click="createSibling(item)">
            +同级
          </button>
          <button class="tree-action-button" title="重命名" type="button" @click="renameItem(item)">
            改名
          </button>
          <button
              :disabled="!canMoveUp(item)"
              class="tree-action-button"
              title="上移"
              type="button"
              @click="moveItem(item, -1)"
          >
            ↑
          </button>
          <button
              :disabled="!canMoveDown(item)"
              class="tree-action-button"
              title="下移"
              type="button"
              @click="moveItem(item, 1)"
          >
            ↓
          </button>
          <select
              :title="`设置《${item.title}》状态`"
              :value="item.status"
              class="tree-status-select"
              @change="onStatusChange(item, $event)"
          >
            <option value="draft">草稿</option>
            <option value="writing">写作中</option>
            <option value="done">完成</option>
          </select>
          <button class="tree-danger-button" title="删除" type="button" @click="deleteItem(item)">
            删除
          </button>
        </div>
      </article>

      <div v-if="!flatItems.length" class="empty-panel">
        暂无文档。请新建卷或章节开始写作。
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import type {
  DocumentCreatePayload,
  DocumentStatus,
  DocumentTreeNode,
  DocumentType,
  MoveDocumentInput
} from '~/types/document'

const props = defineProps<{
  items: DocumentTreeNode[]
  activeDocumentId?: string | null
}>()

const emit = defineEmits<{
  select: [documentId: string]
  createDocument: [payload: DocumentCreatePayload]
  renameDocument: [documentId: string, title: string]
  deleteDocument: [documentId: string]
  moveDocument: [payload: MoveDocumentInput]
  updateStatus: [documentId: string, status: DocumentStatus]
}>()

const flatItems = computed(() => {
  const result: DocumentTreeNode[] = []
  const walk = (nodes: DocumentTreeNode[]) => {
    for (const node of nodes) {
      result.push(node)
      walk(node.children)
    }
  }
  walk(props.items)
  return result
})

function typeLabel(type: DocumentType) {
  switch (type) {
    case 'volume':
      return '卷'
    case 'scene':
      return '场景'
    case 'chapter':
      return '章'
    default:
      return '文档'
  }
}

function statusLabel(status: DocumentStatus) {
  switch (status) {
    case 'writing':
      return '写作中'
    case 'done':
      return '完成'
    case 'draft':
      return '草稿'
    case 'revised':
      return '已修订'
    case 'archived':
      return '已归档'
    default:
      return String(status)
  }
}

function defaultTitle(type: DocumentType, parentId?: string | null) {
  const sameLevelCount = flatItems.value.filter(item => normalizeParentId(item.parentId) === normalizeParentId(parentId) && item.type === type).length
  const index = sameLevelCount + 1
  switch (type) {
    case 'volume':
      return `第 ${index} 卷`
    case 'scene':
      return `场景 ${index}`
    case 'chapter':
      return `第 ${index} 章`
    default:
      return `文档 ${index}`
  }
}

function normalizeParentId(parentId?: string | null) {
  return parentId && parentId.length > 0 ? parentId : null
}

function childTypeFor(item: DocumentTreeNode): DocumentType {
  if (item.type === 'volume') return 'chapter'
  return 'scene'
}

function canCreateChild(item: DocumentTreeNode) {
  return item.type === 'volume' || item.type === 'chapter'
}

function createRoot(type: DocumentType) {
  emit('createDocument', {
    parentId: null,
    type,
    title: defaultTitle(type, null)
  })
}

function createChild(item: DocumentTreeNode) {
  const type = childTypeFor(item)
  emit('createDocument', {
    parentId: item.id,
    type,
    title: defaultTitle(type, item.id)
  })
}

function createSibling(item: DocumentTreeNode) {
  const siblings = siblingsFor(item)
  const index = siblings.findIndex(sibling => sibling.id === item.id)
  emit('createDocument', {
    parentId: normalizeParentId(item.parentId),
    type: item.type,
    title: defaultTitle(item.type, item.parentId),
    sortOrder: index >= 0 ? index + 1 : undefined
  })
}

function renameItem(item: DocumentTreeNode) {
  const title = window.prompt('请输入新的文档标题', item.title)
  if (!title || title.trim() === item.title) return
  emit('renameDocument', item.id, title.trim())
}

function deleteItem(item: DocumentTreeNode) {
  const childHint = item.children.length ? `\n\n它包含 ${item.children.length} 个直接子节点，删除后子节点也会一起删除。` : ''
  if (!window.confirm(`确定删除《${item.title}》吗？${childHint}`)) return
  emit('deleteDocument', item.id)
}

function onStatusChange(item: DocumentTreeNode, event: Event) {
  const target = event.target as HTMLSelectElement | null
  if (!target) return
  emit('updateStatus', item.id, target.value as DocumentStatus)
}

function siblingsFor(item: DocumentTreeNode) {
  const parentId = normalizeParentId(item.parentId)
  return flatItems.value
      .filter(candidate => normalizeParentId(candidate.parentId) === parentId)
      .sort((a, b) => {
        if (a.sortOrder !== b.sortOrder) return a.sortOrder - b.sortOrder
        return a.createdAt - b.createdAt
      })
}

function canMoveUp(item: DocumentTreeNode) {
  return siblingsFor(item).findIndex(candidate => candidate.id === item.id) > 0
}

function canMoveDown(item: DocumentTreeNode) {
  const siblings = siblingsFor(item)
  const index = siblings.findIndex(candidate => candidate.id === item.id)
  return index >= 0 && index < siblings.length - 1
}

function moveItem(item: DocumentTreeNode, direction: -1 | 1) {
  const siblings = siblingsFor(item)
  const index = siblings.findIndex(candidate => candidate.id === item.id)
  if (index < 0) return
  const targetIndex = Math.max(0, Math.min(siblings.length - 1, index + direction))
  if (targetIndex === index) return

  emit('moveDocument', {
    documentId: item.id,
    parentId: normalizeParentId(item.parentId),
    sortOrder: targetIndex
  })
}
</script>
