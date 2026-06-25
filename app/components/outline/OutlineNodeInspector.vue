<template>
  <section class="story-inspector-content">
    <div class="story-panel-header">
      <div>
        <strong>节点详情</strong>
        <span>{{ node ? '编辑剧情节点' : '未选择' }}</span>
      </div>
      <UButton v-if="node" color="neutral" icon="i-lucide-plus" size="xs" variant="ghost"
               @click="$emit('create-child', node.id)"/>
    </div>

    <UEmpty
        v-if="!node"
        description="在看板、节点列表或图谱中选择一个大纲节点。"
        icon="i-lucide-list-tree"
        title="未选择节点"
    />

    <UForm v-else :state="form" class="story-form" @submit="save">
      <UFormField label="标题" name="title" required>
        <UInput v-model="form.title" class="w-full" size="sm"/>
      </UFormField>

      <div class="story-form-grid">
        <UFormField label="类型" name="type">
          <USelect v-model="form.type" :items="typeItems" class="w-full" label-key="label" size="sm" value-key="value"/>
        </UFormField>
        <UFormField label="状态" name="status">
          <USelect v-model="form.status" :items="statusItems" class="w-full" label-key="label" size="sm"
                   value-key="value"/>
        </UFormField>
      </div>

      <UFormField label="绑定章节" name="linkedDocumentId">
        <USelect v-model="form.linkedDocumentId" :items="documentItems" class="w-full" label-key="label" size="sm"
                 value-key="value"/>
      </UFormField>

      <UFormField label="摘要" name="summary">
        <UTextarea v-model="form.summary" :rows="6" autoresize class="w-full" size="sm"/>
      </UFormField>

      <div class="story-inspector-actions">
        <UButton :loading="saving" icon="i-lucide-save" label="保存" size="sm" type="submit"/>
        <UButton
            v-if="node.linkedDocumentId"
            color="neutral"
            icon="i-lucide-file-pen-line"
            label="打开章节"
            size="sm"
            variant="ghost"
            @click="$emit('open-document', node.linkedDocumentId)"
        />
        <UButton color="error" icon="i-lucide-trash-2" label="删除" size="sm" variant="ghost"
                 @click="$emit('delete', node.id)"/>
      </div>
    </UForm>
  </section>
</template>

<script lang="ts" setup>
import type {NovelDocument} from '~/types/document'
import type {OutlineNode, OutlineNodeStatus, OutlineNodeType, UpdateOutlineNodeInput} from '~/types/outline'
import {OUTLINE_STATUS_OPTIONS} from '~/utils/outlinePresentation'

const props = defineProps<{
  node: OutlineNode | null
  documents: NovelDocument[]
  saving?: boolean
}>()

const emit = defineEmits<{
  save: [payload: UpdateOutlineNodeInput & { linkedDocumentId?: string | null }]
  delete: [outlineNodeId: string]
  'create-child': [outlineNodeId: string]
  openDocument: [documentId: string]
}>()

const form = reactive({
  title: '',
  type: 'plot' as OutlineNodeType,
  status: 'planned' as OutlineNodeStatus,
  linkedDocumentId: '__none__',
  summary: ''
})

const typeItems = [
  {label: '剧情', value: 'plot'},
  {label: '冲突', value: 'conflict'},
  {label: '转折', value: 'twist'},
  {label: '事件', value: 'event'},
  {label: '支线', value: 'arc'},
  {label: '主题', value: 'theme'},
  {label: '备注', value: 'note'}
]
const statusItems = OUTLINE_STATUS_OPTIONS.filter(option => option.value !== 'all').map(option => ({
  label: option.label,
  value: option.value
}))
const documentItems = computed(() => [
  {label: '不绑定章节', value: '__none__'},
  ...props.documents.map(document => ({label: `${document.title} · ${document.characterCount} 字`, value: document.id}))
])

watch(() => props.node, node => {
  form.title = node?.title ?? ''
  form.type = node?.type ?? 'plot'
  form.status = node?.status ?? 'planned'
  form.linkedDocumentId = node?.linkedDocumentId ?? '__none__'
  form.summary = node?.summary ?? ''
}, {immediate: true})

function save() {
  if (!props.node) return
  emit('save', {
    outlineNodeId: props.node.id,
    title: form.title.trim() || props.node.title,
    type: form.type,
    status: form.status,
    summary: form.summary.trim(),
    linkedDocumentId: form.linkedDocumentId === '__none__' ? null : form.linkedDocumentId
  })
}
</script>
