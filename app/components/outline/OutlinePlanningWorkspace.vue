<template>
  <section class="story-workspace outline-suite-workspace">
    <UAlert
        v-if="error"
        :description="error"
        class="story-alert"
        color="error"
        icon="i-lucide-circle-alert"
        title="大纲操作失败"
        variant="subtle"
        @close="error = null"
    />

    <div v-if="activeTab === 'board'" class="story-master-detail outline-board-shell">
      <aside class="story-side-panel outline-node-master">
        <div class="story-panel-header">
          <div>
            <strong>节点</strong>
            <span>{{ filteredFlatOutline.length }} 个</span>
          </div>
          <div class="story-panel-actions">
            <UTooltip text="刷新大纲">
              <UButton color="neutral" icon="i-lucide-refresh-cw" size="xs" variant="ghost" @click="reloadAll"/>
            </UTooltip>
            <UDropdownMenu :items="createMenuItems">
              <UButton color="neutral" icon="i-lucide-plus" size="xs" variant="ghost"/>
            </UDropdownMenu>
          </div>
        </div>
        <UInput v-model="outlineQuery" icon="i-lucide-search" placeholder="搜索大纲节点" size="sm"/>
        <USelect
            v-model="statusFilter"
            :items="statusOptions"
            class="w-full"
            label-key="label"
            size="sm"
            value-key="value"
        />
        <div class="story-list-scroll">
          <UButton
              v-for="node in filteredFlatOutline"
              :key="node.id"
              :aria-pressed="node.id === outlineStore.activeOutlineNodeId"
              :class="['story-list-item', {'is-active': node.id === outlineStore.activeOutlineNodeId}]"
              color="neutral"
              type="button"
              variant="ghost"
              @click="outlineStore.selectOutlineNode(node.id)"
          >
            <span :style="{paddingInlineStart: `${Math.min(node.depth, 5) * 0.75}rem`}" class="story-list-title">
              <span :class="['story-status-dot', `is-${node.status}`]"/>
              {{ node.title }}
            </span>
            <small>{{ outlineTypeLabel(node.type) }} · {{ outlineStatusLabel(node.status) }}</small>
          </UButton>
          <UEmpty
              v-if="!filteredFlatOutline.length"
              description="创建剧情、冲突、转折、支线或主题节点后，会在这里形成结构树。"
              icon="i-lucide-list-tree"
              title="暂无大纲节点"
          />
        </div>
      </aside>

      <main class="story-main-panel outline-board-main">
        <div class="story-panel-toolbar">
          <UInput v-model="boardSearch" class="min-w-64" icon="i-lucide-search" placeholder="筛选看板卡片" size="sm"/>
          <UButton color="neutral" icon="i-lucide-refresh-cw" label="刷新" size="sm" variant="ghost" @click="reloadAll"/>
          <UButtonGroup size="sm">
            <UButton
                v-for="option in boardQuickFilters"
                :key="option.value"
                :color="boardStatus === option.value ? 'primary' : 'neutral'"
                :label="option.label"
                :variant="boardStatus === option.value ? 'soft' : 'ghost'"
                @click="boardStatus = option.value"
            />
          </UButtonGroup>
        </div>
        <div class="outline-kanban-scroll">
          <section
              v-for="column in boardColumns"
              :key="column.status"
              class="outline-kanban-column"
              @dragover.prevent
              @drop.prevent="dropOnColumn($event, column.status)"
          >
            <header>
              <div>
                <strong>{{ column.label }}</strong>
                <span>{{ column.description }}</span>
              </div>
              <UBadge color="neutral" size="sm" variant="soft">{{ boardItemsFor(column.status).length }}</UBadge>
            </header>
            <article
                v-for="node in boardItemsFor(column.status)"
                :key="node.id"
                :class="['outline-kanban-card', {'is-active': node.id === outlineStore.activeOutlineNodeId}]"
                draggable="true"
                @click="outlineStore.selectOutlineNode(node.id)"
                @dragstart="dragOutlineNode($event, node.id)"
            >
              <div>
                <UBadge color="neutral" size="sm" variant="subtle">{{ outlineTypeLabel(node.type) }}</UBadge>
                <strong>{{ node.title }}</strong>
              </div>
              <p>{{ node.summary || '暂无摘要。' }}</p>
              <footer>
                <span v-if="node.linkedDocument">{{
                    documentTypeLabel(node.linkedDocument.type)
                  }} · {{ node.linkedDocument.title }}</span>
                <span v-else>未绑定章节</span>
                <UButton v-if="node.linkedDocumentId" color="neutral" icon="i-lucide-file-pen-line" size="xs"
                         variant="ghost" @click.stop="openDocument(node.linkedDocumentId)"/>
              </footer>
            </article>
            <UEmpty
                v-if="!boardItemsFor(column.status).length"
                class="outline-column-empty"
                description="拖入节点即可更新状态。"
                icon="i-lucide-move-horizontal"
                title="暂无卡片"
            />
          </section>
        </div>
      </main>

      <aside class="story-inspector-panel">
        <OutlineNodeInspector
            :documents="documents"
            :node="outlineStore.activeNode"
            :saving="outlineStore.saving"
            @delete="deleteNode"
            @save="saveOutlineNode"
            @create-child="createChild"
            @open-document="openDocument"
        />
      </aside>
    </div>

    <div v-else-if="activeTab === 'mind'" class="story-canvas-mode">
      <aside class="story-side-panel">
        <div class="story-panel-header">
          <div>
            <strong>图谱筛选</strong>
            <span>{{ graphNodes.length }} 个节点</span>
          </div>
        </div>
        <UInput v-model="outlineQuery" icon="i-lucide-search" placeholder="搜索节点" size="sm"/>
        <USelect v-model="statusFilter" :items="statusOptions" class="w-full" label-key="label" size="sm"
                 value-key="value"/>
        <UAlert color="neutral" description="图谱画布本身为自定义 SVG；缩放、筛选、复制等外围控件使用 Nuxt UI。"
                icon="i-lucide-info" title="画布模式" variant="subtle"/>
      </aside>
      <main class="story-main-panel outline-mind-main">
        <div class="story-panel-toolbar">
          <UButtonGroup size="sm">
            <UButton color="neutral" icon="i-lucide-zoom-out" label="缩小" variant="ghost"
                     @click="zoom = Math.max(0.6, zoom - 0.1)"/>
            <UButton color="neutral" label="适应窗口" variant="ghost" @click="zoom = 1"/>
            <UButton color="neutral" icon="i-lucide-zoom-in" label="放大" variant="ghost"
                     @click="zoom = Math.min(1.6, zoom + 0.1)"/>
          </UButtonGroup>
          <UButton color="neutral" icon="i-lucide-refresh-cw" label="刷新" size="sm" variant="ghost" @click="reloadAll"/>
          <UButton color="neutral" icon="i-lucide-copy" label="复制 Mermaid" size="sm" variant="ghost"
                   @click="copyMermaid"/>
        </div>
        <OutlineMindMapView
            :active-outline-node-id="outlineStore.activeOutlineNodeId"
            :documents="documents"
            :items="graphNodes"
            :zoom="zoom"
            @select="outlineStore.selectOutlineNode"
            @open-document="openDocument"
        />
      </main>
      <aside class="story-inspector-panel">
        <OutlineNodeInspector
            :documents="documents"
            :node="outlineStore.activeNode"
            :saving="outlineStore.saving"
            @delete="deleteNode"
            @save="saveOutlineNode"
            @create-child="createChild"
            @open-document="openDocument"
        />
      </aside>
    </div>

    <div v-else class="story-master-detail timeline-suite-layout">
      <aside class="story-side-panel">
        <div class="story-panel-header">
          <div>
            <strong>事件</strong>
            <span>{{ timelineStore.events.length }} 个</span>
          </div>
          <UButton icon="i-lucide-plus" size="xs" variant="ghost" @click="startTimelineCreate"/>
        </div>
        <UInput v-model="timelineQuery" icon="i-lucide-search" placeholder="搜索时间线事件" size="sm"/>
        <USelect v-model="timelineCardFilter" :items="timelineCardItems" class="w-full" label-key="label" size="sm"
                 value-key="value" @update:model-value="applyTimelineFilters"/>
        <div class="story-list-scroll">
          <UButton
              v-for="event in filteredTimelineEvents"
              :key="event.id"
              :aria-pressed="event.id === timelineStore.activeEventId"
              :class="['story-list-item', {'is-active': event.id === timelineStore.activeEventId}]"
              color="neutral"
              type="button"
              variant="ghost"
              @click="selectTimelineEvent(event.id)"
          >
            <span class="story-list-title">{{ event.title }}</span>
            <small>{{ timelineDateLabel(event) }}</small>
          </UButton>
          <UEmpty v-if="!filteredTimelineEvents.length" description="新建事件后，故事会按 sort key 排列成时间线。"
                  icon="i-lucide-clock-3" title="暂无事件"/>
        </div>
      </aside>
      <main class="story-main-panel timeline-canvas-panel">
        <div class="story-panel-toolbar">
          <UButton color="neutral" icon="i-lucide-plus" label="新建事件" size="sm" variant="ghost"
                   @click="startTimelineCreate"/>
          <UButton color="neutral" icon="i-lucide-refresh-cw" label="刷新" size="sm" variant="ghost"
                   @click="reloadTimeline"/>
        </div>
        <ol class="story-timeline-lane">
          <li
              v-for="(event, index) in filteredTimelineEvents"
              :key="event.id"
              :class="{'is-active': event.id === timelineStore.activeEventId}"
              @click="selectTimelineEvent(event.id)"
          >
            <span>{{ index + 1 }}</span>
            <article>
              <div>
                <small>{{ timelineDateLabel(event) }}</small>
                <strong>{{ event.title }}</strong>
              </div>
              <p>{{ event.description || '暂无事件说明。' }}</p>
              <footer>
                <UBadge v-if="documentTitle(event.linkedDocumentId)" color="neutral" size="sm" variant="soft">
                  {{ documentTitle(event.linkedDocumentId) }}
                </UBadge>
                <UBadge v-if="cardName(event.locationCardId)" color="neutral" size="sm" variant="soft">
                  {{ cardName(event.locationCardId) }}
                </UBadge>
              </footer>
            </article>
          </li>
        </ol>
      </main>
      <aside class="story-inspector-panel">
        <TimelineEventInspector
            :cards="cardStore.cards"
            :documents="documents"
            :event="timelineStore.activeEvent"
            :project-id="projectId"
            :saving="timelineStore.saving"
            @create="createTimelineEvent"
            @delete="deleteTimelineEvent"
            @update="updateTimelineEvent"
        />
      </aside>
    </div>
  </section>
