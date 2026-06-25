<template>
  <section
      :class="{
        'has-left-tools': leftSideSlotWindows.length > 0,
        'has-right-tools': rightSideSlotWindows.length > 0,
        'has-bottom-tools': bottomSlotWindows.length > 0,
        'is-focus-mode': controller.focusMode.value
      }"
      :style="stageVars"
      class="writing-workspace"
  >
    <DocumentTabs
        :active-document-id="controller.documentStore.activeDocumentId"
        :tabs="tabsStore.tabs"
        @activate="activateTab"
        @close="closeTab"
        @pin="tabsStore.pin"
        @close-all="closeAllTabs"
        @close-others="tabsStore.closeOthers"
    />

    <div class="writing-workspace-stage">
      <WorkspaceToolStrip
          v-if="!controller.focusMode.value"
          :windows="layoutStore.leftWindows"
          side="left"
          @toggle="layoutStore.toggleVisible"
      />

      <aside
          v-if="!controller.focusMode.value && leftSideSlotWindows.length"
          :style="sideColumnStyle('left')"
          class="writing-tool-column is-left"
      >
        <template v-for="(slot, index) in leftSideSlotWindows" :key="`left-${slot.position}`">
          <div
              :class="['writing-tool-slot', `at-${slot.position}`, { 'is-single': leftSideSlotWindows.length === 1, 'is-split': leftSideSlotWindows.length > 1 }]"
              :style="sideSlotStyle('left', index, leftSideSlotWindows.length)"
          >
            <WorkspaceToolWindow
                :icon="toolIcon(slot.window.id)"
                :state="slot.window"
                :title="toolTitle(slot.window.id)"
                dock="side"
                @close="layoutStore.close"
                @resize="layoutStore.setSize"
                @set-slot="layoutStore.setSlot"
            >
              <ToolWindowContent :id="slot.window.id"/>
            </WorkspaceToolWindow>
          </div>

          <button
              v-if="index === 0 && leftSideSlotWindows.length > 1"
              :key="`left-splitter-${slot.position}`"
              aria-label="调整左侧工具窗口高度"
              class="writing-tool-splitter is-horizontal"
              type="button"
              @pointerdown="startSideSplitResize('left', $event)"
          />
        </template>
      </aside>

      <main class="writing-editor-surface">
        <EditorFocusOverlay
            v-if="controller.focusMode.value"
            @exit="controller.toggleFocusMode"
        />

        <EditorBridge
            v-if="controller.documentStore.activeDocumentId"
            :key="controller.documentStore.activeDocumentId"
            :document-id="controller.documentStore.activeDocumentId"
            :focus-mode="controller.focusMode.value"
            :project-id="controller.projectId.value"
            :settings="controller.settingsStore.editorSettings"
            :show-status-bar="false"
            :target-character-count="controller.activeDocumentTarget.value"
            @saved="controller.handleSaved"
            @session="controller.handleEditorSession"
            @status="handleEditorStatus"
            @paragraph-note="controller.handleParagraphNote"
            @toggle-focus-mode="controller.toggleFocusMode"
        />

        <UEmpty
            v-else
            class="project-workspace-empty writing-empty-state"
            description="从项目树选择文档，或创建新的卷、章节和场景。"
            icon="i-lucide-file-plus-2"
            title="请选择写作文档"
        />
      </main>

      <aside
          v-if="!controller.focusMode.value && rightSideSlotWindows.length"
          :style="sideColumnStyle('right')"
          class="writing-tool-column is-right"
      >
        <template v-for="(slot, index) in rightSideSlotWindows" :key="`right-${slot.position}`">
          <div
              :class="['writing-tool-slot', `at-${slot.position}`, { 'is-single': rightSideSlotWindows.length === 1, 'is-split': rightSideSlotWindows.length > 1 }]"
              :style="sideSlotStyle('right', index, rightSideSlotWindows.length)"
          >
            <WorkspaceToolWindow
                :icon="toolIcon(slot.window.id)"
                :state="slot.window"
                :title="toolTitle(slot.window.id)"
                dock="side"
                @close="layoutStore.close"
                @resize="layoutStore.setSize"
                @set-slot="layoutStore.setSlot"
            >
              <ToolWindowContent :id="slot.window.id"/>
            </WorkspaceToolWindow>
          </div>

          <button
              v-if="index === 0 && rightSideSlotWindows.length > 1"
              :key="`right-splitter-${slot.position}`"
              aria-label="调整右侧工具窗口高度"
              class="writing-tool-splitter is-horizontal"
              type="button"
              @pointerdown="startSideSplitResize('right', $event)"
          />
        </template>
      </aside>

      <div v-if="!controller.focusMode.value && bottomSlotWindows.length" class="writing-bottom-dock">
        <template v-for="(slot, index) in bottomSlotWindows" :key="`bottom-${slot.side}`">
          <div
              :class="['writing-bottom-pane', `is-${slot.side}`, { 'is-single': bottomSlotWindows.length === 1 }]"
              :style="bottomSlotStyle(index, bottomSlotWindows.length)"
          >
            <WorkspaceToolWindow
                :icon="toolIcon(slot.window.id)"
                :state="slot.window"
                :title="toolTitle(slot.window.id)"
                dock="bottom"
                @close="layoutStore.close"
                @resize="layoutStore.setSize"
                @set-slot="layoutStore.setSlot"
            >
              <ToolWindowContent :id="slot.window.id"/>
            </WorkspaceToolWindow>
          </div>

          <button
              v-if="index === 0 && bottomSlotWindows.length > 1"
              :key="`bottom-splitter-${slot.side}`"
              aria-label="调整底部工具窗口宽度"
              class="writing-tool-splitter is-vertical"
              type="button"
              @pointerdown="startBottomSplitResize"
          />
        </template>
      </div>

      <WorkspaceToolStrip
          v-if="!controller.focusMode.value"
          :windows="layoutStore.rightWindows"
          side="right"
          @toggle="layoutStore.toggleVisible"
      />
    </div>
  </section>
