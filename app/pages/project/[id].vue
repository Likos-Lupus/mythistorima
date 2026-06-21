<template>
  <AppShell
      :status="topbarStatus"
      :subtitle="activeDocument?.title || '小说工作台 MVP'"
      :title="projectStore.currentProject?.title || 'Mythistorima'"
      @home="router.push('/')"
  >
    <div :class="{ 'is-focus-mode': focusMode }" class="workspace-layout">
      <aside class="workspace-sidebar app-sidebar glass-panel" data-phase1-area="workspace-sidebar">
        <nav aria-label="项目工作区" class="workspace-mode-switch">
          <button
              :class="['workspace-mode-button', { 'is-active': workspaceMode === 'writing' }]"
              type="button"
              @click="workspaceMode = 'writing'"
          >
            写作
          </button>
          <button
              :class="['workspace-mode-button', { 'is-active': workspaceMode === 'cards' }]"
              type="button"
              @click="workspaceMode = 'cards'"
          >
            设定
          </button>
          <button
              :class="['workspace-mode-button', { 'is-active': workspaceMode === 'notes' }]"
              type="button"
              @click="workspaceMode = 'notes'"
          >
            事项
          </button>
          <button
              :class="['workspace-mode-button', { 'is-active': workspaceMode === 'search' }]"
              type="button"
              @click="workspaceMode = 'search'"
          >
            搜索
          </button>
          <button
              :class="['workspace-mode-button', { 'is-active': workspaceMode === 'export' }]"
              type="button"
              @click="workspaceMode = 'export'"
          >
            导出
          </button>
          <button
              :class="['workspace-mode-button', { 'is-active': workspaceMode === 'settings' }]"
              type="button"
              @click="workspaceMode = 'settings'"
          >
            设置
          </button>
        </nav>

        <DocumentTree
            v-if="workspaceMode === 'writing'"
            :active-document-id="documentStore.activeDocumentId"
            :items="documentStore.documentTree"
            @select="selectDocument"
            @create-document="createDocument"
            @delete-document="deleteDocument"
            @move-document="moveDocument"
            @rename-document="renameDocument"
            @update-status="updateDocumentStatus"
        />

        <section v-else-if="workspaceMode === 'cards'" class="card-sidebar-summary">
          <h2>设定卡</h2>
          <p>管理人物、地点、概念；在正文输入 @ 即可插入设定引用。</p>
          <ul>
            <li>人物：角色身份、动机与备注</li>
            <li>地点：氛围、场景提示</li>
            <li>概念：规则、限制与说明</li>
          </ul>
        </section>

        <section v-else-if="workspaceMode === 'notes'" class="card-sidebar-summary">
          <h2>创作事项</h2>
          <p>管理备忘、待办、伏笔、问题和灵感。</p>
          <ul>
            <li>项目级：全局待办和灵感</li>
            <li>章节级：本章修订事项</li>
            <li>段落级：伏笔与情节提示</li>
          </ul>
        </section>

        <section v-else-if="workspaceMode === 'search'" class="card-sidebar-summary">
          <h2>全文搜索</h2>
          <p>在正文、设定卡和创作事项中查找关键词。</p>
          <ul>
            <li>保存时自动索引</li>
            <li>可手动重建索引</li>
            <li>搜索结果支持跳转</li>
          </ul>
        </section>

        <section v-else-if="workspaceMode === 'export'" class="card-sidebar-summary">
          <h2>导入导出</h2>
          <p>导出 txt / markdown / html / 项目包，并创建本地备份。</p>
          <ul>
            <li>全项目导出</li>
            <li>当前文档导出</li>
            <li>导入文本为章节</li>
          </ul>
        </section>

        <section v-else class="card-sidebar-summary">
          <h2>设置</h2>
          <p>调整主题、字体、字号、行距、自动保存间隔和界面语言。</p>
          <ul>
            <li>纸张 / 明亮 / 夜间主题</li>
            <li>编辑器体验设置</li>
            <li>中文 / 英文文案结构</li>
          </ul>
        </section>
      </aside>

      <main class="workspace-editor workspace-editor-host" data-phase1-area="main-workspace-host">
        <template v-if="workspaceMode === 'writing'">
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
              @paragraph-note="handleParagraphNote"
              @toggle-focus-mode="toggleFocusMode"
          />

          <section v-else class="editor-select-empty paper-card">
            请选择或创建一个卷、章节或场景。
          </section>
        </template>

        <CardWorkspace
            v-else-if="workspaceMode === 'cards'"
            :project-id="projectId"
        />

        <NoteWorkspace
            v-else-if="workspaceMode === 'notes'"
            :active-document-id="documentStore.activeDocumentId"
            :project-id="projectId"
        />

        <SearchWorkspace
            v-else-if="workspaceMode === 'search'"
            :project-id="projectId"
            @open-result="handleOpenSearchResult"
        />

        <ExportWorkspace
            v-else-if="workspaceMode === 'export'"
            :active-document-id="documentStore.activeDocumentId"
            :project-id="projectId"
            @imported="handleImportedDocument"
        />

        <SettingsWorkspace v-else/>
      </main>

      <aside class="workspace-status status-panel glass-panel" data-phase1-area="status-panel">
        <div>
          <h2 class="status-panel-title">Phase 1 Week 8</h2>
          <p class="status-panel-subtitle">主题、设置、i18n、空状态和稳定性收尾已接入。</p>
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
          <div class="status-card">
            <dt>当前主题</dt>
            <dd>{{ themeLabel }}</dd>
          </div>
          <div class="status-card">
            <dt>自动保存</dt>
            <dd>{{ (settingsStore.editorSettings.autosaveIntervalMs / 1000).toFixed(1) }} 秒</dd>
          </div>
        </dl>

        <section class="current-notes-card">
          <div class="current-notes-header">
            <h3>本章事项</h3>
            <button class="ghost-button" type="button" @click="createChapterNote('todo')">+ 待办</button>
          </div>
          <div v-if="currentDocumentNotes.length === 0" class="current-notes-empty">
            暂无未完成事项。
          </div>
          <div v-else class="current-notes-list">
            <article v-for="note in currentDocumentNotes" :key="note.id" class="current-note-item">
              <div>
                <strong>{{ note.title }}</strong>
                <small>{{ noteTypeLabel(note.type) }} · {{ notePriorityLabel(note.priority) }}
                  <template v-if="note.paragraphId"> · 段落</template>
                </small>
              </div>
              <button class="tree-action-button" type="button" @click="markNoteDone(note.id)">完成</button>
            </article>
          </div>
        </section>

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
          <label>
            字体
            <select
                :value="settingsStore.editorSettings.fontFamily"
                class="compact-select-field"
                @change="updateEditorSetting('fontFamily', $event)"
            >
              <option v-for="option in settingsStore.fontFamilyOptions" :key="option.value" :value="option.value">
                {{ option.label }}
              </option>
            </select>
          </label>
          <label>
            自动保存（毫秒）
            <input
                :value="settingsStore.editorSettings.autosaveIntervalMs"
                class="compact-number-field"
                max="10000"
                min="500"
                step="250"
                type="number"
                @change="updateEditorSetting('autosaveIntervalMs', $event)"
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
import CardWorkspace from '~/components/cards/CardWorkspace.vue'
import NoteWorkspace from '~/components/notes/NoteWorkspace.vue'
import SearchWorkspace from '~/components/search/SearchWorkspace.vue'
import ExportWorkspace from '~/components/export/ExportWorkspace.vue'
import SettingsWorkspace from '~/components/settings/SettingsWorkspace.vue'
import {toAppErrorMessage} from '~/utils/appError'
import type {ProjectStats, TodayWritingStats} from '~/types/stats'
import type {SaveState} from '~/composables/useAutoSave'
import type {DocumentCreatePayload, DocumentStatus, DocumentType, MoveDocumentInput} from '~/types/document'
import type {EditorSessionSnapshot, EditorSettings} from '~/types/editor'
import type {CreativeNote, EditorParagraphNoteRequest, NoteType} from '~/types/note'
import type {SearchResult} from '~/types/search'

