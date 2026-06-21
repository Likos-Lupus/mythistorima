<template>
  <AppShell
      :status="topbarStatus"
      :subtitle="activeDocument?.title || '小说工作台 MVP'"
      :title="projectStore.currentProject?.title || 'Mythistorima'"
      @home="router.push('/')"
  >
    <div :class="{ 'is-focus-mode': focusMode }" class="workspace-layout">
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
        <EditorFocusOverlay v-if="focusMode" @exit="toggleFocusMode"/>

        <NovelEditor
            v-if="documentStore.activeDocumentId"
            :key="documentStore.activeDocumentId"
            :document-id="documentStore.activeDocumentId"
            :focus-mode="focusMode"
            :project-id="projectId"
            :settings="settingsStore.editorSettings"
            :target-character-count="activeDocumentTarget"
            @saved="handleSaved"
            @session="handleEditorSession"
            @status="handleEditorStatus"
            @toggle-focus-mode="toggleFocusMode"
        />

        <section v-else class="editor-select-empty paper-card">
          请选择或创建一个卷、章节或场景。
        </section>
      </main>

      <aside class="workspace-status status-panel glass-panel" data-phase1-area="status-panel">
        <div>
          <h2 class="status-panel-title">Phase 1 Week 3</h2>
          <p class="status-panel-subtitle">编辑器增强：段落 ID、查找替换、专注模式、写作计时与编辑器设置。</p>
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
            <dt>当前文档目标</dt>
            <dd>
              <div class="compact-setting-row">
                <input
                    v-model.number="targetDraft"
                    class="compact-number-field"
                    min="0"
                    placeholder="未设置"
                    type="number"
                    @change="saveDocumentTarget"
                >
                <span>字</span>
              </div>
            </dd>
          </div>
          <div class="status-card">
            <dt>项目总字数</dt>
            <dd>{{ projectStats?.characterCount ?? 0 }} 字</dd>
          </div>
          <div class="status-card">
            <dt>今日写作</dt>
            <dd>{{ timerStore.todayCharacterCount }} 字 · {{ formatDuration(timerStore.todayElapsedMs) }}</dd>
          </div>
          <div class="status-card">
            <dt>本次写作</dt>
            <dd>+{{ editorSnapshot.sessionDelta }} 字 · {{ formatDuration(editorSnapshot.sessionElapsedMs) }}</dd>
          </div>
          <div class="status-card">
            <dt>保存状态</dt>
            <dd>{{ saveStateLabel }}</dd>
          </div>
          <div class="status-card">
            <dt>最近保存</dt>
            <dd>{{ editorSnapshot.lastSavedAt ? formatDate(editorSnapshot.lastSavedAt) : '尚未保存' }}</dd>
          </div>
        </dl>

        <section class="settings-card">
          <h3>编辑器设置</h3>
          <label>
            字号
            <input
                :value="settingsStore.editorSettings.fontSize"
                class="compact-number-field"
                max="28"
                min="12"
                type="number"
                @change="updateEditorSetting('fontSize', $event)"
            >
          </label>
          <label>
            行距
            <input
                :value="settingsStore.editorSettings.lineHeight"
                class="compact-number-field"
                max="2.8"
                min="1.3"
                step="0.05"
                type="number"
                @change="updateEditorSetting('lineHeight', $event)"
            >
          </label>
          <label>
            页面宽度
            <input
                :value="settingsStore.editorSettings.pageWidth"
                class="compact-number-field"
                max="1100"
                min="560"
                step="10"
                type="number"
                @change="updateEditorSetting('pageWidth', $event)"
            >
          </label>
          <button class="secondary-button full-width-button" type="button" @click="toggleFocusMode">
            {{ focusMode ? '退出专注模式' : '进入专注模式' }}
          </button>
        </section>

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
import EditorFocusOverlay from '~/components/editor/EditorFocusOverlay.vue'
import type {ProjectStats, TodayWritingStats} from '~/types/stats'
import type {SaveState} from '~/composables/useAutoSave'
import type {DocumentCreatePayload, DocumentStatus, DocumentType, MoveDocumentInput} from '~/types/document'
import type {EditorSessionSnapshot, EditorSettings} from '~/types/editor'

const route = useRoute()
const router = useRouter()
const projectStore = useProjectStore()
const documentStore = useDocumentStore()
const settingsStore = useSettingsStore()
const timerStore = useTimerStore()
const {call} = useTauriInvoke()

const projectId = computed(() => String(route.params.id))
const projectStats = ref<ProjectStats | null>(null)
const todayStats = ref<TodayWritingStats | null>(null)
const pageError = ref<string | null>(null)
const focusMode = ref(false)
const targetDraft = ref<number | null>(null)