</template>

<script lang="ts" setup>
import {defineComponent, h, type PropType} from 'vue'
import DocumentTabs from '~/components/writing/DocumentTabs.vue'
import DocumentExplorer from '~/components/writing/DocumentExplorer.vue'
import EditorBridge from '~/components/writing/EditorBridge.vue'
import WritingInspector from '~/components/writing/WritingInspector.vue'
import WorkspaceToolWindow from '~/components/workspace/WorkspaceToolWindow.vue'
import WorkspaceToolStrip from '~/components/workspace/WorkspaceToolStrip.vue'
import EditorFocusOverlay from '~/components/editor/EditorFocusOverlay.vue'
import NoteWorkspace from '~/components/notes/NoteWorkspace.vue'
import ForeshadowWorkspace from '~/components/foreshadow/ForeshadowWorkspace.vue'
import ProofreadingWorkspace from '~/components/proofreading/ProofreadingWorkspace.vue'
import SearchWorkspace from '~/components/search/SearchWorkspace.vue'
import type {SaveState} from '~/composables/useAutoSave'
import type {ToolWindowId, ToolWindowPosition, ToolWindowSide, ToolWindowState} from '~/stores/workspaceLayout.store'

const controller = useProjectWorkspaceContext()
const tabsStore = useDocumentTabsStore()
const layoutStore = useWorkspaceLayoutStore()

const sidePositions: ToolWindowPosition[] = ['top', 'center']
const bottomSides: ToolWindowSide[] = ['left', 'right']
const documentsById = computed(() => new Map(controller.documentStore.documents.map(document => [document.id, document])))
const hydratedProjectId = ref<string | null>(null)
const sideSplit = reactive<Record<ToolWindowSide, number>>({left: 0.5, right: 0.5})
const bottomSplit = ref(0.5)
const leftSideSlotWindows = computed(() => visibleSideSlots('left'))
const rightSideSlotWindows = computed(() => visibleSideSlots('right'))
const bottomSlotWindows = computed(() => bottomSides
    .map(side => ({side, window: layoutStore.activeWindowForSlot(side, 'bottom')}))
    .filter((slot): slot is { side: ToolWindowSide, window: ToolWindowState } => Boolean(slot.window))
)
const stageVars = computed(() => ({
  '--writing-left-tool-size': `${layoutStore.activeLeftSize}px`,
  '--writing-right-tool-size': `${layoutStore.activeRightSize}px`,
  '--writing-bottom-tool-size': `${layoutStore.activeBottomSize}px`
}))

