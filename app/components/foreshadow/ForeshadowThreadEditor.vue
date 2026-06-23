<template>
  <section class="foreshadow-editor paper-card">
    <div class="list-panel-header">
      <div>
        <p class="section-kicker">线程编辑</p>
        <h2>{{ draft.id ? '编辑伏笔' : '新建伏笔' }}</h2>
      </div>
      <button class="secondary-button" type="button" @click="resetDraft">清空</button>
    </div>

    <div class="foreshadow-note-import">
      <label>
        从伏笔事项创建
        <select v-model="selectedNoteId" class="compact-select-field">
          <option value="">选择一个 foreshadow 事项…</option>
          <option v-for="note in foreshadowNotes" :key="note.id" :value="note.id">
            {{ note.title }}{{ note.documentTitle ? ` · ${note.documentTitle}` : '' }}
          </option>
        </select>
      </label>
      <button :disabled="!selectedNoteId || saving" class="secondary-button" type="button"
              @click="createFromSelectedNote">
        生成线程
      </button>
    </div>

    <form class="foreshadow-editor-form" @submit.prevent="submit">
      <label>
        标题
        <input v-model.trim="draft.title" class="text-field" placeholder="例如：第一章出现的银钥匙" required>
      </label>

      <label>
        说明
        <textarea v-model.trim="draft.description" class="textarea-field"
                  placeholder="记录提出方式、读者应获得的信息和预期回收方式。" rows="5"></textarea>
      </label>

      <div class="form-two-columns">
        <label>
          状态
          <select v-model="draft.status" class="compact-select-field">
            <option value="open">未回收</option>
            <option value="planned">计划回收</option>
            <option value="paid_off">已回收</option>
            <option value="abandoned">放弃</option>
            <option value="archived">归档</option>
          </select>
        </label>
        <label>
          优先级
          <select v-model="draft.priority" class="compact-select-field">
            <option value="high">高</option>
            <option value="normal">普通</option>
            <option value="low">低</option>
          </select>
        </label>
      </div>

      <div class="form-two-columns">
        <label>
          提出章节
          <select v-model="draft.setupDocumentId" class="compact-select-field">
            <option value="">未设置</option>
            <option v-for="document in documents" :key="document.id" :value="document.id">
              {{ documentTypeLabel(document.type) }} · {{ document.title }}
            </option>
          </select>
        </label>
        <label>
          回收章节
          <select v-model="draft.payoffDocumentId" class="compact-select-field">
            <option value="">未设置</option>
            <option v-for="document in documents" :key="document.id" :value="document.id">
              {{ documentTypeLabel(document.type) }} · {{ document.title }}
            </option>
          </select>
        </label>
      </div>

      <div class="form-two-columns">
        <button
            v-if="draft.setupDocumentId"
            class="secondary-button"
            type="button"
            @click="$emit('open-document', draft.setupDocumentId)"
        >
          打开提出章节
        </button>
        <button
            v-if="draft.payoffDocumentId"
            class="secondary-button"
            type="button"
            @click="$emit('open-document', draft.payoffDocumentId)"
        >
          打开回收章节
        </button>
      </div>

      <div class="form-action-row">
        <button :disabled="saving || !draft.title.trim()" class="primary-button" type="submit">
          {{ saving ? '保存中…' : '保存线程' }}
        </button>
        <button
            v-if="draft.id && draft.status !== 'paid_off'"
            :disabled="saving"
            class="secondary-button"
            type="button"
            @click="$emit('mark-paid-off', draft.id, normalizeId(draft.payoffDocumentId))"
        >
          标记已回收
        </button>
        <button
            v-if="draft.id"
            :disabled="saving"
            class="danger-button"
            type="button"
            @click="requestDelete"
        >
          删除
        </button>
      </div>
    </form>
  </section>
</template>

<script lang="ts" setup>
import type {NovelDocument} from '~/types/document'
import type {CreativeNote} from '~/types/note'
import type {CreateForeshadowThreadInput, ForeshadowThread, UpdateForeshadowThreadInput} from '~/types/foreshadow'

const props = defineProps<{
  projectId: string
  thread?: ForeshadowThread | null
  documents: NovelDocument[]
  foreshadowNotes: CreativeNote[]
  saving?: boolean
}>()

const emit = defineEmits<{
  create: [payload: CreateForeshadowThreadInput]
  update: [payload: UpdateForeshadowThreadInput]
  delete: [threadId: string]
  'create-from-note': [noteId: string]
  'mark-paid-off': [threadId: string, payoffDocumentId?: string | null]
  'open-document': [documentId: string]
}>()

const selectedNoteId = ref('')
const draft = reactive({
  id: '',
  title: '',
  description: '',
  status: 'open',
  priority: 'normal',
  setupNoteId: '',
  payoffNoteId: '',
  setupDocumentId: '',
  payoffDocumentId: ''
})

watch(() => props.thread, thread => {
  if (thread) fillDraft(thread)
  else resetDraft()
}, {immediate: true})

function fillDraft(thread: ForeshadowThread) {
  draft.id = thread.id
  draft.title = thread.title
  draft.description = thread.description
  draft.status = thread.status || 'open'
  draft.priority = thread.priority || 'normal'
  draft.setupNoteId = thread.setupNoteId ?? ''
  draft.payoffNoteId = thread.payoffNoteId ?? ''
  draft.setupDocumentId = thread.setupDocumentId ?? ''
  draft.payoffDocumentId = thread.payoffDocumentId ?? ''
}

function resetDraft() {
  draft.id = ''
  draft.title = '新的伏笔线程'
  draft.description = ''
  draft.status = 'open'
  draft.priority = 'normal'
  draft.setupNoteId = ''
  draft.payoffNoteId = ''
  draft.setupDocumentId = ''
  draft.payoffDocumentId = ''
}

function submit() {
  if (draft.id) {
    emit('update', {
      threadId: draft.id,
      title: draft.title,
      description: draft.description,
      status: draft.status,
      priority: draft.priority,
      setupNoteId: normalizeId(draft.setupNoteId),
      payoffNoteId: normalizeId(draft.payoffNoteId),
      setupDocumentId: normalizeId(draft.setupDocumentId),
      payoffDocumentId: normalizeId(draft.payoffDocumentId)
    })
    return
  }

  emit('create', {
    projectId: props.projectId,
    title: draft.title,
    description: draft.description,
    status: draft.status,
    priority: draft.priority,
    setupNoteId: normalizeId(draft.setupNoteId),
    payoffNoteId: normalizeId(draft.payoffNoteId),
    setupDocumentId: normalizeId(draft.setupDocumentId),
    payoffDocumentId: normalizeId(draft.payoffDocumentId)
  })
}

function createFromSelectedNote() {
  if (!selectedNoteId.value) return
  emit('create-from-note', selectedNoteId.value)
  selectedNoteId.value = ''
}

function requestDelete() {
  if (!draft.id) return
  if (confirm('确定删除这个伏笔线程吗？')) emit('delete', draft.id)
}

function normalizeId(value?: string | null) {
  return value && value.trim() ? value.trim() : null
}

function documentTypeLabel(type: string) {
  switch (type) {
    case 'volume':
      return '卷'
    case 'scene':
      return '场景'
    case 'chapter':
      return '章节'
    default:
      return '文档'
  }
}
</script>
