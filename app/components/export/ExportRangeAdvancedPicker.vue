<template>
  <section aria-label="导出范围" class="publication-panel">
    <header class="publication-panel-header">
      <div>
        <p class="publication-kicker">Range</p>
        <h2>范围</h2>
        <p>当前文档会自动包含其子文档；自选范围只导出勾选项。</p>
      </div>
    </header>

    <div aria-label="导出范围" class="publication-range-options" role="radiogroup">
      <UButton
          v-for="option in rangeOptions"
          :key="option.value"
          :aria-checked="modelValue === option.value"
          :color="modelValue === option.value ? 'primary' : 'neutral'"
          :disabled="option.value === 'current' && !activeDocumentId"
          :variant="modelValue === option.value ? 'soft' : 'ghost'"
          block
          class="publication-range-button"
          role="radio"
          size="sm"
          @click="selectRange(option.value)"
      >
        <span>
          <strong>{{ option.label }}</strong>
          <small>{{ option.description }}</small>
        </span>
      </UButton>
    </div>

    <section v-if="modelValue === 'selected'" aria-label="自选文档" class="publication-selection-panel">
      <div class="publication-selection-actions">
        <UBadge color="neutral" size="sm" variant="soft">已选 {{ selectedDocumentIds.length }}</UBadge>
        <div>
          <UButton label="全选" size="xs" variant="ghost" @click="selectAll"/>
          <UButton label="清空" size="xs" variant="ghost" @click="clearAll"/>
        </div>
      </div>

      <UEmpty
          v-if="!flatDocuments.length"
          description="创建章节或卷后即可选择导出范围。"
          icon="i-lucide-file-text"
          title="没有可导出的文档"
      />

      <div v-else class="publication-document-checks">
        <UCheckbox
            v-for="document in flatDocuments"
            :key="document.id"
            :model-value="selectedDocumentIds.includes(document.id)"
            :style="{'--publication-depth': document.depth}"
            class="publication-document-check"
            @update:model-value="toggleDocument(document.id)"
        >
          <template #label>
            <span class="publication-document-check-label">
              <span>{{ document.title }}</span>
              <small>{{ typeLabel(document.type) }} · {{ document.characterCount }} 字</small>
            </span>
          </template>
        </UCheckbox>
      </div>
    </section>
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
const rangeOptions = computed<{ label: string, description: string, value: ExportRange }[]>(() => [
  {label: '全项目', description: `${flatDocuments.value.length} 个文档`, value: 'all'},
  {label: '当前文档及子文档', description: currentDocumentTitle.value, value: 'current'},
  {label: '自选文档', description: `已选 ${props.selectedDocumentIds.length} 项`, value: 'selected'}
])

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

function flattenDocuments(documents: NovelDocument[], parentId: string | null = null, depth = 0): FlatDocument[] {
  return documents
      .filter(document => document.parentId === parentId)
      .sort((a, b) => a.orderIndex - b.orderIndex)
      .flatMap(document => [
        {...document, depth},
        ...flattenDocuments(documents, document.id, depth + 1)
      ])
}

function typeLabel(type: NovelDocument['type']) {
  if (type === 'volume') return '卷'
  if (type === 'scene') return '场景'
  return '章节'
}
</script>
