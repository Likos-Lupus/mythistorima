<template>
  <section class="timeline-lane-panel paper-card">
    <header class="timeline-panel-header">
      <div>
        <p class="timeline-panel-kicker">Story Lane</p>
        <h2>故事顺序</h2>
      </div>
      <p class="timeline-lane-help">按 sort_key 从小到大显示。可用上下按钮快速调整相邻事件顺序。</p>
    </header>

    <div v-if="events.length === 0" class="timeline-lane-empty">
      创建事件后，这里会生成长篇故事的顺序轨道。
    </div>

    <ol v-else class="timeline-lane">
      <li
          v-for="(event, index) in events"
          :key="event.id"
          :class="['timeline-lane-item', { 'is-active': event.id === activeEventId }]"
          @click="$emit('select', event.id)"
      >
        <span class="timeline-lane-dot">{{ index + 1 }}</span>
        <div class="timeline-lane-card">
          <div class="timeline-lane-card-head">
            <div>
              <span>{{ timelineDateLabel(event) }}</span>
              <strong>{{ event.title }}</strong>
            </div>
            <div class="timeline-lane-actions">
              <button :disabled="index === 0" class="tree-action-button" type="button"
                      @click.stop="move(event.id, index, index - 1)">↑
              </button>
              <button :disabled="index === events.length - 1" class="tree-action-button" type="button"
                      @click.stop="move(event.id, index, index + 1)">↓
              </button>
            </div>
          </div>
          <p v-if="event.description">{{ event.description }}</p>
          <div class="timeline-chip-row">
            <button
                v-if="event.linkedDocumentId"
                class="timeline-chip"
                type="button"
                @click.stop="$emit('open-document', event.linkedDocumentId)"
            >
              章节：{{ documentTitle(event.linkedDocumentId) }}
            </button>
            <span v-if="locationName(event.locationCardId)"
                  class="timeline-chip">地点：{{ locationName(event.locationCardId) }}</span>
            <span v-for="participant in participantCards(event.id)" :key="participant.id" class="timeline-chip">
              {{ participant.name }}
            </span>
          </div>
        </div>
      </li>
    </ol>
  </section>
</template>

<script lang="ts" setup>
import type {SettingCard} from '~/types/card'
import type {NovelDocument} from '~/types/document'
import type {TimelineEvent, TimelineEventCard} from '~/types/timeline'
import {timelineDateLabel} from '~/types/timeline'

const props = defineProps<{
  events: TimelineEvent[]
  eventCards: TimelineEventCard[]
  cards: SettingCard[]
  documents: NovelDocument[]
  activeEventId?: string | null
}>()

const emit = defineEmits<{
  select: [timelineEventId: string]
  reorder: [timelineEventId: string, sortKey: number]
  'open-document': [documentId: string]
}>()

const cardsById = computed(() => new Map(props.cards.map(card => [card.id, card])))
const documentsById = computed(() => new Map(props.documents.map(document => [document.id, document])))
const eventCardMap = computed(() => {
  const result = new Map<string, TimelineEventCard[]>()
  for (const item of props.eventCards) {
    const list = result.get(item.timelineEventId) ?? []
    list.push(item)
    result.set(item.timelineEventId, list)
  }
  return result
})

function move(timelineEventId: string, currentIndex: number, targetIndex: number) {
  const target = props.events[targetIndex]
  if (!target) return
  const direction = targetIndex > currentIndex ? 1 : -1
  emit('reorder', timelineEventId, target.sortKey + direction)
}

function locationName(cardId?: string | null) {
  return cardId ? cardsById.value.get(cardId)?.name ?? '' : ''
}

function documentTitle(documentId: string) {
  return documentsById.value.get(documentId)?.title ?? '未知章节'
}

function participantCards(eventId: string) {
  return (eventCardMap.value.get(eventId) ?? [])
      .map(item => cardsById.value.get(item.cardId))
      .filter(Boolean) as SettingCard[]
}
</script>
