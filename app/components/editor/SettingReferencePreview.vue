<template>
  <aside
      :style="positionStyle"
      aria-label="设定卡预览"
      class="setting-reference-preview glass-panel"
  >
    <p class="setting-reference-kicker">{{ cardTypeLabel(card.type) }}</p>
    <h3>{{ card.name }}</h3>
    <p v-if="aliasesLabel" class="setting-reference-aliases">{{ aliasesLabel }}</p>
    <p v-if="card.description" class="setting-reference-description">{{ card.description }}</p>
    <dl v-if="fieldEntries.length" class="setting-reference-fields">
      <div v-for="entry in fieldEntries" :key="entry.key">
        <dt>{{ fieldLabel(entry.key, card.type) }}</dt>
        <dd>{{ entry.value }}</dd>
      </div>
    </dl>
  </aside>
</template>

<script lang="ts" setup>
import type {SettingCard} from '~/types/card'

const props = defineProps<{
  card: SettingCard
  top: number
  left: number
}>()

const positionStyle = computed(() => ({
  top: `${props.top}px`,
  left: `${props.left}px`
}))

const aliasesLabel = computed(() => {
  try {
    const aliases = JSON.parse(props.card.aliasesJson)
    return Array.isArray(aliases) && aliases.length > 0
        ? `别名：${aliases.slice(0, 5).map(String).join(' / ')}`
        : ''
  } catch {
    return ''
  }
})

const fieldEntries = computed(() => {
  try {
    const fields = JSON.parse(props.card.fieldsJson)
    if (!fields || typeof fields !== 'object' || Array.isArray(fields)) return []
    return Object.entries(fields as Record<string, unknown>)
        .map(([key, value]) => ({key, value: String(value ?? '').trim()}))
        .filter(entry => entry.value.length > 0)
        .slice(0, 4)
  } catch {
    return []
  }
})

function cardTypeLabel(type: string) {
  switch (type) {
    case 'character':
      return '人物设定'
    case 'location':
      return '地点设定'
    case 'concept':
      return '概念设定'
    default:
      return '设定卡'
  }
}

function fieldLabel(key: string, type: string) {
  const labels: Record<string, string> = type === 'character'
      ? {role: '定位', motivation: '动机', notes: '备注'}
      : type === 'location'
          ? {atmosphere: '氛围', notes: '备注'}
          : {rules: '规则', limits: '限制', notes: '备注'}
  return labels[key] ?? key
}
</script>
