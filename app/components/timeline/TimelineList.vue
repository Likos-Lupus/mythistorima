<template>
  <section class="timeline-list-panel paper-card">
    <header class="timeline-panel-header">
      <div>
        <p class="timeline-panel-kicker">timeline_events</p>
        <h2>事件列表</h2>
      </div>
      <button class="primary-button" type="button" @click="$emit('create')">新建事件</button>
    </header>

    <div v-if="loading" class="empty-panel">正在加载时间线…</div>
    <div v-else-if="events.length === 0" class="empty-panel">
      暂无时间线事件。可以先创建一个关键事件，再绑定地点、章节和参与人物。
    </div>

    <div v-else class="timeline-list">
      <article
          v-for="event in events"
          :key="event.id"
          :class="['timeline-list-item', { 'is-active': event.id === activeEventId }]"
          role="button"
          tabindex="0"
          @click="$emit('select', event.id)"
          @keydown.enter.prevent="$emit('select', event.id)"
      >
        <span class="timeline-list-date">{{ timelineDateLabel(event) }}</span>
        <strong>{{ event.title }}</strong>
        <small>
          #{{ event.sortKey }}
          <template v-if="locationName(event.locationCardId)"> · {{ locationName(event.locationCardId) }}</template>
          <template v-if="participantNames(event.id)"> · {{ participantNames(event.id) }}</template>
        </small>
        <button
            v-if="event.linkedDocumentId"
            class="tree-action-button"
            type="button"
            @click.stop="$emit('open-document', event.linkedDocumentId)"
        >
          跳转章节
        </button>
      </article>
    </div>
  </section>
</template>

<script lang="ts" setup>
import type {SettingCard} from '~/types/card'
import type {TimelineEvent, TimelineEventCard} from '~/types/timeline'
import {timelineDateLabel} from '~/types/timeline'

const props = defineProps<{
  events: TimelineEvent[]
  cards: SettingCard[]
  eventCards: TimelineEventCard[]
  activeEventId?: string | null
  loading?: boolean
}>()

defineEmits<{
  select: [timelineEventId: string]
  create: []
  'open-document': [documentId: string]
}>()

const cardsById = computed(() => new Map(props.cards.map(card => [card.id, card])))
const eventCardMap = computed(() => {
  const result = new Map<string, TimelineEventCard[]>()
  for (const item of props.eventCards) {
    const list = result.get(item.timelineEventId) ?? []
    list.push(item)
    result.set(item.timelineEventId, list)
  }
  return result
})

function locationName(cardId?: string | null) {
  return cardId ? cardsById.value.get(cardId)?.name ?? '' : ''
}

function participantNames(eventId: string) {
  return (eventCardMap.value.get(eventId) ?? [])
      .map(item => cardsById.value.get(item.cardId)?.name)
      .filter(Boolean)
      .slice(0, 3)
      .join('、')
}
</script>
