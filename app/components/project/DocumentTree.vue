<template>
  <section class="document-tree">
    <header class="document-tree-header">
      <div>
        <h2 class="document-tree-title">项目树</h2>
        <p class="document-tree-subtitle">单击预览，双击固定标签</p>
      </div>
      <UDropdownMenu :items="createMenuItems">
        <UButton color="neutral" icon="i-lucide-plus" label="新建" size="sm" variant="soft"/>
      </UDropdownMenu>
    </header>

    <UContextMenu :items="contextMenuItems">
      <UTree
          :get-key="getTreeItemKey"
          :items="treeItems"
          :model-value="selectedTreeItem"
          class="document-tree-utree"
          @update:model-value="onTreeSelect"
      >
        <template #item="{ item }">
          <div
              :class="{'is-active': item.value === activeDocumentId}"
              class="document-tree-node"
              @click="onNodeSelect(item.value)"
              @contextmenu="contextNodeId = item.value"
              @dblclick="onNodePinned(item.value)"
          >
            <span :class="`is-${item.raw.status}`" class="document-tree-status-dot"/>
            <UIcon :name="item.icon" class="document-tree-node-icon"/>
            <span class="document-tree-node-title">{{ item.label }}</span>
            <span class="document-tree-node-count">{{ item.raw.characterCount }}</span>
            <UDropdownMenu :items="nodeMenuItems(item.raw)">
              <UButton
                  aria-label="文档操作"
                  color="neutral"
                  icon="i-lucide-more-horizontal"
                  size="xs"
                  variant="ghost"
                  @click.stop="contextNodeId = item.value"
              />
            </UDropdownMenu>
          </div>
        </template>
      </UTree>
    </UContextMenu>

    <UEmpty
        v-if="!flatItems.length"
        class="document-tree-empty"
        description="新建卷或章节后即可开始写作。"
        icon="i-lucide-file-plus-2"
        title="暂无文档"
    />

    <UModal
        v-model:open="renameOpen"
        description="修改后会同步更新文档标签和项目树。"
        title="重命名文档"
    >
      <template #body>
        <UForm :state="renameState" class="document-tree-dialog-form" @submit="confirmRename">
          <UFormField label="文档标题" name="title" required>
            <UInput
                v-model="renameState.title"
                autofocus
                class="w-full"
                placeholder="输入新的文档标题"
                size="sm"
                @keydown.enter.prevent="confirmRename"
            />
          </UFormField>
        </UForm>
      </template>

      <template #footer>
        <div class="document-tree-dialog-actions">
          <UButton color="neutral" label="取消" size="sm" variant="ghost" @click="renameOpen = false"/>
          <UButton
              :disabled="renameState.title.trim().length === 0"
              color="primary"
              label="保存"
              size="sm"
              @click="confirmRename"
          />
        </div>
      </template>
    </UModal>

    <UModal
        v-model:open="deleteOpen"
        :description="deleteDescription"
        title="删除文档"
    >
      <template #body>
        <UAlert
            color="warning"
            icon="i-lucide-triangle-alert"
            title="此操作会删除当前文档"
            variant="subtle"
        />
      </template>

      <template #footer>
        <div class="document-tree-dialog-actions">
          <UButton color="neutral" label="取消" size="sm" variant="ghost" @click="deleteOpen = false"/>
          <UButton color="error" icon="i-lucide-trash-2" label="删除" size="sm" @click="confirmDelete"/>
        </div>
      </template>
    </UModal>
  </section>
</template>

<script lang="ts" setup>
import type {
  DocumentCreatePayload,
  DocumentStatus,
  DocumentTreeNode,
  DocumentType,
  MoveDocumentInput
} from '~/types/document'

interface TreeItem {
  label: string
  value: string
  icon: string
  raw: DocumentTreeNode
  children?: TreeItem[]
}

const props = defineProps<{
  items: DocumentTreeNode[]
  activeDocumentId?: string | null
}>()

const emit = defineEmits<{
  select: [documentId: string]
  openPinned: [documentId: string]
  createDocument: [payload: DocumentCreatePayload]
  renameDocument: [documentId: string, title: string]
  deleteDocument: [documentId: string]
  moveDocument: [payload: MoveDocumentInput]
  updateStatus: [documentId: string, status: DocumentStatus]
}>()

const contextNodeId = ref<string | null>(null)
const renameOpen = ref(false)
const renameTarget = ref<DocumentTreeNode | null>(null)
const renameState = reactive({title: ''})
const deleteOpen = ref(false)
const deleteTarget = ref<DocumentTreeNode | null>(null)

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

