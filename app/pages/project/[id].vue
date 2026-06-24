<template>
  <AppShell
      :command-shortcut="formattedCommandShortcut"
      :show-command="true"
      :status="topbarStatus"
      :subtitle="activeDocument?.title || activeWorkspaceDefinition.label"
      :title="projectStore.currentProject?.title || 'Mythistorima'"
      @command="openCommandPalette"
      @home="router.push('/')"
      @settings="showSettings = true"
  >
    <div v-if="commandFeedback" class="command-feedback-toast">{{ commandFeedback }}</div>

    <div :class="{ 'is-focus-mode': focusMode }" class="workspace-layout">
      <aside class="workspace-sidebar app-sidebar glass-panel" data-area="workspace-sidebar">
        <nav aria-label="一级视图" class="project-primary-navigation">
          <UTooltip
              v-for="view in projectPrimaryViews"
              :key="view.id"
              :delay-duration="250"
              :text="view.description"
          >
            <UButton
                :class="['project-view-button', { 'is-active': activePrimaryView === view.id }]"
                :color="activePrimaryView === view.id ? 'primary' : 'neutral'"
                :icon="view.icon"
                :label="view.label"
                :variant="activePrimaryView === view.id ? 'soft' : 'ghost'"
                size="sm"
                @click="selectPrimaryView(view.id)"
            />
          </UTooltip>
        </nav>

        <USeparator/>

        <nav
            v-if="secondaryWorkspaces.length > 1"
            :aria-label="`${activePrimaryDefinition.label}视图`"
            class="project-secondary-navigation"
        >
          <UButton
              v-for="item in secondaryWorkspaces"
              :key="item.mode"
              :color="workspaceMode === item.mode ? 'primary' : 'neutral'"
              :icon="item.icon"
              :label="item.label"
              :variant="workspaceMode === item.mode ? 'soft' : 'ghost'"
              size="xs"
              @click="selectWorkspaceMode(item.mode)"
          />
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

        <section v-else class="project-sidebar-context">
          <h2>{{ activeWorkspaceDefinition.label }}</h2>
          <p>{{ activeWorkspaceDefinition.description }}</p>
        </section>
      </aside>

      <main class="workspace-editor workspace-editor-host">
        <ProjectDashboard
            v-if="workspaceMode === 'dashboard' && projectStore.currentProject"
            :backups="exportStore.backups"
            :project="projectStore.currentProject"
            :saving="projectSaving"
            :stats="projectStats"
            @backup="createManualBackup"
            @delete-project="deleteCurrentProject"
            @open-mode="selectWorkspaceMode($event)"
            @update-project="updateProjectInfo"
        />

        <template v-else-if="workspaceMode === 'writing'">
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

        <OutlineWorkspace
            v-else-if="workspaceMode === 'outline'"
            :documents="documentStore.documents"
            :project-id="projectId"
            @open-document="targetId => openTarget({type: 'document', targetId, source: 'outline'})"
        />

        <OutlineBoardWorkspace
            v-else-if="workspaceMode === 'board'"
            :documents="documentStore.documents"
            :project-id="projectId"
            @open-document="targetId => openTarget({type: 'document', targetId, source: 'outline'})"
        />

        <TimelineWorkspace
            v-else-if="workspaceMode === 'timeline'"
            :documents="documentStore.documents"
            :project-id="projectId"
            @open-target="openTarget"
        />

        <CardWorkspace
            v-else-if="workspaceMode === 'cards'"
            :project-id="projectId"
        />

        <RelationWorkspace
            v-else-if="workspaceMode === 'relations'"
            :project-id="projectId"
            @open-target="openTarget"
        />

        <StatsWorkspace
            v-else-if="workspaceMode === 'stats'"
            :documents="documentStore.documents"
            :project-id="projectId"
            @open-target="openTarget"
        />

        <NoteWorkspace
            v-else-if="workspaceMode === 'notes'"
            :active-document-id="documentStore.activeDocumentId"
            :documents="documentStore.documents"
            :project-id="projectId"
        />

        <ForeshadowWorkspace
            v-else-if="workspaceMode === 'foreshadow'"
            :documents="documentStore.documents"
            :project-id="projectId"
            @open-target="openTarget"
        />

        <ProofreadingWorkspace
            v-else-if="workspaceMode === 'proofreading'"
            :active-document-id="documentStore.activeDocumentId"
            :documents="documentStore.documents"
            :project-id="projectId"
            @open-target="openTarget"
        />

        <SearchWorkspace
            v-else-if="workspaceMode === 'search'"
            :project-id="projectId"
            @open-target="openTarget"
        />

        <ExportWorkspace
            v-else-if="workspaceMode === 'export'"
            :active-document-id="documentStore.activeDocumentId"
            :documents="documentStore.documents"
            :project-id="projectId"
            @imported="handleImportedDocument"
        />

        <UEmpty
            v-else
            description="请选择左侧的工作视图。"
            icon="i-lucide-panel-left"
            title="未选择工作区"
        />
      </main>

      <aside class="workspace-status status-panel glass-panel">
        <WorkspaceContextPanel
            v-if="workspaceMode !== 'writing'"
            :active-document-title="activeDocument?.title"
            :project-character-count="projectStats?.characterCount ?? 0"
            :project-id="projectId"
            :workspace="workspaceMode"
            @open-target="openTarget"
        />
        <template v-else>
          <div>
            <h2 class="status-panel-title">写作上下文</h2>
            <p class="status-panel-subtitle">当前章节、写作进度、未完成事项和编辑器设置。</p>
          </div>

          <dl class="status-list">
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

        </template>
        <ErrorBanner :message="pageError" title="工作区操作失败" @dismiss="pageError = null"/>
        <ErrorBanner
            :message="editorSnapshot.errorMessage"
            title="编辑器错误"
            @dismiss="editorSnapshot.errorMessage = null"
        />
      </aside>
    </div>

    <CommandPalette
        :items="commandPaletteItems"
        :open="commandStore.isPaletteOpen"
        :open-shortcut="commandStore.shortcutFor('commandPalette.open')"
        @close="commandStore.closePalette"
        @execute="executeCommandPaletteItem"
    />

    <AppSettingsModal v-model:open="showSettings"/>
  </AppShell>
