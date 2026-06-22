<template>
  <div aria-label="大纲状态筛选" class="outline-status-filter" role="group">
    <button
        v-for="option in options"
        :key="String(option.value)"
        :class="['outline-filter-chip', { 'is-active': modelValue === option.value }]"
        :title="option.description"
        type="button"
        @click="$emit('update:modelValue', option.value)"
    >
      {{ option.label }}
      <span v-if="typeof countFor(option.value) === 'number'">{{ countFor(option.value) }}</span>
    </button>
  </div>
</template>

<script lang="ts" setup>
import type {OutlineNode, OutlineNodeStatus} from '~/types/outline'
import {OUTLINE_STATUS_OPTIONS} from '~/utils/outlinePresentation'

const props = withDefaults(defineProps<{
  modelValue: OutlineNodeStatus | 'all'
  nodes: OutlineNode[]
  includeArchived?: boolean
}>(), {
  includeArchived: true
})

defineEmits<{
  'update:modelValue': [status: OutlineNodeStatus | 'all']
}>()

const options = computed(() => props.includeArchived
    ? OUTLINE_STATUS_OPTIONS
    : OUTLINE_STATUS_OPTIONS.filter(option => option.value !== 'archived')
)

function countFor(status: OutlineNodeStatus | 'all') {
  if (status === 'all') return props.includeArchived ? props.nodes.length : props.nodes.filter(node => node.status !== 'archived').length
  return props.nodes.filter(node => node.status === status).length
}
</script>
