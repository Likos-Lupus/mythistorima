<template>
  <div aria-label="设定卡类型" class="card-type-tabs" role="tablist">
    <button
        v-for="item in items"
        :key="item.value"
        :class="['card-type-tab', { 'is-active': modelValue === item.value }]"
        type="button"
        @click="$emit('update:modelValue', item.value)"
    >
      <span>{{ item.label }}</span>
      <strong>{{ counts?.[item.value] ?? 0 }}</strong>
    </button>
  </div>
</template>

<script lang="ts" setup>
import {type CardType, cardTypeOptions} from '~/types/card'

defineProps<{
  modelValue: CardType
  counts?: Record<CardType, number>
}>()

defineEmits<{
  'update:modelValue': [value: CardType]
}>()

const items: Array<{ value: CardType, label: string }> = cardTypeOptions
</script>
