<template>
  <section class="export-range-picker glass-panel">
    <header class="export-template-panel-header">
      <div>
        <h3>导出范围</h3>
        <p>当前文档会自动包含其子文档；自选范围只导出勾选项。</p>
      </div>
    </header>

    <div class="export-range-options">
      <label :class="{ 'is-active': modelValue === 'all' }">
        <input :checked="modelValue === 'all'" name="export-range" type="radio" @change="selectRange('all')">
        <span><strong>全项目</strong><small>{{ flatDocuments.length }} 个文档</small></span>
      </label>
      <label :class="{ 'is-active': modelValue === 'current' }">
        <input
            :checked="modelValue === 'current'"
            :disabled="!activeDocumentId"
            name="export-range"
            type="radio"
            @change="selectRange('current')"
        >
        <span><strong>当前文档及子文档</strong><small>{{ currentDocumentTitle }}</small></span>
      </label>
      <label :class="{ 'is-active': modelValue === 'selected' }">
        <input :checked="modelValue === 'selected'" name="export-range" type="radio" @change="selectRange('selected')">
        <span><strong>自选文档</strong><small>已选 {{ selectedDocumentIds.length }} 项</small></span>
      </label>
    </div>

    <div v-if="modelValue === 'selected'" class="export-document-selection">
      <div class="export-selection-actions">
        <button class="text-button" type="button" @click="selectAll">全选</button>
        <button class="text-button" type="button" @click="clearAll">清空</button>
      </div>
      <label
          v-for="document in flatDocuments"
          :key="document.id"
          :style="{ '--export-depth': document.depth }"
          class="export-document-check"
      >
        <input
            :checked="selectedDocumentIds.includes(document.id)"
            type="checkbox"
            @change="toggleDocument(document.id)"
        >
        <span class="export-document-type">{{ typeLabel(document.type) }}</span>
        <span>{{ document.title }}</span>
        <small>{{ document.characterCount }} 字</small>
      </label>
    </div>
  </section>
</template>

<script lang="ts" setup>
import type {ExportRange} from '~/types/export'
import type {NovelDocument} from '~/types/document'

interface FlatDocument extends NovelDocument {
  depth: number
}

const props = defineProps<{
  documents: NovelDocument[]
  activeDocumentId: string | null
  modelValue: ExportRange
  selectedDocumentIds: string[]
}>()

const emit = defineEmits<{
  'update:modelValue': [range: ExportRange]
  'update:selectedDocumentIds': [ids: string[]]
}>()

const flatDocuments = computed(() => flattenDocuments(props.documents))
const currentDocumentTitle = computed(
    () => props.documents.find(document => document.id === props.activeDocumentId)?.title ?? '当前未选择文档'
)

function selectRange(range: ExportRange) {
  emit('update:modelValue', range)
}

function toggleDocument(documentId: string) {
  const selected = new Set(props.selectedDocumentIds)
  if (selected.has(documentId)) selected.delete(documentId)
  else selected.add(documentId)
  emit('update:selectedDocumentIds', [...selected])
}

function selectAll() {
  emit('update:selectedDocumentIds', flatDocuments.value.map(document => document.id))
}

function clearAll() {
  emit('update:selectedDocumentIds', [])
}

function flattenDocuments(documents: NovelDocument[]) {
  const result: FlatDocument[] = []
  const children = new Map<string | null, NovelDocument[]>()
  for (const document of documents) {
    const parentId = document.parentId || null
    const list = children.get(parentId) ?? []
    list.push(document)
    children.set(parentId, list)
  }
  for (const list of children.values()) {
    list.sort((a, b) => a.sortOrder - b.sortOrder || a.createdAt - b.createdAt)
  }
  const walk = (parentId: string | null, depth: number) => {
    for (const document of children.get(parentId) ?? []) {
      result.push({...document, depth})
      walk(document.id, depth + 1)
    }
  }
  walk(null, 0)
  return result
}

function typeLabel(type: string) {
  if (type === 'volume') return '卷'
  if (type === 'chapter') return '章'
  if (type === 'scene') return '场景'
  return '文档'
}
</script>
