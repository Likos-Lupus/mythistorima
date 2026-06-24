<template>
  <div class="workspace-context-panel">
    <header class="workspace-context-header">
      <p class="eyebrow">Context</p>
      <h2>{{ title }}</h2>
      <p>{{ subtitle }}</p>
    </header>

    <template v-if="workspace === 'timeline'">
      <ContextEmpty v-if="!timelineStore.activeEvent" title="未选择时间线事件"/>
      <template v-else>
        <ContextField :value="timelineStore.activeEvent.title" label="事件"/>
        <ContextField :value="timelineDateLabel(timelineStore.activeEvent)" label="时间"/>
        <ContextField :value="timelineStore.activeEvent.description || '暂无说明'" label="说明"/>
        <button
            v-if="timelineStore.activeEvent.linkedDocumentId"
            class="secondary-button full-width-button"
            type="button"
            @click="openTimelineDocument"
        >
          打开关联章节
        </button>
        <button
            v-if="timelineStore.activeEvent.locationCardId"
            class="secondary-button full-width-button"
            type="button"
            @click="openTimelineLocation"
        >
          打开事件地点
        </button>
      </template>
    </template>

    <template v-else-if="workspace === 'relations'">
      <ContextEmpty v-if="!relationStore.activeRelation" title="未选择关系"/>
      <template v-else>
        <ContextField :value="relationTypeLabel(relationStore.activeRelation.relationType)" label="关系"/>
        <ContextField :value="relationDirectionLabel(relationStore.activeRelation.direction)" label="方向"/>
        <ContextField :value="relationStore.activeRelation.description || '暂无说明'" label="说明"/>
        <button class="secondary-button full-width-button" type="button"
                @click="openRelationCard('source')">
          打开来源设定
        </button>
        <button class="secondary-button full-width-button" type="button"
                @click="openRelationCard('target')">
          打开目标设定
        </button>
      </template>
    </template>

    <template v-else-if="workspace === 'foreshadow'">
      <ContextEmpty v-if="!foreshadowStore.activeThread" title="未选择伏笔线程"/>
      <template v-else>
        <ContextField :value="foreshadowStore.activeThread.title" label="伏笔"/>
        <ContextField :value="foreshadowStatusLabel(foreshadowStore.activeThread.status)" label="状态"/>
        <ContextField :value="foreshadowPriorityLabel(foreshadowStore.activeThread.priority)" label="优先级"/>
        <button
            v-if="foreshadowStore.activeThread.setupDocumentId"
            class="secondary-button full-width-button"
            type="button"
            @click="openForeshadowLocation('setup')"
        >
          定位提出位置
        </button>
        <button
            v-if="foreshadowStore.activeThread.payoffDocumentId"
            class="secondary-button full-width-button"
            type="button"
            @click="openForeshadowLocation('payoff')"
        >
          定位回收位置
        </button>
      </template>
    </template>

    <template v-else-if="workspace === 'proofreading'">
      <template v-if="activeIssue">
        <ContextField :value="activeIssue.message" label="问题"/>
        <ContextField :value="proofreadingSeverityLabel(activeIssue.severity)" label="严重程度"/>
        <ContextField :value="activeIssue.suggestion || '暂无建议'" label="建议"/>
        <button class="primary-button full-width-button" type="button"
                @click="emitTarget({type: 'proofreadingIssue', issue: activeIssue})">
          定位正文
        </button>
      </template>
      <template v-else-if="proofreadingStore.activeRule">
        <ContextField :value="proofreadingStore.activeRule.name" label="规则"/>
        <ContextField :value="proofreadingRuleTypeLabel(proofreadingStore.activeRule.type)" label="类型"/>
        <ContextField :value="`${proofreadingStore.issues.length}`" label="问题数"/>
      </template>
      <ContextEmpty v-else title="未选择校对规则或问题"/>
    </template>

    <template v-else-if="workspace === 'outline' || workspace === 'board'">
      <ContextEmpty v-if="!outlineStore.activeNode" title="未选择大纲节点"/>
      <template v-else>
        <ContextField :value="outlineStore.activeNode.title" label="节点"/>
        <ContextField :value="outlineStore.activeNode.status" label="状态"/>
        <ContextField :value="outlineStore.activeNode.summary || '暂无摘要'" label="摘要"/>
        <button
            v-if="outlineStore.activeNode.linkedDocumentId"
            class="secondary-button full-width-button"
            type="button"
            @click="openOutlineDocument"
        >
          打开绑定章节
        </button>
      </template>
    </template>

    <template v-else-if="workspace === 'cards'">
      <ContextEmpty v-if="!cardStore.activeCard" title="未选择设定卡"/>
      <template v-else>
        <ContextField :value="cardStore.activeCard.name" label="设定"/>
        <ContextField :value="cardTypeLabel(cardStore.activeCard.type)" label="类型"/>
        <ContextField :value="cardStore.activeCard.description || '暂无简介'" label="简介"/>
        <ContextField :value="`${cardStore.references.length}`" label="正文引用"/>
      </template>
    </template>

    <template v-else-if="workspace === 'notes'">
      <ContextEmpty v-if="!noteStore.activeNote" title="未选择事项"/>
      <template v-else>
        <ContextField :value="noteStore.activeNote.title" label="事项"/>
        <ContextField :value="noteTypeLabel(noteStore.activeNote.type)" label="类型"/>
        <ContextField :value="noteStore.activeNote.status" label="状态"/>
        <button
            v-if="noteStore.activeNote.documentId"
            class="secondary-button full-width-button"
            type="button"
            @click="openNoteDocument"
        >
          定位正文
        </button>
      </template>
    </template>

    <template v-else-if="workspace === 'search'">
      <ContextField :value="searchStore.query || '尚未搜索'" label="关键词"/>
      <ContextField :value="`${searchStore.results.length}`" label="结果"/>
      <ContextField :value="searchStore.scopes.join('、')" label="范围"/>
    </template>

    <template v-else-if="workspace === 'export'">
      <ContextEmpty v-if="!exportTemplateStore.activeTemplate" title="未选择导出模板"/>
      <template v-else>
        <ContextField :value="exportTemplateStore.activeTemplate.name" label="模板"/>
        <ContextField :value="exportTemplateFormatLabel(exportTemplateStore.activeTemplate.format)" label="格式"/>
        <ContextField :value="exportTemplateStore.activeTemplate.isBuiltin ? '内置模板' : '项目模板'" label="类型"/>
      </template>
    </template>

    <template v-else>
      <ContextField :value="projectId" label="项目"/>
      <ContextField :value="workspaceLabel" label="当前工作区"/>
      <ContextField :value="activeDocumentTitle || '未选择'" label="文档"/>
      <ContextField :value="`${projectCharacterCount} 字`" label="项目字数"/>
    </template>

    <p v-if="navigationStore.navigationMessage" class="workspace-context-message">
      {{ navigationStore.navigationMessage }}
    </p>
  </div>
