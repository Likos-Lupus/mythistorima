<template>
  <section class="relation-list-panel paper-card">
    <header class="relation-panel-header">
      <div>
        <p class="relation-panel-kicker">card_relations</p>
        <h2>关系列表</h2>
      </div>
      <button class="primary-button" type="button" @click="$emit('create')">新建关系</button>
    </header>

    <div v-if="loading" class="empty-panel">正在加载关系…</div>
    <div v-else-if="relations.length === 0" class="empty-panel">
      暂无关系。可以先建立“人物属于组织”“人物持有道具”“事件发生于地点”等连接。
    </div>

    <div v-else class="relation-list">
      <button
          v-for="relation in relations"
          :key="relation.id"
          :class="['relation-list-item', { 'is-active': relation.id === activeRelationId }]"
          type="button"
          @click="$emit('select', relation.id)"
      >
        <span class="relation-list-type">{{ relationTypeLabel(relation.relationType) }}</span>
        <strong>{{ cardName(relation.sourceCardId) }} → {{ cardName(relation.targetCardId) }}</strong>
        <small>{{ relation.description || relationDirectionLabel(relation.direction) }}</small>
      </button>
    </div>
  </section>
</template>

<script lang="ts" setup>
import {type CardRelation, relationDirectionLabel, relationTypeLabel} from '~/types/relation'
import type {SettingCard} from '~/types/card'

const props = defineProps<{
  relations: CardRelation[]
  cards: SettingCard[]
  activeRelationId?: string | null
  loading?: boolean
}>()

defineEmits<{
  select: [relationId: string]
  create: []
}>()

const cardsById = computed(() => new Map(props.cards.map(card => [card.id, card])))

function cardName(cardId: string) {
  return cardsById.value.get(cardId)?.name ?? '未知设定'
}
</script>
