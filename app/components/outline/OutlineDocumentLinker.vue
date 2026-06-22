<template>
  <div class="outline-document-linker">
    <label>
      绑定章节 / 场景
      <select :value="modelValue ?? ''" class="outline-select" @change="onChange">
        <option value="">未绑定</option>
        <option
            v-for="document in linkableDocuments"
            :key="document.id"
            :value="document.id"
        >
          {{ documentLabel(document) }}
        </option>
      </select>
    </label>

    <div v-if="linkedDocument" class="outline-linked-document-card">
      <span>{{ documentTypeLabel(linkedDocument.type) }}</span>
      <strong>{{ linkedDocument.title }}</strong>
      <button class="tree-action-button" type="button" @click="$emit('openDocument', linkedDocument.id)">打开</button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import type {NovelDocument} from '~/types/document'

const props = defineProps<{
  modelValue?: string | null
  documents: NovelDocument[]
}>()

const emit = defineEmits<{
  'update:modelValue': [documentId: string | null]
  openDocument: [documentId: string]
}>()

const linkableDocuments = computed(() => props.documents.filter(document => document.type === 'chapter' || document.type === 'scene'))
const linkedDocument = computed(() => props.modelValue ? props.documents.find(document => document.id === props.modelValue) ?? null : null)

function onChange(event: Event) {
  const target = event.target as HTMLSelectElement | null
  emit('update:modelValue', target?.value || null)
}

function documentTypeLabel(type: string) {
  switch (type) {
    case 'chapter':
      return '章'
    case 'scene':
      return '场景'
    case 'volume':
      return '卷'
    default:
      return '文档'
  }
}

function documentLabel(document: NovelDocument) {
  return `${documentTypeLabel(document.type)} · ${document.title}`
}
</script>
