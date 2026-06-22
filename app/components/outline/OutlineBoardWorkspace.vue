<template>
  <section class="outline-workspace outline-board-workspace">
    <header class="card-workspace-header glass-panel">
      <div>
        <p class="card-workspace-kicker">Phase 2 · Week 3</p>
        <h1>大纲看板</h1>
        <p>用 planned / drafting / done 看板推进剧情节点，并一键生成 Mermaid 剧情流程图。</p>
      </div>
      <div class="outline-header-actions">
        <button class="secondary-button" type="button" @click="reload">刷新</button>
      </div>
    </header>

    <div class="outline-board-toolbar glass-panel">
      <OutlineStatusFilter
          v-model="visibleStatus"
          :include-archived="false"
          :nodes="outlineStore.nodes"
      />
      <p>
        {{ boardSummary }}
      </p>
    </div>

    <div class="outline-board-layout">
      <section class="outline-board-panel glass-panel">
        <OutlineBoard
            :active-outline-node-id="outlineStore.activeOutlineNodeId"
            :items="boardItems"
            @select="outlineStore.selectOutlineNode"
            @change-status="changeStatus"
            @open-document="openDocument"
        />
      </section>

      <aside class="outline-board-detail glass-panel">
        <div v-if="activeNode" class="outline-board-selected-card">
          <p class="card-editor-kicker">Selected Node</p>
          <h2>{{ activeNode.title }}</h2>
          <span class="outline-status-pill">{{ outlineStatusLabel(activeNode.status) }}</span>
          <p v-if="activeNode.summary">{{ activeNode.summary }}</p>
          <p v-else class="paper-muted-text">这个节点还没有摘要。可以回到“大纲”工作区补充剧情目标和冲突说明。</p>
          <div class="outline-editor-actions">
            <button
                v-for="status in boardStatuses"
                :key="status"
                :disabled="status === activeNode.status"
                class="secondary-button"
                type="button"
                @click="changeStatus(activeNode.id, status)"
            >
              标记为{{ outlineStatusLabel(status) }}
            </button>
            <button
                v-if="activeNode.linkedDocumentId"
                class="primary-button"
                type="button"
                @click="openDocument(activeNode.linkedDocumentId)"
            >
              打开绑定章节
            </button>
          </div>
        </div>
        <div v-else class="outline-editor-empty empty-panel">
          请选择一个看板卡片查看摘要、绑定章节和状态操作。
        </div>
      </aside>
    </div>

    <OutlineMermaidView
        :documents="documents"
        :items="mermaidTree"
    />

    <p v-if="errorMessage" class="editor-error">{{ errorMessage }}</p>
  </section>
</template>

<script lang="ts" setup>
import OutlineBoard from '~/components/outline/OutlineBoard.vue'
import OutlineMermaidView from '~/components/outline/OutlineMermaidView.vue'
import OutlineStatusFilter from '~/components/outline/OutlineStatusFilter.vue'
import type {NovelDocument} from '~/types/document'
import type {OutlineNodeStatus} from '~/types/outline'
import {toAppErrorMessage} from '~/utils/appError'
import {flattenOutlineTree, OUTLINE_BOARD_STATUSES, outlineStatusLabel} from '~/utils/outlinePresentation'

const props = defineProps<{
  projectId: string
  documents: NovelDocument[]
}>()

const emit = defineEmits<{
  openDocument: [documentId: string]
}>()

const outlineStore = useOutlineStore()
const errorMessage = ref<string | null>(null)
const visibleStatus = ref<OutlineNodeStatus | 'all'>('all')
const boardStatuses = OUTLINE_BOARD_STATUSES

const activeNode = computed(() => outlineStore.activeNode)
const boardTree = computed(() => outlineStore.treeWithDocuments(props.documents))
const boardFlatItems = computed(() => flattenOutlineTree(boardTree.value))
const boardItems = computed(() => {
  const visible = boardFlatItems.value.filter(item => item.status !== 'archived')
  if (visibleStatus.value === 'all') return visible
  return visible.filter(item => item.status === visibleStatus.value)
})
const mermaidTree = computed(() => outlineStore.treeWithDocuments(props.documents))
const boardSummary = computed(() => {
  const planned = outlineStore.nodes.filter(node => node.status === 'planned').length
  const drafting = outlineStore.nodes.filter(node => node.status === 'drafting').length
  const done = outlineStore.nodes.filter(node => node.status === 'done').length
  return `计划 ${planned} · 推进中 ${drafting} · 完成 ${done}`
})

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
  await run(() => outlineStore.loadOutlineNodes(props.projectId), '加载大纲看板失败')
}

async function changeStatus(outlineNodeId: string, status: OutlineNodeStatus) {
  const node = outlineStore.nodes.find(item => item.id === outlineNodeId)
  if (!node || node.status === status) return
  await run(() => outlineStore.updateOutlineNode({outlineNodeId, status}), '更新大纲状态失败')
}

function openDocument(documentId: string) {
  emit('openDocument', documentId)
}
</script>