watch(
    [
      () => controller.projectId.value,
      () => controller.documentStore.documents,
      () => controller.documentStore.activeDocumentId
    ],
    ([projectId, documents, activeDocumentId]) => {
      if (!projectId) return
      const isFirstHydrateForProject = hydratedProjectId.value !== projectId
      tabsStore.hydrate(projectId, documents, activeDocumentId)
      layoutStore.hydrate(projectId)
      hydratedProjectId.value = projectId
      if (isFirstHydrateForProject && tabsStore.activeTabId && tabsStore.activeTabId !== activeDocumentId) {
        controller.selectDocument(tabsStore.activeTabId)
      }
    },
    {immediate: true, deep: true}
)

watch(
    () => controller.documentStore.activeDocumentId,
    documentId => {
      if (!documentId) return
      const document = documentsById.value.get(documentId)
      if (document) tabsStore.openPreview(document)
    }
)

onMounted(() => {
  window.addEventListener('keydown', handleWorkspaceKeydown)
})

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleWorkspaceKeydown)
})

function visibleSideSlots(side: ToolWindowSide) {
  return sidePositions
      .map(position => ({position, window: layoutStore.activeWindowForSlot(side, position)}))
      .filter((slot): slot is { position: ToolWindowPosition, window: ToolWindowState } => Boolean(slot.window))
}

function sideColumnStyle(side: ToolWindowSide) {
  return {
    '--writing-side-split': String(sideSplit[side])
  }
}

function sideSlotStyle(side: ToolWindowSide, index: number, total: number) {
  if (total <= 1) return undefined
  if (index === 0) {
    const percent = Math.round(sideSplit[side] * 1000) / 10
    return {flex: `0 0 calc(${percent}% - 0.1875rem)`}
  }
  return {flex: '1 1 0'}
}

function bottomSlotStyle(index: number, total: number) {
  if (total <= 1) return undefined
  if (index === 0) {
    const percent = Math.round(bottomSplit.value * 1000) / 10
    return {flex: `0 0 calc(${percent}% - 0.1875rem)`}
  }
  return {flex: '1 1 0'}
}

function startSideSplitResize(side: ToolWindowSide, event: PointerEvent) {
  event.preventDefault()
  const container = event.currentTarget instanceof HTMLElement
      ? event.currentTarget.parentElement
      : null
  if (!container) return
  const rect = container.getBoundingClientRect()

  const move = (nextEvent: PointerEvent) => {
    const raw = (nextEvent.clientY - rect.top) / Math.max(1, rect.height)
    sideSplit[side] = clamp(raw, 0.26, 0.74)
  }

  startSplitDrag('row-resize', move)
}

function startBottomSplitResize(event: PointerEvent) {
  event.preventDefault()
  const container = event.currentTarget instanceof HTMLElement
      ? event.currentTarget.parentElement
      : null
  if (!container) return
  const rect = container.getBoundingClientRect()

  const move = (nextEvent: PointerEvent) => {
    const raw = (nextEvent.clientX - rect.left) / Math.max(1, rect.width)
    bottomSplit.value = clamp(raw, 0.28, 0.72)
  }

  startSplitDrag('col-resize', move)
}

function startSplitDrag(cursor: string, move: (event: PointerEvent) => void) {
  const stop = () => {
    window.removeEventListener('pointermove', move)
    window.removeEventListener('pointerup', stop)
    document.body.style.removeProperty('cursor')
    document.body.style.removeProperty('user-select')
  }

  document.body.style.cursor = cursor
  document.body.style.userSelect = 'none'
  window.addEventListener('pointermove', move)
  window.addEventListener('pointerup', stop, {once: true})
}

function clamp(value: number, min: number, max: number) {
  return Math.min(max, Math.max(min, value))
}

function handleEditorStatus(payload: {
  saveState: SaveState
  characterCount: number
  lastSavedAt: number | null
  errorMessage: string | null
  sessionElapsedMs: number
  sessionDelta: number
}) {
  controller.handleEditorStatus(payload)
  if (controller.documentStore.activeDocumentId) {
    tabsStore.markDirty(controller.documentStore.activeDocumentId, payload.saveState === 'dirty' || payload.saveState === 'saving')
  }
}

function openPreview(documentId: string) {
  const document = documentsById.value.get(documentId)
  if (document) tabsStore.openPreview(document)
  controller.selectDocument(documentId)
}

function openPinned(documentId: string) {
  const document = documentsById.value.get(documentId)
  if (document) tabsStore.openPinned(document)
  controller.selectDocument(documentId)
}