</template>

<script lang="ts" setup>
import {defineComponent, h} from 'vue'
import type {ProjectWorkspaceMode} from '~/constants/projectViews'
import type {OpenTarget} from '~/types/navigation'
import {cardTypeLabel} from '~/types/card'
import {exportTemplateFormatLabel} from '~/utils/exportTemplate'
import {foreshadowPriorityLabel, foreshadowStatusLabel} from '~/types/foreshadow'
import {proofreadingRuleTypeLabel, proofreadingSeverityLabel} from '~/types/proofreading'
import {relationDirectionLabel, relationTypeLabel} from '~/types/relation'
import {timelineDateLabel} from '~/types/timeline'

const props = defineProps<{
  workspace: ProjectWorkspaceMode
  projectId: string
  activeDocumentTitle?: string | null
  projectCharacterCount?: number
}>()

const emit = defineEmits<{
  'open-target': [target: OpenTarget]
}>()

const timelineStore = useTimelineStore()
const relationStore = useRelationStore()
const foreshadowStore = useForeshadowStore()
const proofreadingStore = useProofreadingStore()
const outlineStore = useOutlineStore()
const cardStore = useCardStore()
const noteStore = useNoteStore()
const searchStore = useSearchStore()
const exportTemplateStore = useExportTemplateStore()
const navigationStore = useNavigationStore()

