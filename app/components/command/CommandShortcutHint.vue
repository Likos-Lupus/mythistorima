<template>
  <kbd v-if="shortcut" :class="{ 'is-compact': compact }" class="command-shortcut-hint">
    {{ label }}
  </kbd>
</template>

<script lang="ts" setup>
import {formatShortcut} from '~/utils/shortcut'

const props = withDefaults(defineProps<{
  shortcut?: string | null
  compact?: boolean
}>(), {
  shortcut: '',
  compact: false
})

const label = computed(() => formatShortcut(props.shortcut ?? '', props.compact))
</script>