function activateTab(documentId: string) {
  tabsStore.activate(documentId)
  controller.selectDocument(documentId)
}

function closeTab(documentId: string) {
  const nextId = tabsStore.close(documentId)
  if (nextId) controller.selectDocument(nextId)
  else controller.documentStore.activeDocumentId = null
}

function closeAllTabs() {
  tabsStore.closeAll()
  controller.documentStore.activeDocumentId = null
}

function handleWorkspaceKeydown(event: KeyboardEvent) {
  const isMod = event.ctrlKey || event.metaKey
  if (!isMod || controller.workspaceMode.value !== 'writing') return

  if (event.key.toLowerCase() === 'w') {
    event.preventDefault()
    if (controller.documentStore.activeDocumentId) closeTab(controller.documentStore.activeDocumentId)
    return
  }

  if (event.key === 'Tab') {
    event.preventDefault()
    const nextId = tabsStore.activateRecent(event.shiftKey ? -1 : 1)
    if (nextId) controller.selectDocument(nextId)
  }
}

function toolTitle(id: ToolWindowId) {
  switch (id) {
    case 'project':
      return '项目树'
    case 'inspector':
      return '章节信息'
    case 'notes':
      return '事项'
    case 'foreshadow':
      return '伏笔'
    case 'proofreading':
      return '校对结果'
    case 'references':
      return '设定引用'
    case 'stats':
      return '写作统计'
    case 'search':
      return '项目搜索'
  }
}

function toolIcon(id: ToolWindowId) {
  switch (id) {
    case 'project':
      return 'i-lucide-list-tree'
    case 'inspector':
      return 'i-lucide-panel-right'
    case 'notes':
      return 'i-lucide-list-checks'
    case 'foreshadow':
      return 'i-lucide-sparkles'
    case 'proofreading':
      return 'i-lucide-spell-check-2'
    case 'references':
      return 'i-lucide-at-sign'
    case 'stats':
      return 'i-lucide-bar-chart-3'
    case 'search':
      return 'i-lucide-search'
  }
}

const ToolWindowContent = defineComponent({
  props: {
    id: {
      type: String as PropType<ToolWindowId>,
      required: true
    }
  },
  setup(props) {
    return () => {
      if (props.id === 'project') {
        return h(DocumentExplorer, {
          items: controller.documentStore.documentTree,
          activeDocumentId: controller.documentStore.activeDocumentId,
          onOpenPreview: openPreview,
          onOpenPinned: openPinned,
          onCreateDocument: controller.createDocument,
          onRenameDocument: controller.renameDocument,
          onDeleteDocument: controller.deleteDocument,
          onMoveDocument: controller.moveDocument,
          onUpdateStatus: controller.updateDocumentStatus
        })
      }

      if (props.id === 'inspector') return h(WritingInspector)

      if (props.id === 'notes') {
        return h(NoteWorkspace, {
          projectId: controller.projectId.value,
          activeDocumentId: controller.documentStore.activeDocumentId,
          documents: controller.documentStore.documents
        })
      }

      if (props.id === 'foreshadow') {
        return h(ForeshadowWorkspace, {
          projectId: controller.projectId.value,
          documents: controller.documentStore.documents,
          onOpenTarget: controller.openTarget
        })
      }

      if (props.id === 'proofreading') {
        return h(ProofreadingWorkspace, {
          projectId: controller.projectId.value,
          activeDocumentId: controller.documentStore.activeDocumentId,
          documents: controller.documentStore.documents,
          onOpenTarget: controller.openTarget
        })
      }

      if (props.id === 'search') {
        return h(SearchWorkspace, {
          projectId: controller.projectId.value,
          onOpenTarget: controller.openTarget
        })
      }

      if (props.id === 'stats') {
        return h('div', {class: 'writing-tool-placeholder'}, [
          h('strong', '当前章节'),
          h('span', `${controller.editorSnapshot.characterCount} 字`),
          h('span', `今日 +${controller.timerStore.todayCharacterCount} 字`),
          h('span', `项目 ${controller.projectStats.value?.characterCount ?? 0} 字`)
        ])
      }

      return h('div', {class: 'writing-tool-placeholder'}, [
        h('strong', '设定引用'),
        h('span', '在正文中输入 @ 可插入人物、地点或概念引用。')
      ])
    }
  }
})
</script>
