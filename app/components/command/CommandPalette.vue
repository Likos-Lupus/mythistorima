<template>
  <Teleport to="body">
    <div v-if="open" class="command-palette-backdrop" @mousedown.self="$emit('close')">
      <section
          aria-label="命令面板"
          aria-modal="true"
          class="command-palette"
          role="dialog"
          @keydown="handleKeydown"
      >
        <header class="command-palette-search">
          <span aria-hidden="true" class="command-palette-search-icon">⌕</span>
          <input
              ref="inputElement"
              v-model="query"
              autocomplete="off"
              placeholder="搜索命令、章节、设定卡或事项…"
              spellcheck="false"
              type="text"
          >
          <CommandShortcutHint :shortcut="openShortcut" compact/>
        </header>

        <div class="command-palette-meta">
          <span>{{ filteredItems.length }} 项结果</span>
          <span>↑↓ 选择 · Enter 执行 · Esc 关闭</span>
        </div>

        <CommandResultList
            :items="filteredItems"
            :selected-index="selectedIndex"
            @execute="execute"
            @select-index="selectedIndex = $event"
        />
      </section>
    </div>
  </Teleport>
</template>

<script lang="ts" setup>
import CommandResultList from '~/components/command/CommandResultList.vue'
import CommandShortcutHint from '~/components/command/CommandShortcutHint.vue'
import type {CommandPaletteItem} from '~/types/command'

const props = defineProps<{
  open: boolean
  items: CommandPaletteItem[]
  openShortcut?: string | null
}>()

const emit = defineEmits<{
  close: []
  execute: [item: CommandPaletteItem]
}>()

const inputElement = ref<HTMLInputElement | null>(null)
const query = ref('')
const selectedIndex = ref(0)

const filteredItems = computed(() => {
  const normalized = normalizeSearch(query.value)
  const source = normalized
      ? props.items.filter(item => {
          const haystack = normalizeSearch([
            item.title,
            item.subtitle,
            item.group,
            ...item.keywords
          ].join(' '))
          return normalized.split(/\s+/).every(token => haystack.includes(token))
        })
      : props.items

  return source.slice(0, 80)
})

watch(() => props.open, open => {
  if (!open) return
  query.value = ''
  selectedIndex.value = 0
  nextTick(() => inputElement.value?.focus())
})

watch(query, () => {
  selectedIndex.value = 0
})

watch(filteredItems, items => {
  if (!items.length) selectedIndex.value = 0
  else if (selectedIndex.value >= items.length) selectedIndex.value = items.length - 1
})

function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'Escape') {
    event.preventDefault()
    emit('close')
    return
  }
  if (event.key === 'ArrowDown') {
    event.preventDefault()
    selectedIndex.value = filteredItems.value.length
        ? (selectedIndex.value + 1) % filteredItems.value.length
        : 0
    return
  }
  if (event.key === 'ArrowUp') {
    event.preventDefault()
    selectedIndex.value = filteredItems.value.length
        ? (selectedIndex.value - 1 + filteredItems.value.length) % filteredItems.value.length
        : 0
    return
  }
  if (event.key === 'Enter') {
    event.preventDefault()
    const item = filteredItems.value[selectedIndex.value]
    if (item && !item.disabled) execute(item)
  }
}

function execute(item: CommandPaletteItem) {
  emit('execute', item)
}

function normalizeSearch(value: string) {
  return value.trim().toLocaleLowerCase('zh-CN').replace(/\s+/g, ' ')
}
</script>