</template>

<script lang="ts" setup>
import OutlineMindMapView from '~/components/outline/OutlineMindMapView.vue'
import OutlineNodeInspector from '~/components/outline/OutlineNodeInspector.vue'
import TimelineEventInspector from '~/components/outline/TimelineEventInspector.vue'
import type {NovelDocument} from '~/types/document'
import type {OutlineNodeStatus, OutlineNodeType, UpdateOutlineNodeInput} from '~/types/outline'
import type {CreateTimelineEventInput, UpdateTimelineEventInput} from '~/types/timeline'
import {timelineDateLabel} from '~/types/timeline'
import {toAppErrorMessage} from '~/utils/appError'
import {
  documentTypeLabel,
  flattenOutlineTree,
  generateOutlineMermaid,
  OUTLINE_BOARD_COLUMNS,
  OUTLINE_STATUS_OPTIONS,
  outlineStatusLabel,
  outlineTypeLabel
} from '~/utils/outlinePresentation'
import type {ProjectWorkspaceMode} from '~/constants/projectViews'

const props = defineProps<{
  projectId: string
  documents: NovelDocument[]
  mode: ProjectWorkspaceMode
}>()

const emit = defineEmits<{
  selectMode: [mode: ProjectWorkspaceMode]
  openDocument: [documentId: string]
}>()

