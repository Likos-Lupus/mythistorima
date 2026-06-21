<template>
  <AppShell
      :status="topbarStatus"
      :subtitle="activeDocument?.title || '本地写作闭环'"
      :title="projectStore.currentProject?.title || 'Mythistorima'"
      @home="router.push('/')"
  >
    <div class="workspace-layout">
      <aside class="workspace-sidebar app-sidebar glass-panel" data-phase0-area="chapter-sidebar">
        <ChapterTree
            :active-document-id="documentStore.activeDocumentId"
            :documents="documentStore.documents"
            @create="createChapter"
            @select="selectDocument"
        />
      </aside>

      <main class="workspace-editor workspace-editor-host" data-phase0-area="novel-editor-host">
        <NovelEditor
            v-if="documentStore.activeDocumentId"
            :key="documentStore.activeDocumentId"
            :document-id="documentStore.activeDocumentId"
            @saved="handleSaved"
            @status="handleEditorStatus"
        />

        <section v-else class="editor-select-empty paper-card">
          请选择或创建一个章节。
        </section>
      </main>

      <aside class="workspace-status status-panel glass-panel" data-phase0-area="status-panel">
        <div>
          <h2 class="status-panel-title">Phase 0 状态</h2>
          <p class="status-panel-subtitle">用于验证本地写作闭环。</p>
        </div>

        <dl class="status-list">
          <div class="status-card">
            <dt>项目 ID</dt>
            <dd>{{ projectId }}</dd>
          </div>
          <div class="status-card">
            <dt>章节 ID</dt>
            <dd>{{ documentStore.activeDocumentId || '未选择' }}</dd>
          </div>
          <div class="status-card">
            <dt>当前章节字数</dt>
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
import ChapterTree from '~/components/project/ChapterTree.vue'
import NovelEditor from '~/components/editor/NovelEditor.vue'
import type {ProjectStats} from '~/types/stats'
import type {SaveState} from '~/composables/useAutoSave'

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
    pageError.value = typeof error === 'object' && error && 'message' in error
        ? String((error as { message?: string }).message)
        : '加载项目失败'
  }
}

async function refreshStats() {
  projectStats.value = await projectStore.loadProjectStats(projectId.value)
}

async function createChapter() {
  const nextNumber = documentStore.documents.length + 1
  try {
    await documentStore.createDocument({
      projectId: projectId.value,
      type: 'chapter',
      title: `第 ${nextNumber} 章`,
      sortOrder: nextNumber - 1
    })
    await refreshStats()
  } catch (error) {
    pageError.value = typeof error === 'object' && error && 'message' in error
        ? String((error as { message?: string }).message)
        : '创建章节失败'
  }
}

function selectDocument(documentId: string) {
  documentStore.selectDocument(documentId)
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
