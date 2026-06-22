<template>
  <div class="outline-create-menu">
    <button
        v-for="option in rootOptions"
        :key="option.type"
        class="primary-button outline-create-button"
        type="button"
        @click="$emit('createRoot', option.type)"
    >
      + {{ option.label }}
    </button>
  </div>
</template>

<script lang="ts" setup>
import type {OutlineNodeType} from '~/types/outline'

defineEmits<{
  createRoot: [type: OutlineNodeType]
}>()

const rootOptions: Array<{ type: OutlineNodeType; label: string }> = [
  {type: 'plot', label: '剧情'},
  {type: 'conflict', label: '冲突'},
  {type: 'twist', label: '转折'}
]
</script>