const outlineStore = useOutlineStore()
const timelineStore = useTimelineStore()
const cardStore = useCardStore()
const error = ref<string | null>(null)
const activeTab = ref<'board' | 'mind' | 'timeline'>(modeToTab(props.mode))
const outlineQuery = ref('')
const boardSearch = ref('')
const boardStatus = ref<OutlineNodeStatus | 'all'>('all')
const statusFilter = computed({
  get: () => outlineStore.statusFilter,
  set: value => {
    outlineStore.statusFilter = value
  }
})
const timelineQuery = ref('')
const timelineCardFilter = ref('all')
const zoom = ref(1)

const statusOptions = OUTLINE_STATUS_OPTIONS.map(option => ({label: option.label, value: option.value}))
const boardQuickFilters = statusOptions.filter(option => option.value !== 'archived')
const boardColumns = OUTLINE_BOARD_COLUMNS
const createMenuItems = [[
  {label: '剧情', icon: 'i-lucide-waypoints', onSelect: () => createRoot('plot')},
  {label: '冲突', icon: 'i-lucide-zap', onSelect: () => createRoot('conflict')},
  {label: '转折', icon: 'i-lucide-corner-down-right', onSelect: () => createRoot('twist')},
  {label: '支线', icon: 'i-lucide-git-branch', onSelect: () => createRoot('arc')},
  {label: '主题', icon: 'i-lucide-badge-check', onSelect: () => createRoot('theme')}
]]

