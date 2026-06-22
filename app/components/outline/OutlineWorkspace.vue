<template>
  <section class="outline-workspace">
    <header class="card-workspace-header glass-panel">
      <div>
        <p class="card-workspace-kicker">Phase 2 · Week 2</p>
        <h1>大纲</h1>
        <p>规划剧情节点、冲突、转折、支线和主题，并把大纲绑定到章节或场景。</p>
      </div>
      <div class="outline-header-actions">
        <button class="secondary-button" type="button" @click="reload">刷新</button>
      </div>
    </header>

    <div class="outline-workspace-grid">
      <aside class="outline-tree-panel glass-panel">
        <OutlineCreateMenu @create-root="createRoot"/>
        <OutlineTree
            :active-outline-node-id="outlineStore.activeOutlineNodeId"
            :items="outlineTree"
            :status-filter="outlineStore.statusFilter"
            @delete="deleteNode"
            @move="moveNode"
            @rename="renameNode"
            @select="outlineStore.selectOutlineNode"
            @create-child="createChild"
            @create-sibling="createSibling"
            @update-status="updateNodeStatus"
            @update-status-filter="outlineStore.statusFilter = $event"
            @open-document="openDocument"
        />
      </aside>

      <OutlineEditor
          :documents="documents"
          :node="outlineStore.activeNode"
          :saving="outlineStore.saving"
          @save="saveNode"
          @link-document="linkDocument"
          @unlink-document="unlinkDocument"
          @open-document="openDocument"
      />
    </div>

    <p v-if="errorMessage" class="editor-error">{{ errorMessage }}</p>
  </section>
</template>

<script lang="ts" setup>
import OutlineCreateMenu from '~/components/outline/OutlineCreateMenu.vue'
import OutlineEditor from '~/components/outline/OutlineEditor.vue'
import OutlineTree from '~/components/outline/OutlineTree.vue'
import type {NovelDocument} from '~/types/document'
import type {OutlineNodeStatus, OutlineNodeType, OutlineTreeNode, UpdateOutlineNodeInput} from '~/types/outline'
import {toAppErrorMessage} from '~/utils/appError'

const props = defineProps<{
  projectId: string
  documents: NovelDocument[]
}>()

const emit = defineEmits<{
  openDocument: [documentId: string]
}>()

const outlineStore = useOutlineStore()
const errorMessage = ref<string | null>(null)

const outlineTree = computed(() => outlineStore.treeWithDocuments(props.documents))
const flatOutline = computed(() => outlineStore.flatTreeWithDocuments(props.documents))

watch(() => props.projectId, () => {
  void reload()
}, {immediate: true})

async function run(action: () => Promise<unknown>, fallback: string) {
  errorMessage.value = null
  try {
    await action()
  } catch (error) {
    errorMessage.value = toAppErrorMessage(error, fallback)
  }
}

async function reload() {
  await run(() => outlineStore.loadOutlineNodes(props.projectId), '加载大纲失败')
}

function defaultTitle(type: OutlineNodeType, parentId?: string | null) {
  const siblings = outlineStore.nodes.filter(node => normalizeParentId(node.parentId) === normalizeParentId(parentId) && node.type === type)
  const index = siblings.length + 1
  switch (type) {
    case 'conflict':
      return `冲突 ${index}`
    case 'twist':
      return `转折 ${index}`
    case 'event':
      return `事件 ${index}`
    case 'arc':
      return `支线 ${index}`
    case 'theme':
      return `主题 ${index}`
    case 'note':
      return `备注 ${index}`
    default:
      return `剧情 ${index}`
  }
}

function normalizeParentId(parentId?: string | null) {
  return parentId && parentId.length > 0 ? parentId : null
}

async function createRoot(type: OutlineNodeType) {
  await run(() => outlineStore.createOutlineNode({
    projectId: props.projectId,
    parentId: null,
    type,
    title: defaultTitle(type, null)
  }), '创建大纲节点失败')
}

async function createChild(node: OutlineTreeNode) {
  await run(() => outlineStore.createOutlineNode({
    projectId: props.projectId,
    parentId: node.id,
    type: 'plot',
    title: defaultTitle('plot', node.id)
  }), '创建子级大纲失败')
}

async function createSibling(node: OutlineTreeNode) {
  const siblings = siblingsFor(node)
  const index = siblings.findIndex(item => item.id === node.id)
  await run(() => outlineStore.createOutlineNode({
    projectId: props.projectId,
    parentId: normalizeParentId(node.parentId),
    type: node.type,
    title: defaultTitle(node.type, node.parentId),
    sortOrder: index >= 0 ? index + 1 : undefined
  }), '创建同级大纲失败')
}

async function renameNode(node: OutlineTreeNode) {
  const title = window.prompt('请输入新的大纲标题', node.title)
  if (!title || title.trim() === node.title) return
  await saveNode({outlineNodeId: node.id, title: title.trim()})
}

async function deleteNode(node: OutlineTreeNode) {
  const childHint = node.children.length ? `\n\n它包含 ${node.children.length} 个直接子节点，删除后子节点也会一起删除。` : ''
  if (!window.confirm(`确定删除大纲节点《${node.title}》吗？${childHint}`)) return
  await run(() => outlineStore.deleteOutlineNode(node.id), '删除大纲节点失败')
}

async function moveNode(node: OutlineTreeNode, direction: -1 | 1) {
  const siblings = siblingsFor(node)
  const index = siblings.findIndex(item => item.id === node.id)
  if (index < 0) return
  const targetIndex = Math.max(0, Math.min(siblings.length - 1, index + direction))
  if (targetIndex === index) return
  await run(() => outlineStore.moveOutlineNode({
    outlineNodeId: node.id,
    parentId: normalizeParentId(node.parentId),
    sortOrder: targetIndex
  }), '移动大纲节点失败')
}

async function updateNodeStatus(outlineNodeId: string, status: OutlineNodeStatus) {
  await saveNode({outlineNodeId, status})
}

async function saveNode(input: UpdateOutlineNodeInput) {
  await run(() => outlineStore.updateOutlineNode(input), '保存大纲节点失败')
}

async function linkDocument(outlineNodeId: string, documentId: string) {
  await run(() => outlineStore.linkOutlineNodeToDocument({outlineNodeId, documentId}), '绑定章节失败')
}

async function unlinkDocument(outlineNodeId: string) {
  await run(() => outlineStore.unlinkOutlineNodeDocument(outlineNodeId), '解除章节绑定失败')
}

function openDocument(documentId: string) {
  emit('openDocument', documentId)
}

function siblingsFor(node: OutlineTreeNode) {
  const parentId = normalizeParentId(node.parentId)
  return flatOutline.value
      .filter(candidate => normalizeParentId(candidate.parentId) === parentId)
      .sort((a, b) => {
        if (a.sortOrder !== b.sortOrder) return a.sortOrder - b.sortOrder
        return a.createdAt - b.createdAt
      })
}
</script>
