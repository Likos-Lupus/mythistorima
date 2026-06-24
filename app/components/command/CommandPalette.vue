<template>
  <UModal
      v-model:open="modalOpen"
      :ui="{ content: 'max-w-2xl overflow-hidden p-0' }"
  >
    <template #content>
      <UCommandPalette
          v-model:search-term="searchTerm"
          :fuse="{ resultLimit: 100 }"
          :groups="groups"
          :virtualize="{ estimateSize: 44, overscan: 12 }"
          autofocus
          class="h-[min(70vh,38rem)]"
          placeholder="搜索命令、章节、设定卡或事项…"
      >
        <template #footer>
          <div class="command-palette-footer">
            <span>↑↓ 选择</span>
            <span>Enter 执行</span>
            <span>Esc 关闭</span>
            <UKbd v-if="openShortcut" size="sm">{{ formattedShortcut }}</UKbd>
          </div>
        </template>
      </UCommandPalette>
    </template>
  </UModal>
</template>

<script lang="ts" setup>
import type {CommandPaletteItem as NuxtCommandPaletteItem} from '@nuxt/ui'
import type {CommandPaletteItem} from '~/types/command'
import {formatShortcut} from '~/utils/shortcut'

const props = withDefaults(defineProps<{
  open: boolean
  items: CommandPaletteItem[]
  openShortcut?: string | null
}>(), {
  openShortcut: ''
})

const emit = defineEmits<{
  close: []
  execute: [item: CommandPaletteItem]
}>()

const searchTerm = ref('')

const modalOpen = computed({
  get: () => props.open,
  set: value => {
    if (!value) emit('close')
  }
})

const formattedShortcut = computed(() => formatShortcut(props.openShortcut ?? '', true))

const groups = computed(() => {
  const grouped = new Map<string, CommandPaletteItem[]>()
  for (const item of props.items) {
    const current = grouped.get(item.group) ?? []
    current.push(item)
    grouped.set(item.group, current)
  }

  return [...grouped.entries()].map(([label, items], index) => ({
    id: `group-${index}-${label}`,
    label,
    items: items.map(item => toNuxtItem(item))
  }))
})

watch(() => props.open, open => {
  if (open) searchTerm.value = ''
})

function toNuxtItem(item: CommandPaletteItem): NuxtCommandPaletteItem {
  return {
    id: item.id,
    label: item.title,
    suffix: item.subtitle,
    icon: kindIcon(item.kind),
    disabled: item.disabled,
    kbds: item.shortcut ? shortcutTokens(item.shortcut) : undefined,
    onSelect: () => {
      emit('execute', item)
      emit('close')
    }
  }
}

function kindIcon(kind: CommandPaletteItem['kind']) {
  if (kind === 'document') return 'i-lucide-file-text'
  if (kind === 'card') return 'i-lucide-contact-round'
  if (kind === 'note') return 'i-lucide-list-checks'
  return 'i-lucide-command'
}

function shortcutTokens(shortcut: string) {
  const isMac = typeof navigator !== 'undefined' && navigator.platform.includes('Mac')
  return shortcut
      .replace('Mod', isMac ? 'meta' : 'ctrl')
      .split('+')
      .map(token => token.trim().toLowerCase())
      .filter(Boolean)
}
</script>