const outlineTree = computed(() => outlineStore.treeWithDocuments(props.documents))
const flatOutline = computed(() => flattenOutlineTree(outlineTree.value))
const filteredFlatOutline = computed(() => filterOutline(flatOutline.value, outlineQuery.value))
const graphNodes = computed(() => filterOutline(flatOutline.value, outlineQuery.value).filter(node => node.status !== 'archived'))
const boardNodes = computed(() => {
  const query = boardSearch.value.trim().toLowerCase()
  return flatOutline.value.filter(node => {
    if (node.status === 'archived') return false
    if (boardStatus.value !== 'all' && node.status !== boardStatus.value) return false
    if (!query) return true
    return [node.title, node.summary, node.type].some(value => String(value).toLowerCase().includes(query))
  })
})

const timelineCardItems = computed(() => [
  {label: '全部设定', value: 'all'},
  ...cardStore.cards.map(card => ({label: card.name, value: card.id}))
])
const filteredTimelineEvents = computed(() => {
  const query = timelineQuery.value.trim().toLowerCase()
  if (!query) return timelineStore.events
  return timelineStore.events.filter(event => [event.title, event.description, event.startsAtLabel, event.endsAtLabel]
      .some(value => String(value ?? '').toLowerCase().includes(query)))
})

watch(() => props.mode, mode => {
  activeTab.value = modeToTab(mode)
})

watch(activeTab, tab => {
  const next = tabToMode(tab)
  if (props.mode !== next) emit('selectMode', next)
  if (tab === 'timeline') void reloadTimeline()
})

watch(() => props.projectId, () => {
  void reloadAll()
}, {immediate: true})

async function run(action: () => Promise<unknown>, fallback: string) {
  error.value = null
  try {
    await action()
  } catch (err) {
    error.value = toAppErrorMessage(err, fallback)
  }
}

async function reloadAll() {
  await run(async () => {
    await Promise.all([
      outlineStore.loadOutlineNodes(props.projectId),
      cardStore.loadCards(props.projectId, 'all')
    ])
    if (activeTab.value === 'timeline') await timelineStore.loadTimeline(props.projectId)
  }, '加载大纲工作区失败')
}

async function reloadTimeline() {
  await run(async () => {
    await Promise.all([
      cardStore.loadCards(props.projectId, 'all'),
      timelineStore.loadTimeline(props.projectId)
    ])
  }, '加载时间线失败')
}

function modeToTab(mode: ProjectWorkspaceMode): 'board' | 'mind' | 'timeline' {
  if (mode === 'timeline') return 'timeline'
  if (mode === 'outline') return 'mind'
  return 'board'
}

function tabToMode(tab: 'board' | 'mind' | 'timeline'): ProjectWorkspaceMode {
  if (tab === 'timeline') return 'timeline'
  if (tab === 'mind') return 'outline'
  return 'board'
}

