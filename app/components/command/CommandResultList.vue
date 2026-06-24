<template>
  <div ref="listElement" class="command-result-list" role="listbox">
    <button
        v-for="(item, index) in items"
        :key="item.id"
        :aria-selected="index === selectedIndex"
        :class="{ 'is-selected': index === selectedIndex, 'is-disabled': item.disabled }"
        :disabled="item.disabled"
        class="command-result-item"
        role="option"
        type="button"
        @mouseenter="$emit('select-index', index)"
        @click="$emit('execute', item)"
    >
      <span :class="`is-${item.kind}`" class="command-result-kind">{{ kindLabel(item.kind) }}</span>
      <span class="command-result-copy">
        <strong>{{ item.title }}</strong>
        <small>{{ item.subtitle }}</small>
      </span>
      <CommandShortcutHint v-if="item.shortcut" :shortcut="item.shortcut" compact/>
    </button>

    <div v-if="!items.length" class="command-result-empty">
      没有匹配的命令、章节、设定或事项。
    </div>
  </div>
</template>

<script lang="ts" setup>
import CommandShortcutHint from '~/components/command/CommandShortcutHint.vue'
import type {CommandPaletteItem, CommandPaletteItemKind} from '~/types/command'

const props = defineProps<{
  items: CommandPaletteItem[]
  selectedIndex: number
}>()

defineEmits<{
  execute: [item: CommandPaletteItem]
  'select-index': [index: number]
}>()

const listElement = ref<HTMLElement | null>(null)

watch(() => props.selectedIndex, index => {
  nextTick(() => {
    const element = listElement.value?.querySelectorAll<HTMLElement>('.command-result-item')[index]
    element?.scrollIntoView({block: 'nearest'})
  })
})

function kindLabel(kind: CommandPaletteItemKind) {
  if (kind === 'document') return '文档'
  if (kind === 'card') return '设定'
  if (kind === 'note') return '事项'
  return '命令'
}
</script>