</template>

<script lang="ts" setup>
import AppShell from '~/components/layout/AppShell.vue'
import DocumentTree from '~/components/project/DocumentTree.vue'
import ProjectDashboard from '~/components/project/ProjectDashboard.vue'
import NovelEditor from '~/components/editor/NovelEditor.vue'
import EditorFocusOverlay from '~/components/editor/EditorFocusOverlay.vue'
import CardWorkspace from '~/components/cards/CardWorkspace.vue'
import NoteWorkspace from '~/components/notes/NoteWorkspace.vue'
import SearchWorkspace from '~/components/search/SearchWorkspace.vue'
import ExportWorkspace from '~/components/export/ExportWorkspace.vue'
import OutlineWorkspace from '~/components/outline/OutlineWorkspace.vue'
import OutlineBoardWorkspace from '~/components/outline/OutlineBoardWorkspace.vue'
import TimelineWorkspace from '~/components/timeline/TimelineWorkspace.vue'
import RelationWorkspace from '~/components/relations/RelationWorkspace.vue'
import ForeshadowWorkspace from '~/components/foreshadow/ForeshadowWorkspace.vue'
import StatsWorkspace from '~/components/stats/StatsWorkspace.vue'
import ProofreadingWorkspace from '~/components/proofreading/ProofreadingWorkspace.vue'
import CommandPalette from '~/components/command/CommandPalette.vue'
import WorkspaceContextPanel from '~/components/layout/WorkspaceContextPanel.vue'
import ErrorBanner from '~/components/common/ErrorBanner.vue'
import AppSettingsModal from '~/components/settings/AppSettingsModal.vue'
import {
  getPrimaryViewDefinition,
  getPrimaryViewForWorkspace,
  getSecondaryWorkspaces,
  getWorkspaceDefinition,
  type ProjectPrimaryView,
  projectPrimaryViews,
  type ProjectWorkspaceMode
} from '~/constants/projectViews'
import {toAppErrorMessage} from '~/utils/appError'
import {appCommandRegistry} from '~/constants/commandRegistry'
import {cardTypeLabel, defaultCardName, defaultFieldsJson} from '~/types/card'
import type {AppCommandId, CommandPaletteItem} from '~/types/command'
import type {ProjectStats, TodayWritingStats} from '~/types/stats'
import type {SaveState} from '~/composables/useAutoSave'
import {useAppShortcuts} from '~/composables/useAppShortcuts'
import type {DocumentCreatePayload, DocumentStatus, DocumentType, MoveDocumentInput} from '~/types/document'
import type {UpdateProjectInput} from '~/types/project'
import type {EditorSessionSnapshot, EditorSettings} from '~/types/editor'
import type {CreativeNote, EditorParagraphNoteRequest, NoteType} from '~/types/note'
import type {OpenTarget} from '~/types/navigation'