function filterOutline(items: ReturnType<typeof flattenOutlineTree>, query: string) {
  const normalized = query.trim().toLowerCase()
  if (!normalized) return items
  return items.filter(node => [node.title, node.summary, node.type, node.status]
      .some(value => String(value).toLowerCase().includes(normalized)))
}

function defaultOutlineTitle(type: OutlineNodeType, parentId?: string | null) {
  const count = outlineStore.nodes.filter(node => (node.parentId ?? null) === (parentId ?? null) && node.type === type).length + 1
  return `${outlineTypeLabel(type)} ${count}`
}

async function createRoot(type: OutlineNodeType) {
  await run(() => outlineStore.createOutlineNode({
    projectId: props.projectId,
    parentId: null,
    type,
    title: defaultOutlineTitle(type),
    status: 'planned'
  }), '创建大纲节点失败')
}

async function createChild(parentId: string) {
  await run(() => outlineStore.createOutlineNode({
    projectId: props.projectId,
    parentId,
    type: 'plot',
    title: defaultOutlineTitle('plot', parentId),
    status: 'planned'
  }), '创建子级大纲失败')
}

async function saveOutlineNode(payload: UpdateOutlineNodeInput & { linkedDocumentId?: string | null }) {
  await run(async () => {
    const current = outlineStore.nodes.find(node => node.id === payload.outlineNodeId)
    await outlineStore.updateOutlineNode(payload)
    if (payload.linkedDocumentId && payload.linkedDocumentId !== current?.linkedDocumentId) {
      await outlineStore.linkOutlineNodeToDocument({
        outlineNodeId: payload.outlineNodeId,
        documentId: payload.linkedDocumentId
      })
    } else if (payload.linkedDocumentId === null && current?.linkedDocumentId) {
      await outlineStore.unlinkOutlineNodeDocument(payload.outlineNodeId)
    }
  }, '保存大纲节点失败')
}

async function deleteNode(outlineNodeId: string) {
  await run(() => outlineStore.deleteOutlineNode(outlineNodeId), '删除大纲节点失败')
}

function dragOutlineNode(event: DragEvent, outlineNodeId: string) {
  event.dataTransfer?.setData('application/x-mythistorima-outline-node', outlineNodeId)
  event.dataTransfer?.setData('text/plain', outlineNodeId)
}

async function dropOnColumn(event: DragEvent, status: OutlineNodeStatus) {
  const id = event.dataTransfer?.getData('application/x-mythistorima-outline-node') || event.dataTransfer?.getData('text/plain')
  if (!id) return
  await run(() => outlineStore.updateOutlineNode({outlineNodeId: id, status}), '更新看板状态失败')
}

function boardItemsFor(status: OutlineNodeStatus) {
  return boardNodes.value.filter(node => node.status === status)
}

function openDocument(documentId: string) {
  emit('openDocument', documentId)
}

async function copyMermaid() {
  const text = generateOutlineMermaid(graphNodes.value, props.documents)
  await navigator.clipboard?.writeText(text)
}

function startTimelineCreate() {
  timelineStore.selectEvent(null)
}

function selectTimelineEvent(id: string) {
  timelineStore.selectEvent(id)
}

async function applyTimelineFilters() {
  timelineStore.setFilters(timelineCardFilter.value === 'all' ? null : timelineCardFilter.value, null)
  await reloadTimeline()
}

async function createTimelineEvent(payload: CreateTimelineEventInput) {
  await run(() => timelineStore.createEvent(payload), '创建时间线事件失败')
}

async function updateTimelineEvent(payload: UpdateTimelineEventInput) {
  await run(() => timelineStore.updateEvent(payload), '保存时间线事件失败')
}

async function deleteTimelineEvent(timelineEventId: string) {
  await run(() => timelineStore.deleteEvent(timelineEventId), '删除时间线事件失败')
}

function documentTitle(documentId?: string | null) {
  if (!documentId) return ''
  return props.documents.find(document => document.id === documentId)?.title ?? ''
}

function cardName(cardId?: string | null) {
  if (!cardId) return ''
  return cardStore.cards.find(card => card.id === cardId)?.name ?? ''
}
</script>
