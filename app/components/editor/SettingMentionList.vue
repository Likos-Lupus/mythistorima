<template>
  <div
      :style="positionStyle"
      aria-label="设定卡建议"
      class="setting-mention-list glass-panel"
      role="listbox"
  >
    <header class="setting-mention-header">
      <span>@ 设定引用</span>
      <small v-if="query">{{ query }}</small>
    </header>

    <button
        v-for="(card, index) in cards"
        :key="card.id"
        :aria-selected="index === activeIndex"
        :class="['setting-mention-item', { 'is-active': index === activeIndex }]"
        role="option"
        type="button"
        @mouseenter="$emit('hover', index)"
        @mousedown.prevent="$emit('select', card)"
    >
      <span class="setting-mention-type">{{ cardTypeLabel(card.type) }}</span>
      <span class="setting-mention-main">
        <strong>{{ card.name }}</strong>
        <small v-if="aliasesLabel(card.aliasesJson)">{{ aliasesLabel(card.aliasesJson) }}</small>
      </span>
    </button>

    <p v-if="cards.length === 0" class="setting-mention-empty">
      没有匹配的设定卡。先在“设定”工作区创建人物、地点或概念。
    </p>
  </div>
</template>

<script lang="ts" setup>
import type {SettingCard} from '~/types/card'

const props = defineProps<{
  cards: SettingCard[]
  activeIndex: number
  query: string
  top: number
  left: number
}>()

defineEmits<{
  select: [card: SettingCard]
  hover: [index: number]
}>()

const positionStyle = computed(() => ({
  top: `${props.top}px`,
  left: `${props.left}px`
}))

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

function aliasesLabel(raw: string) {
  try {
    const aliases = JSON.parse(raw)
    return Array.isArray(aliases) && aliases.length > 0
        ? aliases.slice(0, 3).map(String).join(' / ')
        : ''
  } catch {
    return ''
  }
}
</script>