const editorSnapshot = reactive({
  saveState: 'idle' as SaveState,
  characterCount: 0,
  lastSavedAt: null as number | null,
  errorMessage: null as string | null,
  sessionElapsedMs: 0,
  sessionDelta: 0
})

const activeDocument = computed(() => documentStore.activeDocument)
const activeDocumentTarget = computed(() => getDocumentTarget(activeDocument.value?.metadataJson))

watch(activeDocumentTarget, value => {
  targetDraft.value = value ?? null
}, {immediate: true})

const topbarStatus = computed(() => {
  const title = activeDocument.value?.title ? `${activeDocument.value.title} · ` : ''
  const today = timerStore.todayCharacterCount ? ` · 今日 ${timerStore.todayCharacterCount} 字` : ''
  return `${title}${projectStats.value?.characterCount ?? 0} 字${today}`
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

onBeforeUnmount(() => {
  timerStore.finishSession()
})

async function loadProjectWorkspace() {
  pageError.value = null
  try {
    await settingsStore.loadAppSettings()
    await projectStore.loadProject(projectId.value)
    await documentStore.loadDocuments(projectId.value)
    await refreshStats()
    await refreshTodayStats()
  } catch (error) {
    pageError.value = errorMessage(error, '加载项目失败')
  }
}

async function refreshStats() {
  projectStats.value = await projectStore.loadProjectStats(projectId.value)
}

async function refreshTodayStats() {
  const range = getTodayRange()
  todayStats.value = await call<TodayWritingStats>('get_today_writing_stats', {
    projectId: projectId.value,
    dayStart: range.dayStart,
    dayEnd: range.dayEnd
  })
  timerStore.setTodayStats(todayStats.value)
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

async function saveDocumentTarget() {
  if (!activeDocument.value) return
  pageError.value = null
  try {
    const target = typeof targetDraft.value === 'number' && targetDraft.value > 0
        ? Math.round(targetDraft.value)
        : null
    await documentStore.updateDocumentGoal({
      documentId: activeDocument.value.id,
      targetCharacterCount: target
    })
  } catch (error) {
    pageError.value = errorMessage(error, '更新目标字数失败')
  }
}

async function handleSaved() {
  await refreshStats()
  await refreshTodayStats()
}

function handleEditorSession(payload: EditorSessionSnapshot) {
  editorSnapshot.sessionElapsedMs = payload.elapsedMs
  editorSnapshot.sessionDelta = payload.sessionDelta
}

function handleEditorStatus(payload: {
  saveState: SaveState
  characterCount: number
  lastSavedAt: number | null
  errorMessage: string | null
  sessionElapsedMs: number
  sessionDelta: number
}) {
  editorSnapshot.saveState = payload.saveState
  editorSnapshot.characterCount = payload.characterCount
  editorSnapshot.lastSavedAt = payload.lastSavedAt
  editorSnapshot.errorMessage = payload.errorMessage
  editorSnapshot.sessionElapsedMs = payload.sessionElapsedMs
  editorSnapshot.sessionDelta = payload.sessionDelta
}

function toggleFocusMode() {
  focusMode.value = !focusMode.value
}

async function updateEditorSetting(key: keyof EditorSettings, event: Event) {
  const value = Number((event.target as HTMLInputElement).value)
  if (!Number.isFinite(value)) return
  try {
    await settingsStore.updateEditorSetting(key, value)
  } catch (error) {
    pageError.value = errorMessage(error, '保存编辑器设置失败')
  }
}

function getDocumentTarget(metadataJson?: string | null) {
  if (!metadataJson) return null
  try {
    const parsed = JSON.parse(metadataJson) as { targetCharacterCount?: unknown }
    return typeof parsed.targetCharacterCount === 'number' && parsed.targetCharacterCount > 0
        ? parsed.targetCharacterCount
        : null
  } catch {
    return null
  }
}

function getTodayRange() {
  const now = new Date()
  const start = new Date(now.getFullYear(), now.getMonth(), now.getDate())
  const end = new Date(start)
  end.setDate(start.getDate() + 1)
  return {
    dayStart: start.getTime(),
    dayEnd: end.getTime()
  }
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

function formatDuration(ms: number) {
  const totalSeconds = Math.floor(ms / 1000)
  const hours = Math.floor(totalSeconds / 3600)
  const minutes = Math.floor((totalSeconds % 3600) / 60)
  const seconds = totalSeconds % 60
  if (hours > 0) return `${hours}:${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`
  return `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`
}
</script>
