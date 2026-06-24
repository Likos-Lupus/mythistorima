<template>
  <section class="timeline-workspace" data-phase2-area="timeline-workspace">
    <header class="timeline-workspace-header glass-panel">
      <div>
        <p class="timeline-workspace-kicker">Phase 2 Week 5</p>
        <h1>时间线系统</h1>
        <p>管理长篇故事事件顺序、显示时间、关联章节、地点和参与设定。</p>
      </div>
      <div class="timeline-workspace-actions">
        <button class="secondary-button" type="button" @click="refresh">刷新</button>
        <button class="primary-button" type="button" @click="startCreate">新建事件</button>
      </div>
    </header>

    <TimelineFilters
        :card-id="timelineStore.cardFilterId"
        :cards="cardStore.cards"
        :location-id="timelineStore.locationFilterId"
        @clear="clearFilters"
        @update:card-id="updateCardFilter"
        @update:location-id="updateLocationFilter"
    />

    <div class="timeline-layout">
      <div class="timeline-main-stack">
        <TimelineLaneView
            :active-event-id="timelineStore.activeEventId"
            :cards="cardStore.cards"
            :documents="documents"
            :event-cards="timelineStore.eventCards"
            :events="timelineStore.events"
            @reorder="reorderEvent"
            @select="timelineStore.selectEvent"
            @open-document="$emit('open-target', {type: 'document', targetId: $event, source: 'timeline'})"
        />

        <TimelineList
            :active-event-id="timelineStore.activeEventId"
            :cards="cardStore.cards"
            :event-cards="timelineStore.eventCards"
            :events="timelineStore.events"
            :loading="timelineStore.loading"
            @create="startCreate"
            @select="timelineStore.selectEvent"
            @open-document="$emit('open-target', {type: 'document', targetId: $event, source: 'timeline'})"
        />
      </div>

      <aside class="timeline-side-stack">
        <TimelineEventEditor
            :cards="cardStore.cards"
            :documents="documents"
            :event="timelineStore.activeEvent"
            :project-id="projectId"
            :saving="timelineStore.saving"
            @create="createEvent"
            @delete="deleteEvent"
            @update="updateEvent"
        />

        <TimelineEventCardPicker
            :cards="cardStore.cards"
            :event="timelineStore.activeEvent"
            :event-cards="timelineStore.activeEventCards"
            :saving="timelineStore.saving"
            @attach="attachCard"
            @detach="detachCard"
            @open-card="$emit('open-target', {type: 'card', targetId: $event})"
        />
      </aside>
    </div>

    <ErrorBanner :message="error" title="时间线加载失败" @dismiss="error = null"/>
  </section>
</template>

<script lang="ts" setup>
import TimelineEventCardPicker from '~/components/timeline/TimelineEventCardPicker.vue'
import TimelineEventEditor from '~/components/timeline/TimelineEventEditor.vue'
import TimelineFilters from '~/components/timeline/TimelineFilters.vue'
import TimelineLaneView from '~/components/timeline/TimelineLaneView.vue'
import TimelineList from '~/components/timeline/TimelineList.vue'
import ErrorBanner from '~/components/common/ErrorBanner.vue'
import type {NovelDocument} from '~/types/document'
import type {CreateTimelineEventInput, TimelineParticipantRole, UpdateTimelineEventInput} from '~/types/timeline'
import {toAppErrorMessage} from '~/utils/appError'

const props = defineProps<{
  projectId: string
  documents: NovelDocument[]
}>()

defineEmits<{
  'open-target': [target: import('~/types/navigation').OpenTarget]
}>()

const timelineStore = useTimelineStore()
const cardStore = useCardStore()
const error = ref<string | null>(null)

watch(() => props.projectId, async projectId => {
  if (projectId) await refresh()
}, {immediate: true})

async function refresh() {
  error.value = null
  try {
    await Promise.all([
      cardStore.loadCards(props.projectId, 'all'),
      timelineStore.loadTimeline(props.projectId)
    ])
  } catch (err) {
    error.value = toAppErrorMessage(err, '加载时间线失败')
  }
}

function startCreate() {
  timelineStore.selectEvent(null)
}

async function updateCardFilter(cardId: string | null) {
  timelineStore.cardFilterId = cardId
  await refreshTimelineOnly()
}

async function updateLocationFilter(locationId: string | null) {
  timelineStore.locationFilterId = locationId
  await refreshTimelineOnly()
}

async function clearFilters() {
  timelineStore.setFilters(null, null)
  await refreshTimelineOnly()
}

async function refreshTimelineOnly() {
  error.value = null
  try {
    await timelineStore.loadTimeline(props.projectId)
  } catch (err) {
    error.value = toAppErrorMessage(err, '筛选时间线失败')
  }
}

async function createEvent(payload: CreateTimelineEventInput) {
  error.value = null
  try {
    await timelineStore.createEvent(payload)
  } catch (err) {
    error.value = toAppErrorMessage(err, '创建时间线事件失败')
  }
}

async function updateEvent(payload: UpdateTimelineEventInput) {
  error.value = null
  try {
    await timelineStore.updateEvent(payload)
  } catch (err) {
    error.value = toAppErrorMessage(err, '保存时间线事件失败')
  }
}

async function deleteEvent(timelineEventId: string) {
  error.value = null
  try {
    await timelineStore.deleteEvent(timelineEventId)
  } catch (err) {
    error.value = toAppErrorMessage(err, '删除时间线事件失败')
  }
}

async function attachCard(cardId: string, role: TimelineParticipantRole) {
  if (!timelineStore.activeEvent) return
  error.value = null
  try {
    await timelineStore.attachCard({
      projectId: props.projectId,
      timelineEventId: timelineStore.activeEvent.id,
      cardId,
      role
    })
  } catch (err) {
    error.value = toAppErrorMessage(err, '绑定参与设定失败')
  }
}

async function detachCard(timelineEventId: string, cardId: string) {
  error.value = null
  try {
    await timelineStore.detachCard(timelineEventId, cardId)
  } catch (err) {
    error.value = toAppErrorMessage(err, '移除参与设定失败')
  }
}

async function reorderEvent(timelineEventId: string, sortKey: number) {
  error.value = null
  try {
    await timelineStore.reorderEvent({timelineEventId, sortKey})
  } catch (err) {
    error.value = toAppErrorMessage(err, '调整事件顺序失败')
  }
}
</script>
