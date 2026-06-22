<template>
  <div aria-label="关系类型筛选" class="relation-type-picker">
    <button
        v-for="option in relationTypeOptions"
        :key="option.value"
        :class="['relation-filter-chip', { 'is-active': modelValue === option.value }]"
        :title="option.hint"
        type="button"
        @click="$emit('update:modelValue', option.value)"
    >
      <span>{{ option.label }}</span>
      <strong>{{ counts?.[option.value] ?? 0 }}</strong>
    </button>
  </div>
</template>

<script lang="ts" setup>
import {type CardRelationType, relationTypeOptions} from '~/types/relation'

defineProps<{
  modelValue: CardRelationType | 'all'
  counts?: Record<string, number>
}>()

defineEmits<{
  'update:modelValue': [value: CardRelationType | 'all']
}>()
</script>
