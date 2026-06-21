<template>
  <AppShell
      :status="topbarStatus"
      :subtitle="activeDocument?.title || '小说工作台 MVP'"
      :title="projectStore.currentProject?.title || 'Mythistorima'"
      @home="router.push('/')"
  >
    <div class="workspace-layout">
      <aside class="workspace-sidebar app-sidebar glass-panel" data-phase1-area="document-tree-sidebar">
        <DocumentTree
            :active-document-id="documentStore.activeDocumentId"
            :items="documentStore.documentTree"
            @select="selectDocument"
            @create-document="createDocument"
            @delete-document="deleteDocument"
            @move-document="moveDocument"
            @rename-document="renameDocument"
            @update-status="updateDocumentStatus"
        />
      </aside>

      <main class="workspace-editor workspace-editor-host" data-phase1-area="novel-editor-host">
        <NovelEditor
            v-if="documentStore.activeDocumentId"
            :key="documentStore.activeDocumentId"
            :document-id="documentStore.activeDocumentId"
            @saved="handleSaved"
            @status="handleEditorStatus"
        />

        <section v-else class="editor-select-empty paper-card">
          请选择或创建一个卷、章节或场景。
        </section>
      </main>

      <aside class="workspace-status status-panel glass-panel" data-phase1-area="status-panel">
        <div>
          <h2 class="status-panel-title">Phase 1 Week 2</h2>
          <p class="status-panel-subtitle">文档树增强：卷 / 章 / 场景、排序、状态。</p>
        </div>

        <dl class="status-list">
          <div class="status-card">
            <dt>项目 ID</dt>
            <dd>{{ projectId }}</dd>
          </div>
          <div class="status-card">
            <dt>当前文档</dt>
            <dd>{{
                activeDocument ? `${documentTypeLabel(activeDocument.type)} · ${activeDocument.title}` : '未选择'
              }}
            </dd>
          </div>
          <div class="status-card">
            <dt>文档状态</dt>
            <dd>{{ activeDocument ? documentStatusLabel(activeDocument.status) : '未选择' }}</dd>
          </div>
          <div class="status-card">
            <dt>当前文档字数</dt>
            <dd>{{ editorSnapshot.characterCount }} 字</dd>
          </div>
          <div class="status-card">
            <dt>项目总字数</dt>
            <dd>{{ projectStats?.characterCount ?? 0 }} 字</dd>
          </div>
          <div class="status-card">
            <dt>保存状态</dt>
            <dd>{{ saveStateLabel }}</dd>
          </div>
          <div class="status-card">
            <dt>最近保存</dt>
            <dd>
              {{ editorSnapshot.lastSavedAt ? formatDate(editorSnapshot.lastSavedAt) : '尚未保存' }}
            </dd>
          </div>
        </dl>

        <p v-if="pageError" class="editor-error">{{ pageError }}</p>
        <p v-if="editorSnapshot.errorMessage" class="editor-error">{{ editorSnapshot.errorMessage }}</p>
      </aside>
    </div>
  </AppShell>
</template>

<script lang="ts" setup>
import AppShell from '~/components/layout/AppShell.vue'
import DocumentTree from '~/components/project/DocumentTree.vue'
import NovelEditor from '~/components/editor/NovelEditor.vue'
import type {ProjectStats} from '~/types/stats'
import type {SaveState} from '~/composables/useAutoSave'
import type {DocumentCreatePayload, DocumentStatus, DocumentType, MoveDocumentInput} from '~/types/document'

const route = useRoute()
const router = useRouter()
const projectStore = useProjectStore()
const documentStore = useDocumentStore()

const projectId = computed(() => String(route.params.id))
const projectStats = ref<ProjectStats | null>(null)
const pageError = ref<string | null>(null)

const editorSnapshot = reactive({
  saveState: 'idle' as SaveState,
  characterCount: 0,
  lastSavedAt: null as number | null,
  errorMessage: null as string | null
})

const activeDocument = computed(() => documentStore.activeDocument)

const topbarStatus = computed(() => {
  const title = activeDocument.value?.title ? `${activeDocument.value.title} · ` : ''
  return `${title}${projectStats.value?.characterCount ?? 0} 字`
})

const saveStateLabel = computed(() => {
  switch (editorSnapshot.saveState) {
    case 'dirty':
      return '有未保存修改'
    case 'saving':
      return '保存中'
    case 'saved':
      return '已保存'
    case 'failed':
      return '保存失败'
    default:
      return '等待输入'
  }
})

onMounted(async () => {
  await loadProjectWorkspace()
})

async function loadProjectWorkspace() {
  pageError.value = null
  try {
    await projectStore.loadProject(projectId.value)
    await documentStore.loadDocuments(projectId.value)
    await refreshStats()
  } catch (error) {
    pageError.value = errorMessage(error, '加载项目失败')
  }
}

async function refreshStats() {
  projectStats.value = await projectStore.loadProjectStats(projectId.value)
}

async function createDocument(payload: DocumentCreatePayload) {
  pageError.value = null
  try {
    await documentStore.createDocument({
      projectId: projectId.value,
      parentId: payload.parentId ?? null,
      type: payload.type,
      title: payload.title,
      sortOrder: payload.sortOrder ?? null
    })
    await refreshStats()
  } catch (error) {
    pageError.value = errorMessage(error, '创建文档失败')
  }
}

function selectDocument(documentId: string) {
  documentStore.selectDocument(documentId)
}

async function renameDocument(documentId: string, title: string) {
  pageError.value = null
  try {
    await documentStore.renameDocument(documentId, title)
    await refreshStats()
  } catch (error) {
    pageError.value = errorMessage(error, '重命名文档失败')
  }
}

async function deleteDocument(documentId: string) {
  pageError.value = null
  try {
    await documentStore.deleteDocument(documentId)
    await refreshStats()
  } catch (error) {
    pageError.value = errorMessage(error, '删除文档失败')
  }
}

async function moveDocument(input: MoveDocumentInput) {
  pageError.value = null
  try {
    await documentStore.moveDocument(input)
    await refreshStats()
  } catch (error) {
    pageError.value = errorMessage(error, '移动文档失败')
  }
}

async function updateDocumentStatus(documentId: string, status: DocumentStatus) {
  pageError.value = null
  try {
    await documentStore.updateDocumentStatus({documentId, status})
    await refreshStats()
  } catch (error) {
    pageError.value = errorMessage(error, '更新文档状态失败')
  }
}

async function handleSaved() {
  await refreshStats()
}

function handleEditorStatus(payload: {
  saveState: SaveState
  characterCount: number
  lastSavedAt: number | null
  errorMessage: string | null
}) {
  editorSnapshot.saveState = payload.saveState
  editorSnapshot.characterCount = payload.characterCount
  editorSnapshot.lastSavedAt = payload.lastSavedAt
  editorSnapshot.errorMessage = payload.errorMessage
}

function documentTypeLabel(type: DocumentType) {
  switch (type) {
    case 'volume':
      return '卷'
    case 'scene':
      return '场景'
    case 'chapter':
      return '章节'
    default:
      return '文档'
  }
}

function documentStatusLabel(status: DocumentStatus) {
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

function errorMessage(error: unknown, fallback: string) {
  return typeof error === 'object' && error && 'message' in error
      ? String((error as { message?: string }).message)
      : fallback
}

function formatDate(timestamp: number) {
  return new Intl.DateTimeFormat('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  }).format(new Date(timestamp))
}
</script>
