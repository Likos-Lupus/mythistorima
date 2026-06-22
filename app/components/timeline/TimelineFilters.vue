<template>
  <section class="timeline-filters paper-card">
    <label>
      按人物 / 组织 / 道具筛选
      <select :value="cardId || ''" class="compact-select-field" @change="emitCard">
        <option value="">全部经历</option>
        <option v-for="card in participantCards" :key="card.id" :value="card.id">
          {{ cardTypeLabel(card.type) }} · {{ card.name }}
        </option>
      </select>
    </label>

    <label>
      按地点筛选
      <select :value="locationId || ''" class="compact-select-field" @change="emitLocation">
        <option value="">全部地点</option>
        <option v-for="card in locationCards" :key="card.id" :value="card.id">
          {{ card.name }}
        </option>
      </select>
    </label>

    <button class="secondary-button" type="button" @click="$emit('clear')">清空筛选</button>
  </section>
</template>

<script lang="ts" setup>
import {cardTypeLabel, type SettingCard} from '~/types/card'

const props = defineProps<{
  cards: SettingCard[]
  cardId?: string | null
  locationId?: string | null
}>()

const emit = defineEmits<{
  'update:cardId': [cardId: string | null]
  'update:locationId': [locationId: string | null]
  clear: []
}>()

const participantCards = computed(() => props.cards.filter(card => card.type !== 'location'))
const locationCards = computed(() => props.cards.filter(card => card.type === 'location'))

function emitCard(event: Event) {
  const value = (event.target as HTMLSelectElement).value
  emit('update:cardId', value || null)
}

function emitLocation(event: Event) {
  const value = (event.target as HTMLSelectElement).value
  emit('update:locationId', value || null)
}
</script>
