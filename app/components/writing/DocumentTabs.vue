<template>
  <section aria-label="打开的文档标签" class="document-tabs-bar">
    <UTabs
        v-if="tabs.length"
        v-model="activeValue"
        :content="false"
        :items="tabItems"
        :ui="tabsUi"
        activation-mode="manual"
        class="document-tabs-tabs"
        color="neutral"
        size="sm"
        variant="link"
        @auxclick="handleAuxClick"
        @dblclick="handleDoubleClick"
    >
      <template #leading="{ item }">
        <UIcon :name="item.preview ? 'i-lucide-file-search' : 'i-lucide-file-text'" class="document-tabs-icon"/>
      </template>

      <template #default="{ item }">
        <span
            :class="{'is-preview': item.preview}"
            :data-document-id="item.value"
            :title="item.title"
            class="document-tabs-label"
        >
          {{ item.title }}
        </span>
      </template>

      <template #trailing="{ item }">
        <span v-if="item.dirty" aria-label="有未保存修改" class="document-tab-dirty"/>
        <UIcon v-else-if="item.pinned" class="document-tab-pin" name="i-lucide-pin"/>
        <UDropdownMenu :items="tabMenu(String(item.value))">
          <UButton
              aria-label="标签操作"
              color="neutral"
              icon="i-lucide-more-horizontal"
              size="xs"
              variant="ghost"
              @click.stop
          />
        </UDropdownMenu>
        <UButton
            aria-label="关闭标签"
            color="neutral"
            icon="i-lucide-x"
            size="xs"
            variant="ghost"
            @click.stop="$emit('close', String(item.value))"
        />
      </template>
    </UTabs>

    <div v-else aria-hidden="true" class="document-tabs-placeholder"/>
  </section>
</template>

<script lang="ts" setup>
import type {OpenDocumentTab} from '~/stores/documentTabs.store'

const props = defineProps<{
  tabs: OpenDocumentTab[]
  activeDocumentId?: string | null
}>()

const emit = defineEmits<{
  activate: [documentId: string]
  close: [documentId: string]
  pin: [documentId: string]
  closeOthers: [documentId: string]
  closeAll: []
}>()

const activeValue = computed({
  get: () => props.activeDocumentId ?? undefined,
  set: value => {
    if (typeof value === 'string') emit('activate', value)
  }
})

const tabItems = computed(() => props.tabs.map(tab => ({
  label: tab.title,
  title: tab.title,
  value: tab.documentId,
  preview: tab.preview,
  pinned: tab.pinned,
  dirty: tab.dirty,
  ui: {
    trigger: tab.preview ? 'italic' : undefined
  }
})))

const tabsUi = {
  root: 'document-tabs-root',
  list: 'document-tabs-list',
  trigger: 'document-tabs-trigger',
  label: 'min-w-0',
  leadingIcon: 'document-tabs-leading-icon'
}

function tabMenu(documentId: string) {
  return [[
    {label: '固定标签', icon: 'i-lucide-pin', onSelect: () => emit('pin', documentId)},
    {label: '关闭', icon: 'i-lucide-x', onSelect: () => emit('close', documentId)},
    {label: '关闭其他', icon: 'i-lucide-panel-top-close', onSelect: () => emit('closeOthers', documentId)},
    {label: '关闭全部', icon: 'i-lucide-panels-top-left', onSelect: () => emit('closeAll')}
  ]]
}

function handleDoubleClick(event: MouseEvent) {
  const id = documentIdFromEvent(event)
  if (id) emit('pin', id)
}

function handleAuxClick(event: MouseEvent) {
  if (event.button !== 1) return
  const id = documentIdFromEvent(event)
  if (!id) return
  event.preventDefault()
  emit('close', id)
}

function documentIdFromEvent(event: MouseEvent) {
  const target = event.target instanceof HTMLElement
      ? event.target.closest<HTMLElement>('[data-document-id]')
      : null
  return target?.dataset.documentId ?? null
}
</script>