const activeIssue = computed(() =>
    proofreadingStore.issues.find(issue => issue.id === proofreadingStore.activeIssueId) ?? null
)

const workspaceLabel = computed(() => {
  const labels: Partial<Record<ProjectWorkspaceMode, string>> = {
    dashboard: '概览',
    outline: '大纲',
    board: '看板',
    timeline: '时间线',
    cards: '设定',
    relations: '关系图',
    notes: '事项',
    foreshadow: '伏笔',
    proofreading: '校对',
    search: '搜索',
    export: '导出',
  }
  return labels[props.workspace] ?? '工作台'
})

const title = computed(() => `${workspaceLabel.value}上下文`)
const subtitle = computed(() => '显示当前选择及可继续执行的跳转。')

const ContextField = defineComponent({
  props: {
    label: {type: String, required: true},
    value: {type: String, required: true}
  },
  setup(fieldProps) {
    return () => h('div', {class: 'workspace-context-field'}, [
      h('span', fieldProps.label),
      h('strong', fieldProps.value)
    ])
  }
})

const ContextEmpty = defineComponent({
  props: {
    title: {type: String, required: true}
  },
  setup(emptyProps) {
    return () => h('div', {class: 'workspace-context-empty'}, emptyProps.title)
  }
})


function openTimelineDocument() {
  const event = timelineStore.activeEvent
  if (!event?.linkedDocumentId) return
  emitTarget({type: 'document', targetId: event.linkedDocumentId, source: 'timeline', label: event.title})
}

function openTimelineLocation() {
  const event = timelineStore.activeEvent
  if (!event?.locationCardId) return
  emitTarget({type: 'card', targetId: event.locationCardId})
}

function openRelationCard(side: 'source' | 'target') {
  const relation = relationStore.activeRelation
  if (!relation) return
  emitTarget({
    type: 'card',
    targetId: side === 'source' ? relation.sourceCardId : relation.targetCardId
  })
}

function openOutlineDocument() {
  const node = outlineStore.activeNode
  if (!node?.linkedDocumentId) return
  emitTarget({type: 'document', targetId: node.linkedDocumentId, source: 'outline', label: node.title})
}

function openNoteDocument() {
  const note = noteStore.activeNote
  if (!note?.documentId) return
  emitTarget({
    type: 'document',
    targetId: note.documentId,
    paragraphId: note.paragraphId,
    source: 'note',
    label: note.title
  })
}

function emitTarget(target: OpenTarget) {
  emit('open-target', target)
}

function openForeshadowLocation(location: 'setup' | 'payoff') {
  const thread = foreshadowStore.activeThread
  if (!thread) return
  if (location === 'setup' && thread.setupDocumentId) {
    emitTarget({
      type: 'document',
      targetId: thread.setupDocumentId,
      paragraphId: thread.setupParagraphId,
      source: 'foreshadow-setup',
      label: '伏笔提出位置'
    })
  }
  if (location === 'payoff' && thread.payoffDocumentId) {
    emitTarget({
      type: 'document',
      targetId: thread.payoffDocumentId,
      paragraphId: thread.payoffParagraphId,
      source: 'foreshadow-payoff',
      label: '伏笔回收位置'
    })
  }
}

function noteTypeLabel(type: string) {
  if (type === 'todo') return '待办'
  if (type === 'foreshadow') return '伏笔'
  if (type === 'idea') return '灵感'
  if (type === 'issue') return '问题'
  return '备忘'
}
</script>