const treeItems = computed<TreeItem[]>(() => props.items.map(toTreeItem))
const selectedTreeItem = computed(() => findTreeItemById(treeItems.value, props.activeDocumentId ?? null))
const contextNode = computed(() => flatItems.value.find(item => item.id === contextNodeId.value) ?? null)
const deleteDescription = computed(() => {
  const item = deleteTarget.value
  if (!item) return '请选择要删除的文档。'
  const childHint = item.children.length ? `它包含 ${item.children.length} 个直接子节点，删除后子节点也会一起删除。` : '删除后可从备份中恢复。'
  return `确定删除《${item.title}》吗？${childHint}`
})

const createMenuItems = computed(() => [[
  {label: '新建卷', icon: 'i-lucide-book-open', onSelect: () => createRoot('volume')},
  {label: '新建章', icon: 'i-lucide-file-text', onSelect: () => createRoot('chapter')}
]])

const contextMenuItems = computed(() => {
  const node = contextNode.value
  if (!node) return createMenuItems.value
  return nodeMenuItems(node)
})

function toTreeItem(node: DocumentTreeNode): TreeItem {
  return {
    label: node.title,
    value: node.id,
    icon: iconFor(node.type),
    raw: node,
    children: node.children.map(toTreeItem)
  }
}

function getTreeItemKey(item: TreeItem) {
  return item.value
}

function findTreeItemById(items: TreeItem[], id: string | null): TreeItem | undefined {
  if (!id) return undefined
  for (const item of items) {
    if (item.value === id) return item
    const child = findTreeItemById(item.children ?? [], id)
    if (child) return child
  }
  return undefined
}

function iconFor(type: DocumentType) {
  switch (type) {
    case 'volume':
      return 'i-lucide-book-open'
    case 'scene':
      return 'i-lucide-clapperboard'
    case 'chapter':
      return 'i-lucide-file-text'
    default:
      return 'i-lucide-file'
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

function nodeMenuItems(item: DocumentTreeNode) {
  return [
    [
      {label: '打开预览', icon: 'i-lucide-file-search', onSelect: () => emit('select', item.id)},
      {label: '在新标签固定', icon: 'i-lucide-pin', onSelect: () => emit('openPinned', item.id)}
    ],
    [
      ...(canCreateChild(item) ? [{
        label: '新建子级',
        icon: 'i-lucide-file-plus-2',
        onSelect: () => createChild(item)
      }] : []),
      {label: '新建同级', icon: 'i-lucide-copy-plus', onSelect: () => createSibling(item)},
      {label: '重命名', icon: 'i-lucide-pencil', onSelect: () => renameItem(item)}
    ],
    [
      {label: `状态：${statusLabel(item.status)}`, icon: 'i-lucide-circle-dot'},
      {label: '草稿', icon: 'i-lucide-circle', onSelect: () => emit('updateStatus', item.id, 'draft')},
      {label: '写作中', icon: 'i-lucide-circle-dot', onSelect: () => emit('updateStatus', item.id, 'writing')},
      {label: '完成', icon: 'i-lucide-check-circle-2', onSelect: () => emit('updateStatus', item.id, 'done')}
    ],
    [
      {label: '上移', icon: 'i-lucide-arrow-up', disabled: !canMoveUp(item), onSelect: () => moveItem(item, -1)},
      {label: '下移', icon: 'i-lucide-arrow-down', disabled: !canMoveDown(item), onSelect: () => moveItem(item, 1)}
    ],
    [
      {label: '删除', icon: 'i-lucide-trash-2', color: 'error', onSelect: () => deleteItem(item)}
    ]
  ]
}

function onTreeSelect(value: unknown) {
  const selected = Array.isArray(value) ? value[0] : value
  if (selected && typeof selected === 'object' && 'value' in selected) {
    const documentId = (selected as { value?: unknown }).value
    if (typeof documentId === 'string') onNodeSelect(documentId)
    return
  }
  if (typeof selected === 'string') onNodeSelect(selected)
}

function onNodeSelect(documentId: string) {
  contextNodeId.value = documentId
  emit('select', documentId)
}

function onNodePinned(documentId: string) {
  contextNodeId.value = documentId
  emit('openPinned', documentId)
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
  renameTarget.value = item
  renameState.title = item.title
  renameOpen.value = true
}

function confirmRename() {
  const item = renameTarget.value
  const title = renameState.title.trim()
  if (!item || !title || title === item.title) {
    renameOpen.value = false
    return
  }
  emit('renameDocument', item.id, title)
  renameOpen.value = false
  renameTarget.value = null
}

function deleteItem(item: DocumentTreeNode) {
  deleteTarget.value = item
  deleteOpen.value = true
}

function confirmDelete() {
  const item = deleteTarget.value
  if (!item) return
  emit('deleteDocument', item.id)
  deleteOpen.value = false
  deleteTarget.value = null
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
