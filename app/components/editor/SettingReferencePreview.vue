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
import {cardTypeLabel as baseCardTypeLabel, type SettingCard} from '~/types/card'

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
  return `${baseCardTypeLabel(type)}设定`
}


function fieldLabel(key: string, type: string) {
  const labelGroups: Record<string, Record<string, string>> = {
    character: {role: '定位', motivation: '动机', notes: '备注'},
    location: {atmosphere: '氛围', notes: '备注'},
    organization: {scope: '范围', goal: '目标', structure: '结构', notes: '备注'},
    item: {owner: '持有者', power: '作用', limitations: '限制', notes: '备注'},
    event: {time: '时间', cause: '起因', consequence: '结果', notes: '备注'},
    concept: {rules: '规则', limits: '限制', notes: '备注'}
  }
  return labelGroups[type]?.[key] ?? key
}
</script>