const route = useRoute()
const router = useRouter()
const projectStore = useProjectStore()
const documentStore = useDocumentStore()
const settingsStore = useSettingsStore()
const timerStore = useTimerStore()
const noteStore = useNoteStore()
const exportStore = useExportStore()
const {call} = useTauriInvoke()
const {locale} = useI18n()

const projectId = computed(() => String(route.params.id))
const projectStats = ref<ProjectStats | null>(null)
const todayStats = ref<TodayWritingStats | null>(null)
const pageError = ref<string | null>(null)
const focusMode = ref(false)
const workspaceMode = ref<'writing' | 'cards' | 'notes' | 'search' | 'export' | 'settings'>('writing')
const targetDraft = ref<number | null>(null)
const currentDocumentNotes = ref<CreativeNote[]>([])

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

watch(() => documentStore.activeDocumentId, async () => {
  await refreshCurrentDocumentNotes()
})

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

const themeLabel = computed(() => {
  return settingsStore.themeOptions.find(option => option.value === settingsStore.theme)?.label ?? settingsStore.theme
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
    locale.value = settingsStore.language
    await projectStore.loadProject(projectId.value)
    await documentStore.loadDocuments(projectId.value)
    await refreshStats()
    await refreshTodayStats()
    await refreshCurrentDocumentNotes()
    await createStartupBackup()
  } catch (error) {
    pageError.value = toAppErrorMessage(error, '加载项目失败')
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


async function refreshCurrentDocumentNotes() {
  if (!documentStore.activeDocumentId) {
    currentDocumentNotes.value = []
    return
  }
  try {
    currentDocumentNotes.value = (await noteStore.loadDocumentNotes(projectId.value, documentStore.activeDocumentId, null))
        .filter(note => note.status === 'open' || note.status === 'doing')
  } catch (error) {
    console.warn('[notes] failed to refresh current document notes', error)
    currentDocumentNotes.value = []
  }
}

async function createChapterNote(type: Exclude<NoteType, 'all'> = 'todo') {
  if (!documentStore.activeDocumentId) return
  pageError.value = null
  try {
    await noteStore.createNote({
      projectId: projectId.value,
      documentId: documentStore.activeDocumentId,
      paragraphId: null,
      type,
      title: type === 'foreshadow' ? '新的伏笔' : type === 'memo' ? '新的备忘' : '新的待办',
      body: '',
      priority: type === 'foreshadow' ? 'high' : 'normal'
    })
    await refreshCurrentDocumentNotes()
  } catch (error) {
    pageError.value = toAppErrorMessage(error, '创建事项失败')
  }
}

async function handleParagraphNote(payload: EditorParagraphNoteRequest) {
  if (!documentStore.activeDocumentId) return
  pageError.value = null
  try {
    await noteStore.createNote({
      projectId: projectId.value,
      documentId: documentStore.activeDocumentId,
      paragraphId: payload.paragraphId,
      type: payload.type,
      title: paragraphNoteTitle(payload),
      body: payload.selectedText,
      priority: payload.type === 'foreshadow' ? 'high' : 'normal'
    })
    await refreshCurrentDocumentNotes()
  } catch (error) {
    pageError.value = toAppErrorMessage(error, '创建段落事项失败')
  }
}

async function markNoteDone(noteId: string) {
  pageError.value = null
  try {
    await noteStore.updateNoteStatus(noteId, 'done')
    await refreshCurrentDocumentNotes()
  } catch (error) {
    pageError.value = toAppErrorMessage(error, '更新事项失败')
  }
}

async function createStartupBackup() {
  try {
    await exportStore.createBackup(projectId.value)
    await exportStore.listBackups(projectId.value)
  } catch (error) {
    console.warn('[backup] startup backup failed', error)
  }
}

async function handleImportedDocument() {
  await documentStore.loadDocuments(projectId.value)
  await refreshStats()
}

function handleOpenSearchResult(result: SearchResult) {
  if (result.targetType === 'card') {
    workspaceMode.value = 'cards'
    const cardStore = useCardStore()
    void cardStore.loadCards(projectId.value).then(() => cardStore.selectCard(result.targetId))
    return
  }
  if (result.targetType === 'note') {
    workspaceMode.value = 'notes'
    noteStore.selectNote(result.targetId)
    return
  }
  workspaceMode.value = 'writing'
  documentStore.selectDocument(result.targetId)
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
    pageError.value = toAppErrorMessage(error, '创建文档失败')
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
    pageError.value = toAppErrorMessage(error, '重命名文档失败')
  }
}

async function deleteDocument(documentId: string) {
  pageError.value = null
  try {
    await documentStore.deleteDocument(documentId)
    await refreshStats()
  } catch (error) {
    pageError.value = toAppErrorMessage(error, '删除文档失败')
  }
}

async function moveDocument(input: MoveDocumentInput) {
  pageError.value = null
  try {
    await documentStore.moveDocument(input)
    await refreshStats()
  } catch (error) {
    pageError.value = toAppErrorMessage(error, '移动文档失败')
  }
}

async function updateDocumentStatus(documentId: string, status: DocumentStatus) {
  pageError.value = null
  try {
    await documentStore.updateDocumentStatus({documentId, status})
    await refreshStats()
  } catch (error) {
    pageError.value = toAppErrorMessage(error, '更新文档状态失败')
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
    pageError.value = toAppErrorMessage(error, '更新目标字数失败')
  }
}

async function handleSaved() {
  await refreshStats()
  await refreshTodayStats()
  await refreshCurrentDocumentNotes()
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
  const target = event.target as HTMLInputElement | HTMLSelectElement
  try {
    if (key === 'fontFamily') {
      await settingsStore.updateEditorSetting('fontFamily', target.value as EditorSettings['fontFamily'])
      return
    }

    const value = Number(target.value)
    if (!Number.isFinite(value)) return

    if (key === 'fontSize') await settingsStore.updateEditorSetting('fontSize', value)
    else if (key === 'lineHeight') await settingsStore.updateEditorSetting('lineHeight', value)
    else if (key === 'pageWidth') await settingsStore.updateEditorSetting('pageWidth', value)
    else if (key === 'autosaveIntervalMs') await settingsStore.updateEditorSetting('autosaveIntervalMs', value)
  } catch (error) {
    pageError.value = toAppErrorMessage(error, '保存编辑器设置失败')
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

function paragraphNoteTitle(payload: EditorParagraphNoteRequest) {
  const prefix = payload.type === 'foreshadow'
      ? '段落伏笔'
      : payload.type === 'todo'
          ? '段落待办'
          : '段落备忘'
  const selected = payload.selectedText.trim().replace(/\s+/g, ' ').slice(0, 28)
  return selected ? `${prefix}：${selected}` : prefix
}

function noteTypeLabel(type: string) {
  switch (type) {
    case 'memo':
      return '备忘'
    case 'todo':
      return '待办'
    case 'foreshadow':
      return '伏笔'
    case 'issue':
      return '问题'
    case 'idea':
      return '灵感'
    default:
      return type
  }
}

function notePriorityLabel(priority: string) {
  switch (priority) {
    case 'high':
      return '高'
    case 'low':
      return '低'
    default:
      return '普通'
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
