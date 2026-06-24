<template>
  <AppShell
      :status="topbarStatus"
      :subtitle="activeDocument?.title || '长篇创作工作台'"
      :title="projectStore.currentProject?.title || 'Mythistorima'"
      @home="router.push('/')"
  >
    <button class="command-palette-launcher" type="button" @click="openCommandPalette">
      <span>命令</span>
      <CommandShortcutHint :shortcut="commandStore.shortcutFor('commandPalette.open')" compact/>
    </button>
    <div v-if="commandFeedback" class="command-feedback-toast">{{ commandFeedback }}</div>

    <div :class="{ 'is-focus-mode': focusMode }" class="workspace-layout">
      <aside class="workspace-sidebar app-sidebar glass-panel" data-phase1-area="workspace-sidebar">
        <nav aria-label="项目工作区" class="workspace-mode-switch phase2-workspace-nav">
          <section v-for="group in phase2WorkspaceGroups" :key="group.label" class="workspace-mode-group">
            <h3>{{ group.label }}</h3>
            <button
                v-for="item in group.items"
                :key="item.mode"
                :class="['workspace-mode-button', { 'is-active': workspaceMode === item.mode }]"
                :title="item.description"
                type="button"
                @click="workspaceMode = item.mode"
            >
              {{ item.label }}
            </button>
          </section>
        </nav>

        <section v-if="workspaceMode === 'dashboard'" class="card-sidebar-summary">
          <h2>项目概览</h2>
          <p>编辑作品信息、查看进度、创建备份，并快速进入各个工作区。</p>
          <ul>
            <li>作品标题、作者和简介</li>
            <li>项目 / 每日目标字数</li>
            <li>最近备份与快速入口</li>
          </ul>
        </section>

        <DocumentTree
            v-else-if="workspaceMode === 'writing'"
            :active-document-id="documentStore.activeDocumentId"
            :items="documentStore.documentTree"
            @select="selectDocument"
            @create-document="createDocument"
            @delete-document="deleteDocument"
            @move-document="moveDocument"
            @rename-document="renameDocument"
            @update-status="updateDocumentStatus"
        />

        <section v-else-if="workspaceMode === 'outline'" class="card-sidebar-summary">
          <h2>大纲</h2>
          <p>规划剧情节点、冲突、转折、支线和主题，并绑定到章节或场景。</p>
          <ul>
            <li>outline_nodes 数据表已就绪</li>
            <li>支持剧情、冲突、转折等节点类型</li>
            <li>可绑定章节或场景并快速跳转</li>
          </ul>
        </section>

        <section v-else-if="workspaceMode === 'board'" class="card-sidebar-summary">
          <h2>看板</h2>
          <p>以 planned / drafting / done 管理大纲节点状态，并生成 Mermaid 剧情流程图。</p>
          <ul>
            <li>拖动卡片切换剧情状态</li>
            <li>一键复制 Mermaid flowchart 文本</li>
            <li>预览剧情节点和章节绑定关系</li>
          </ul>
        </section>

        <section v-else-if="workspaceMode === 'timeline'" class="card-sidebar-summary">
          <h2>时间线</h2>
          <p>管理故事事件顺序、参与角色和地点过滤。</p>
          <ul>
            <li>创建、编辑、删除时间线事件</li>
            <li>绑定章节、地点与参与设定</li>
            <li>按人物 / 地点筛选角色经历</li>
          </ul>
        </section>

        <section v-else-if="workspaceMode === 'cards'" class="card-sidebar-summary">
          <h2>设定卡</h2>
          <p>管理人物、地点、组织、道具、事件和概念；在正文输入 @ 即可插入设定引用。</p>
          <ul>
            <li>人物：角色身份、动机与备注</li>
            <li>地点：氛围、场景提示</li>
            <li>组织 / 道具 / 事件：用于关系图和世界观网络</li>
            <li>概念：规则、限制与说明</li>
          </ul>
        </section>

        <section v-else-if="workspaceMode === 'relations'" class="card-sidebar-summary">
          <h2>关系图</h2>
          <p>建立设定卡之间的人物关系、组织归属、道具持有和事件参与。</p>
          <ul>
            <li>card_relations 数据表已就绪</li>
            <li>支持节点点击打开设定卡</li>
            <li>支持边点击编辑关系说明</li>
          </ul>
        </section>

        <section v-else-if="workspaceMode === 'stats'" class="card-sidebar-summary">
          <h2>统计</h2>
          <p>追踪角色出场、章节矩阵和长篇结构统计。</p>
          <ul>
            <li>appearance_stats 缓存表已就绪</li>
            <li>Week 6 接入出场统计</li>
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

        <section v-else-if="workspaceMode === 'foreshadow'" class="card-sidebar-summary">
          <h2>伏笔线程</h2>
          <p>把 Phase 1 的伏笔事项升级为提出 / 回收 / 状态追踪。</p>
          <ul>
            <li>foreshadow_threads 数据表已就绪</li>
            <li>Week 6 接入伏笔线程工作流</li>
          </ul>
        </section>

        <section v-else-if="workspaceMode === 'proofreading'" class="card-sidebar-summary">
          <h2>校对</h2>
          <p>本地规则检查重复词、标点、超长句、敏感词和名称一致性。</p>
          <ul>
            <li>proofreading_rules 数据表已就绪</li>
            <li>内置基础规则种子已准备</li>
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
          <p>使用可复用模板导出 TXT、Markdown、HTML、DOCX、EPUB、Pixiv 文本或项目包。</p>
          <ul>
            <li>内置与项目级模板</li>
            <li>全项目 / 当前树 / 自选文档</li>
            <li>样式配置与精确预览</li>
          </ul>
        </section>

        <section v-else class="card-sidebar-summary">
          <h2>设置</h2>
          <p>调整主题、编辑器体验、界面语言和应用内快捷键。</p>
          <ul>
            <li>纸张 / 明亮 / 夜间主题</li>
            <li>编辑器与自动保存设置</li>
            <li>快捷键录制、冲突检测与恢复默认</li>
          </ul>
        </section>
      </aside>

      <main class="workspace-editor workspace-editor-host" data-phase1-area="main-workspace-host">
        <ProjectDashboard
            v-if="workspaceMode === 'dashboard' && projectStore.currentProject"
            :backups="exportStore.backups"
            :project="projectStore.currentProject"
            :saving="projectSaving"
            :stats="projectStats"
            @backup="createManualBackup"
            @delete-project="deleteCurrentProject"
            @open-mode="workspaceMode = $event"
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

        <SettingsWorkspace v-else/>
      </main>

      <aside class="workspace-status status-panel glass-panel" data-phase1-area="status-panel">
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
import SettingsWorkspace from '~/components/settings/SettingsWorkspace.vue'
import OutlineWorkspace from '~/components/outline/OutlineWorkspace.vue'
import OutlineBoardWorkspace from '~/components/outline/OutlineBoardWorkspace.vue'
import TimelineWorkspace from '~/components/timeline/TimelineWorkspace.vue'
import RelationWorkspace from '~/components/relations/RelationWorkspace.vue'
import ForeshadowWorkspace from '~/components/foreshadow/ForeshadowWorkspace.vue'
import StatsWorkspace from '~/components/stats/StatsWorkspace.vue'
import ProofreadingWorkspace from '~/components/proofreading/ProofreadingWorkspace.vue'
import CommandPalette from '~/components/command/CommandPalette.vue'
import CommandShortcutHint from '~/components/command/CommandShortcutHint.vue'
import WorkspaceContextPanel from '~/components/layout/WorkspaceContextPanel.vue'
import ErrorBanner from '~/components/common/ErrorBanner.vue'
import {phase2WorkspaceGroups, type Phase2WorkspaceMode} from '~/constants/phase2Features'
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
const workspaceMode = ref<Phase2WorkspaceMode>('writing')
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
  workspaceMode.value = 'settings'
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