const route = useRoute()
const router = useRouter()
const projectStore = useProjectStore()
const documentStore = useDocumentStore()
const settingsStore = useSettingsStore()
const timerStore = useTimerStore()
const noteStore = useNoteStore()
const cardStore = useCardStore()
const proofreadingStore = useProofreadingStore()
const exportStore = useExportStore()
const commandStore = useCommandStore()
const navigationStore = useNavigationStore()
const outlineStore = useOutlineStore()
const timelineStore = useTimelineStore()
const relationStore = useRelationStore()
const foreshadowStore = useForeshadowStore()
const exportTemplateStore = useExportTemplateStore()
const {call} = useTauriInvoke()
const {locale} = useI18n()

const projectId = computed(() => String(route.params.id))
const projectStats = ref<ProjectStats | null>(null)
const todayStats = ref<TodayWritingStats | null>(null)
const pageError = ref<string | null>(null)
const commandFeedback = ref<string | null>(null)
const focusMode = ref(false)
const workspaceMode = ref<ProjectWorkspaceMode>('writing')
const showSettings = ref(false)
const targetDraft = ref<number | null>(null)
const currentDocumentNotes = ref<CreativeNote[]>([])
const projectSaving = ref(false)
let backupInterval: ReturnType<typeof setInterval> | null = null
let commandFeedbackTimer: ReturnType<typeof setTimeout> | null = null

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


const activePrimaryView = computed(() => getPrimaryViewForWorkspace(workspaceMode.value))
const activePrimaryDefinition = computed(() => getPrimaryViewDefinition(activePrimaryView.value))
const secondaryWorkspaces = computed(() => getSecondaryWorkspaces(activePrimaryView.value))
const activeWorkspaceDefinition = computed(() => getWorkspaceDefinition(workspaceMode.value))
const formattedCommandShortcut = computed(() => {
  const shortcut = commandStore.shortcutFor('commandPalette.open')
  const isMac = typeof navigator !== 'undefined' && navigator.platform.includes('Mac')
  return shortcut.replace('Mod', isMac ? '⌘' : 'Ctrl')
})


const commandPaletteItems = computed<CommandPaletteItem[]>(() => {
  const commandItems: CommandPaletteItem[] = appCommandRegistry
      .filter(command => command.id !== 'commandPalette.open')
      .map(command => ({
        id: `command:${command.id}`,
        kind: 'command',
        title: command.title,
        subtitle: command.description,
        group: command.group,
        keywords: command.keywords,
        shortcut: commandStore.shortcutFor(command.id),
        action: {type: 'command', commandId: command.id}
      }))

  const documentItems: CommandPaletteItem[] = documentStore.documents.map(document => ({
    id: `document:${document.id}`,
    kind: 'document',
    title: document.title,
    subtitle: `${documentTypeLabel(document.type)} · ${documentStatusLabel(document.status)} · ${document.characterCount} 字`,
    group: '打开章节',
    keywords: ['章节', '卷', '场景', document.type, document.status],
    action: {type: 'openDocument', targetId: document.id}
  }))

  const cardItems: CommandPaletteItem[] = cardStore.cards.map(card => ({
    id: `card:${card.id}`,
    kind: 'card',
    title: card.name,
    subtitle: `${cardTypeLabel(card.type)} · ${card.description || '暂无简介'}`,
    group: '打开设定',
    keywords: [card.type, card.aliasesJson, card.description],
    action: {type: 'openCard', targetId: card.id}
  }))

  const noteItems: CommandPaletteItem[] = noteStore.notes.map(note => ({
    id: `note:${note.id}`,
    kind: 'note',
    title: note.title,
    subtitle: `${noteTypeLabel(note.type)} · ${note.status === 'done' ? '已完成' : '未完成'}${note.documentTitle ? ` · ${note.documentTitle}` : ''}`,
    group: '打开事项',
    keywords: [note.type, note.status, note.priority, note.body],
    action: {type: 'openNote', targetId: note.id}
  }))

  return [...commandItems, ...documentItems, ...cardItems, ...noteItems]
})

