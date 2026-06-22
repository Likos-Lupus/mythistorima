<template>
  <section class="outline-editor-panel glass-panel">
    <div v-if="node" class="outline-editor">
      <header class="outline-editor-header">
        <div>
          <p class="card-editor-kicker">Outline Node</p>
          <h2>{{ draft.title || '未命名大纲节点' }}</h2>
        </div>
        <span class="outline-status-pill">{{ statusLabel(draft.status) }}</span>
      </header>

      <div class="outline-editor-form">
        <label>
          标题
          <input v-model="draft.title" class="outline-input" placeholder="例如：第一幕转折" type="text">
        </label>

        <div class="outline-form-grid">
          <label>
            类型
            <select v-model="draft.type" class="outline-select">
              <option v-for="option in typeOptions" :key="option.value" :value="option.value">
                {{ option.label }}
              </option>
            </select>
          </label>
          <label>
            状态
            <select v-model="draft.status" class="outline-select">
              <option value="planned">计划</option>
              <option value="drafting">推进中</option>
              <option value="done">完成</option>
              <option value="archived">归档</option>
            </select>
          </label>
        </div>

        <label>
          摘要
          <textarea
              v-model="draft.summary"
              class="outline-textarea"
              placeholder="写下这个节点的剧情目标、冲突、转折或主题。"
              rows="8"
          />
        </label>

        <OutlineDocumentLinker
            v-model="linkedDocumentDraft"
            :documents="documents"
            @open-document="$emit('openDocument', $event)"
        />

        <div class="outline-editor-actions">
          <button :disabled="saving" class="primary-button" type="button" @click="saveNode">
            {{ saving ? '保存中…' : '保存大纲' }}
          </button>
          <button
              v-if="node.linkedDocumentId"
              :disabled="saving"
              class="secondary-button"
              type="button"
              @click="unlinkDocument"
          >
            解除绑定
          </button>
          <button v-if="node.linkedDocumentId" class="secondary-button" type="button" @click="openLinkedDocument">
            打开绑定章节
          </button>
        </div>
      </div>
    </div>

    <div v-else class="outline-editor-empty empty-panel">
      请选择一个大纲节点，或新建剧情 / 冲突 / 转折节点。
    </div>
  </section>
</template>

<script lang="ts" setup>
import OutlineDocumentLinker from '~/components/outline/OutlineDocumentLinker.vue'
import type {NovelDocument} from '~/types/document'
import type {OutlineNode, OutlineNodeStatus, OutlineNodeType, UpdateOutlineNodeInput} from '~/types/outline'

const props = defineProps<{
  node: OutlineNode | null
  documents: NovelDocument[]
  saving?: boolean
}>()

const emit = defineEmits<{
  save: [input: UpdateOutlineNodeInput]
  linkDocument: [outlineNodeId: string, documentId: string]
  unlinkDocument: [outlineNodeId: string]
  openDocument: [documentId: string]
}>()

const draft = reactive({
  title: '',
  type: 'plot' as OutlineNodeType,
  status: 'planned' as OutlineNodeStatus,
  summary: ''
})
const linkedDocumentDraft = ref<string | null>(null)
let syncingFromNode = false

const typeOptions = [
  {value: 'plot', label: '剧情'},
  {value: 'conflict', label: '冲突'},
  {value: 'twist', label: '转折'},
  {value: 'event', label: '事件'},
  {value: 'arc', label: '支线'},
  {value: 'theme', label: '主题'},
  {value: 'note', label: '备注'}
]

watch(() => props.node, node => {
  syncingFromNode = true
  draft.title = node?.title ?? ''
  draft.type = node?.type ?? 'plot'
  draft.status = node?.status ?? 'planned'
  draft.summary = node?.summary ?? ''
  linkedDocumentDraft.value = node?.linkedDocumentId ?? null
  nextTick(() => {
    syncingFromNode = false
  })
}, {immediate: true})

watch(linkedDocumentDraft, async (value, oldValue) => {
  if (syncingFromNode || !props.node || value === oldValue) return
  if (value) {
    emit('linkDocument', props.node.id, value)
  } else if (oldValue) {
    emit('unlinkDocument', props.node.id)
  }
})

function saveNode() {
  if (!props.node) return
  emit('save', {
    outlineNodeId: props.node.id,
    title: draft.title,
    type: draft.type,
    status: draft.status,
    summary: draft.summary,
    metadataJson: props.node.metadataJson
  })
}

function unlinkDocument() {
  if (!props.node) return
  linkedDocumentDraft.value = null
}

function openLinkedDocument() {
  if (!props.node?.linkedDocumentId) return
  emit('openDocument', props.node.linkedDocumentId)
}

function statusLabel(status: string) {
  switch (status) {
    case 'drafting':
      return '推进中'
    case 'done':
      return '完成'
    case 'archived':
      return '归档'
    default:
      return '计划'
  }
}
</script>
