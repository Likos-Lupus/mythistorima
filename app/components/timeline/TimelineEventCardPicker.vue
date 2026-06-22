<template>
  <section class="timeline-picker-panel paper-card">
    <header class="timeline-panel-header">
      <div>
        <p class="timeline-panel-kicker">Participants</p>
        <h2>参与设定</h2>
      </div>
    </header>

    <div v-if="!event" class="empty-panel">
      选择或创建一个时间线事件后，可以绑定参与人物、组织、道具或其他设定。
    </div>

    <template v-else>
      <form class="timeline-picker-form" @submit.prevent="attach">
        <label class="card-form-row">
          设定卡
          <select v-model="form.cardId" class="form-field">
            <option value="">选择要绑定的设定</option>
            <option v-for="card in attachableCards" :key="card.id" :value="card.id">
              {{ cardTypeLabel(card.type) }} · {{ card.name }}
            </option>
          </select>
        </label>

        <label class="card-form-row">
          角色
          <select v-model="form.role" class="form-field">
            <option v-for="option in timelineRoleOptions" :key="option.value" :value="option.value">
              {{ option.label }}
            </option>
          </select>
        </label>

        <button :disabled="saving || !form.cardId" class="primary-button" type="submit">绑定到事件</button>
      </form>

      <div v-if="boundCards.length === 0" class="empty-panel small-empty-panel">
        暂无参与设定。
      </div>

      <div v-else class="timeline-bound-card-list">
        <article v-for="item in boundCards" :key="item.link.id" class="timeline-bound-card">
          <button class="timeline-bound-card-main" type="button" @click="$emit('open-card', item.card.id)">
            <strong>{{ item.card.name }}</strong>
            <small>{{ cardTypeLabel(item.card.type) }} · {{ timelineRoleLabel(item.link.role) }}</small>
          </button>
          <button class="tree-danger-button" type="button" @click="$emit('detach', event.id, item.card.id)">移除
          </button>
        </article>
      </div>
    </template>
  </section>
</template>

<script lang="ts" setup>
import {cardTypeLabel, type SettingCard} from '~/types/card'
import type {TimelineEvent, TimelineEventCard, TimelineParticipantRole} from '~/types/timeline'
import {timelineRoleLabel, timelineRoleOptions} from '~/types/timeline'

const props = defineProps<{
  event: TimelineEvent | null
  eventCards: TimelineEventCard[]
  cards: SettingCard[]
  saving?: boolean
}>()

const emit = defineEmits<{
  attach: [cardId: string, role: TimelineParticipantRole]
  detach: [timelineEventId: string, cardId: string]
  'open-card': [cardId: string]
}>()

const form = reactive({
  cardId: '',
  role: 'participant' as TimelineParticipantRole
})

const cardsById = computed(() => new Map(props.cards.map(card => [card.id, card])))
const boundIds = computed(() => new Set(props.eventCards.map(item => item.cardId)))
const attachableCards = computed(() => props.cards.filter(card => !boundIds.value.has(card.id)))
const boundCards = computed(() => {
  return props.eventCards
      .map(link => {
        const card = cardsById.value.get(link.cardId)
        return card ? {link, card} : null
      })
      .filter(Boolean) as Array<{ link: TimelineEventCard, card: SettingCard }>
})

watch(() => props.event?.id, () => {
  form.cardId = ''
  form.role = 'participant'
})

function attach() {
  if (!form.cardId) return
  emit('attach', form.cardId, form.role)
  form.cardId = ''
  form.role = 'participant'
}
</script>