useAppShortcuts({
  'commandPalette.open': toggleCommandPalette,
  'document.createChapter': () => executeAppCommandSafely('document.createChapter'),
  'card.createCharacter': () => executeAppCommandSafely('card.createCharacter'),
  'editor.toggleFocus': () => executeAppCommandSafely('editor.toggleFocus'),
  'proofreading.runCurrent': () => executeAppCommandSafely('proofreading.runCurrent'),
  'export.currentDocument': () => executeAppCommandSafely('export.currentDocument'),
  'theme.cycle': () => executeAppCommandSafely('theme.cycle'),
  'navigation.settings': () => executeAppCommandSafely('navigation.settings')
})

onMounted(async () => {
  await loadProjectWorkspace()
  commandStore.loadShortcuts()
  startBackupInterval()
})

onBeforeUnmount(() => {
  stopBackupInterval()
  if (commandFeedbackTimer) clearTimeout(commandFeedbackTimer)
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

function selectPrimaryView(viewId: ProjectPrimaryView) {
  const definition = getPrimaryViewDefinition(viewId)
  workspaceMode.value = definition.defaultMode
  focusMode.value = false
}

function selectWorkspaceMode(mode: ProjectWorkspaceMode) {
  workspaceMode.value = mode
  if (mode !== 'writing') focusMode.value = false
}

async function openCommandPalette() {
  commandStore.openPalette()
  await Promise.allSettled([
    cardStore.loadCards(projectId.value),
    noteStore.loadNotes({projectId: projectId.value, type: 'all', status: 'all'})
  ])
}

function toggleCommandPalette() {
  if (commandStore.isPaletteOpen) {
    commandStore.closePalette()
    return
  }
  void openCommandPalette()
}

async function executeCommandPaletteItem(item: CommandPaletteItem) {
  commandStore.closePalette()
  pageError.value = null
  try {
    if (item.action.type === 'command') {
      await executeAppCommandSafely(item.action.commandId)
      return
    }
    if (item.action.type === 'openDocument') {
      await openTarget({type: 'document', targetId: item.action.targetId, source: 'direct'})
      return
    }
    if (item.action.type === 'openCard') {
      await openTarget({type: 'card', targetId: item.action.targetId})
      return
    }
    if (item.action.type === 'openNote') {
      await openTarget({type: 'note', targetId: item.action.targetId})
    }
  } catch (error) {
    pageError.value = toAppErrorMessage(error, '执行命令失败')
  }
}

async function executeAppCommandSafely(commandId: AppCommandId) {
  pageError.value = null
  try {
    await executeAppCommand(commandId)
  } catch (error) {
    const message = toAppErrorMessage(error, '执行命令失败')
    pageError.value = message
    showCommandFeedback(message, 8000)
  }
}

async function executeAppCommand(commandId: AppCommandId) {
  switch (commandId) {
    case 'commandPalette.open':
      await openCommandPalette()
      break
    case 'document.createChapter':
      await commandCreateChapter()
      break
    case 'card.createCharacter':
      await commandCreateCharacter()
      break
    case 'editor.toggleFocus':
      commandToggleFocusMode()
      break
    case 'proofreading.runCurrent':
      await commandRunProofreading()
      break
    case 'export.currentDocument':
      await commandExportCurrentDocument()
      break
    case 'theme.cycle':
      await commandCycleTheme()
      break
    case 'navigation.settings':
      commandOpenSettings()
      break
  }
}

async function commandCreateChapter() {
  const parentId = resolveChapterParentId()
  const document = await documentStore.createDocument({
    projectId: projectId.value,
    parentId,
    type: 'chapter',
    title: '新章节',
    sortOrder: null
  })
  workspaceMode.value = 'writing'
  await refreshStats()
  showCommandFeedback(`已创建“${document.title}”。`)
}

function resolveChapterParentId() {
  let cursor = activeDocument.value
  const visited = new Set<string>()
  while (cursor && !visited.has(cursor.id)) {
    visited.add(cursor.id)
    if (cursor.type === 'volume') return cursor.id
    const parentId = cursor.parentId
    cursor = parentId
        ? documentStore.documents.find(document => document.id === parentId) ?? null
        : null
  }
  return null
}

async function commandCreateCharacter() {
  if (!cardStore.cards.length) await cardStore.loadCards(projectId.value)
  const card = await cardStore.createCard({
    projectId: projectId.value,
    type: 'character',
    name: defaultCardName('character'),
    aliasesJson: '[]',
    description: '',
    fieldsJson: defaultFieldsJson('character'),
    avatarAssetId: null
  })
  cardStore.setTypeFilter('character')
  cardStore.selectCard(card.id)
  workspaceMode.value = 'cards'
  showCommandFeedback('已创建人物设定卡。')
}

function commandToggleFocusMode() {
  workspaceMode.value = 'writing'
  focusMode.value = !focusMode.value
  showCommandFeedback(focusMode.value ? '已进入专注模式。' : '已退出专注模式。')
}

async function commandRunProofreading() {
  if (!documentStore.activeDocumentId) {
    throw new Error('请先选择需要校对的章节或场景')
  }
  await proofreadingStore.loadRules({
    projectId: projectId.value,
    includeBuiltin: true
  })
  const issues = await proofreadingStore.runOnDocument({
    documentId: documentStore.activeDocumentId,
    enabledOnly: true,
    ruleIds: null
  })
  workspaceMode.value = 'proofreading'
  showCommandFeedback(`校对完成，共发现 ${issues.length} 个问题。`)
}

async function commandExportCurrentDocument() {
  if (!documentStore.activeDocumentId) {
    throw new Error('请先选择需要导出的章节或场景')
  }
  const result = await exportStore.exportDocuments({
    projectId: projectId.value,
    format: 'txt',
    range: 'current',
    documentId: documentStore.activeDocumentId,
    documentIds: null,
    outputPath: null
  })
  showCommandFeedback(`当前文档已导出到：${result.path}`, 8000)
}

async function commandCycleTheme() {
  const themes = settingsStore.themeOptions.map(option => option.value)
  const index = themes.indexOf(settingsStore.theme)
  const next = themes[(index + 1) % themes.length] ?? 'paper'
  await settingsStore.setTheme(next)
  showCommandFeedback(`已切换为“${settingsStore.themeOptions.find(option => option.value === next)?.label ?? next}”主题。`)
}

function commandOpenSettings() {
  focusMode.value = false
  showSettings.value = true
  showCommandFeedback('已打开设置。')
}

function showCommandFeedback(message: string, duration = 4000) {
  commandFeedback.value = message
  if (commandFeedbackTimer) clearTimeout(commandFeedbackTimer)
  commandFeedbackTimer = setTimeout(() => {
    commandFeedback.value = null
    commandFeedbackTimer = null
  }, duration)
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

function startBackupInterval() {
  stopBackupInterval()
  backupInterval = setInterval(() => {
    void exportStore.createBackup(projectId.value)
        .then(() => exportStore.listBackups(projectId.value))
        .catch(error => console.warn('[backup] scheduled backup failed', error))
  }, 15 * 60 * 1000)
}

function stopBackupInterval() {
  if (backupInterval) {
    clearInterval(backupInterval)
    backupInterval = null
  }
}

async function createManualBackup() {
  pageError.value = null
  try {
    await exportStore.createBackup(projectId.value)
    await exportStore.listBackups(projectId.value)
  } catch (error) {
    pageError.value = toAppErrorMessage(error, '创建备份失败')
  }
}

async function updateProjectInfo(input: UpdateProjectInput) {
  projectSaving.value = true
  pageError.value = null
  try {
    await projectStore.updateProject(input)
    await projectStore.loadProjects()
  } catch (error) {
    pageError.value = toAppErrorMessage(error, '保存项目信息失败')
  } finally {
    projectSaving.value = false
  }
}

async function deleteCurrentProject() {
  pageError.value = null
  try {
    await projectStore.deleteProject(projectId.value)
    await router.push('/')
  } catch (error) {
    pageError.value = toAppErrorMessage(error, '删除项目失败')
  }
}

async function handleImportedDocument() {
  await documentStore.loadDocuments(projectId.value)
  await refreshStats()
}

async function openTarget(target: OpenTarget) {
  pageError.value = null
  navigationStore.rememberTarget(target)

  try {
    switch (target.type) {
      case 'workspace':
        focusMode.value = false
        workspaceMode.value = target.workspace
        break

      case 'document':
        if (!documentStore.documents.some(document => document.id === target.targetId)) {
          await documentStore.loadDocuments(projectId.value)
        }
        if (!documentStore.documents.some(document => document.id === target.targetId)) {
          throw new Error('目标文档不存在或已被删除')
        }
        navigationStore.requestDocumentNavigation(target)
        documentStore.selectDocument(target.targetId)
        workspaceMode.value = 'writing'
        break

      case 'card':
        cardStore.setTypeFilter('all')
        await cardStore.loadCards(projectId.value)
        if (!cardStore.cards.some(card => card.id === target.targetId)) {
          throw new Error('目标设定卡不存在或已被删除')
        }
        cardStore.selectCard(target.targetId)
        workspaceMode.value = 'cards'
        break

      case 'note':
        noteStore.setTypeFilter('all')
        noteStore.setStatusFilter('all')
        await noteStore.loadNotes({projectId: projectId.value, type: 'all', status: 'all'})
        if (!noteStore.notes.some(note => note.id === target.targetId)) {
          throw new Error('目标事项不存在或已被删除')
        }
        noteStore.selectNote(target.targetId)
        workspaceMode.value = 'notes'
        break

      case 'outline':
        await outlineStore.loadOutlineNodes(projectId.value)
        outlineStore.selectOutlineNode(target.targetId)
        workspaceMode.value = 'outline'
        break

      case 'timeline':
        timelineStore.setFilters(null, null)
        await timelineStore.loadTimeline(projectId.value)
        timelineStore.selectEvent(target.targetId)
        workspaceMode.value = 'timeline'
        break

      case 'relation':
        await relationStore.loadGraph(projectId.value)
        relationStore.selectRelation(target.targetId)
        workspaceMode.value = 'relations'
        break

      case 'foreshadow':
        foreshadowStore.setFilters('all', 'all', false)
        await foreshadowStore.loadThreads(projectId.value)
        foreshadowStore.selectThread(target.targetId)
        workspaceMode.value = 'foreshadow'
        break

      case 'proofreadingRule':
        await proofreadingStore.loadRules({projectId: projectId.value, includeBuiltin: true})
        proofreadingStore.selectRule(target.targetId)
        workspaceMode.value = 'proofreading'
        break

      case 'proofreadingIssue':
        proofreadingStore.selectIssue(target.issue.id)
        await openTarget({
          type: 'document',
          targetId: target.issue.documentId,
          paragraphId: target.issue.paragraphId,
          startOffset: target.issue.startOffset,
          endOffset: target.issue.endOffset,
          source: 'proofreading',
          label: target.issue.message
        })
        return

      case 'exportTemplate':
        await exportTemplateStore.loadTemplates({projectId: projectId.value, includeBuiltin: true})
        exportTemplateStore.selectTemplate(target.targetId)
        workspaceMode.value = 'export'
        break
    }

    navigationStore.setNavigationMessage(targetMessage(target))
    showCommandFeedback(targetMessage(target))
  } catch (error) {
    const message = toAppErrorMessage(error, '无法打开目标')
    pageError.value = message
    navigationStore.setNavigationMessage(message)
    showCommandFeedback(message, 8000)
  }
}

function targetMessage(target: OpenTarget) {
  switch (target.type) {
    case 'document':
      return target.paragraphId || target.startOffset != null ? '已定位到正文位置。' : '已打开文档。'
    case 'card':
      return '已打开设定卡。'
    case 'note':
      return '已打开事项。'
    case 'outline':
      return '已打开大纲节点。'
    case 'timeline':
      return '已打开时间线事件。'
    case 'relation':
      return '已打开设定关系。'
    case 'foreshadow':
      return '已打开伏笔线程。'
    case 'proofreadingRule':
      return '已打开校对规则。'
    case 'proofreadingIssue':
      return '已定位校对问题。'
    case 'exportTemplate':
      return '已打开导出模板。'
    case 'workspace':
      return '已切换工作区。'
  }
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
