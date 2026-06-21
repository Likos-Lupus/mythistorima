<template>
  <section class="card-list-panel">
    <div class="card-list-toolbar">
      <input
          :value="query"
          class="card-search-input"
          placeholder="搜索名称、别名、简介…"
          type="search"
          @input="$emit('update:query', ($event.target as HTMLInputElement).value)"
      >
      <button class="primary-button card-new-button" type="button" @click="$emit('create')">新建</button>
    </div>

    <div v-if="loading" class="empty-panel">正在加载设定卡…</div>
    <div v-else-if="cards.length === 0" class="empty-panel">
      还没有设定卡。创建一个人物、地点或概念，开始整理你的故事世界。
    </div>

    <div v-else class="card-list">
      <button
          v-for="card in cards"
          :key="card.id"
          :class="['card-list-item', { 'is-active': card.id === activeCardId }]"
          type="button"
          @click="$emit('select', card.id)"
      >
        <span class="card-list-type">{{ cardTypeLabel(card.type) }}</span>
        <span class="card-list-name">{{ card.name }}</span>
        <span class="card-list-description">{{ card.description || '暂无简介' }}</span>
        <span v-if="aliasText(card.aliasesJson)" class="card-list-aliases">{{ aliasText(card.aliasesJson) }}</span>
      </button>
    </div>
  </section>
</template>

<script lang="ts" setup>
import type {SettingCard} from '~/types/card'

defineProps<{
  cards: SettingCard[]
  activeCardId?: string | null
  query: string
  loading?: boolean
}>()

defineEmits<{
  select: [cardId: string]
  create: []
  'update:query': [query: string]
}>()

function cardTypeLabel(type: string) {
  switch (type) {
    case 'character':
      return '人物'
    case 'location':
      return '地点'
    case 'concept':
      return '概念'
    default:
      return '设定'
  }
}

function aliasText(raw: string) {
  try {
    const aliases = JSON.parse(raw)
    if (!Array.isArray(aliases)) return ''
    return aliases.filter(Boolean).join(' / ')
  } catch {
    return ''
  }
}
</script>
