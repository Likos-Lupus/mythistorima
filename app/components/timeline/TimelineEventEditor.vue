<template>
  <section class="timeline-editor-panel paper-card">
    <header class="timeline-panel-header">
      <div>
        <p class="timeline-panel-kicker">Event Editor</p>
        <h2>{{ event ? '编辑事件' : '新建事件' }}</h2>
      </div>
      <button v-if="event" class="tree-danger-button" type="button" @click="confirmDelete">删除</button>
    </header>

    <form class="timeline-editor-form" @submit.prevent="submit">
      <label class="card-form-row">
        事件标题
        <input v-model="form.title" class="form-field" maxlength="160" placeholder="例如：北塔夜袭" type="text">
      </label>

      <div class="timeline-form-grid">
        <label class="card-form-row">
          显示开始时间
          <input v-model="form.startsAtLabel" class="form-field" placeholder="例如：王历 12 年 初冬" type="text">
        </label>
        <label class="card-form-row">
          显示结束时间
          <input v-model="form.endsAtLabel" class="form-field" placeholder="可选" type="text">
        </label>
      </div>

      <label class="card-form-row">
        sort_key
        <input v-model.number="form.sortKey" class="form-field" min="0" step="10" type="number">
      </label>

      <label class="card-form-row">
        绑定章节 / 场景
        <select v-model="form.linkedDocumentId" class="form-field">
          <option value="">不绑定</option>
          <option v-for="document in documents" :key="document.id" :value="document.id">
            {{ documentTypeLabel(document.type) }} · {{ document.title }}
          </option>
        </select>
      </label>

      <label class="card-form-row">
        绑定地点
        <select v-model="form.locationCardId" class="form-field">
          <option value="">不绑定地点</option>
          <option v-for="card in locationCards" :key="card.id" :value="card.id">
            {{ card.name }}
          </option>
        </select>
      </label>

      <label class="card-form-row">
        事件主卡
        <select v-model="form.linkedCardId" class="form-field">
          <option value="">不绑定事件设定卡</option>
          <option v-for="card in eventCards" :key="card.id" :value="card.id">
            {{ card.name }}
          </option>
        </select>
      </label>

      <label class="card-form-row card-form-row-full">
        事件说明
        <textarea
            v-model="form.description"
            class="form-field card-textarea"
            placeholder="写下事件因果、冲突、转折和对角色的影响。"
            rows="6"
        ></textarea>
      </label>

      <p v-if="error" class="editor-error">{{ error }}</p>

      <div class="timeline-editor-actions">
        <button :disabled="saving" class="primary-button" type="submit">
          {{ saving ? '保存中…' : event ? '保存事件' : '创建事件' }}
        </button>
        <button class="secondary-button" type="button" @click="resetForm">重置</button>
      </div>
    </form>
  </section>
</template>

<script lang="ts" setup>
import type {SettingCard} from '~/types/card'
import type {NovelDocument} from '~/types/document'
import type {CreateTimelineEventInput, TimelineEvent, UpdateTimelineEventInput} from '~/types/timeline'

const props = defineProps<{
  projectId: string
  event: TimelineEvent | null
  cards: SettingCard[]
  documents: NovelDocument[]
  saving?: boolean
}>()

const emit = defineEmits<{
  create: [payload: CreateTimelineEventInput]
  update: [payload: UpdateTimelineEventInput]
  delete: [timelineEventId: string]
}>()

const error = ref<string | null>(null)
const form = reactive({
  title: '',
  description: '',
  startsAtLabel: '',
  endsAtLabel: '',
  sortKey: 0,
  linkedDocumentId: '',
  locationCardId: '',
  linkedCardId: ''
})

const locationCards = computed(() => props.cards.filter(card => card.type === 'location'))
const eventCards = computed(() => props.cards.filter(card => card.type === 'event'))

watch(() => props.event, () => resetForm(), {immediate: true})

function resetForm() {
  error.value = null
  if (props.event) {
    form.title = props.event.title
    form.description = props.event.description
    form.startsAtLabel = props.event.startsAtLabel ?? ''
    form.endsAtLabel = props.event.endsAtLabel ?? ''
    form.sortKey = props.event.sortKey
    form.linkedDocumentId = props.event.linkedDocumentId ?? ''
    form.locationCardId = props.event.locationCardId ?? ''
    form.linkedCardId = props.event.linkedCardId ?? ''
    return
  }

  form.title = '新的时间线事件'
  form.description = ''
  form.startsAtLabel = ''
  form.endsAtLabel = ''
  form.sortKey = 0
  form.linkedDocumentId = ''
  form.locationCardId = ''
  form.linkedCardId = ''
}

function submit() {
  error.value = null
  if (!form.title.trim()) {
    error.value = '事件标题不能为空。'
    return
  }

  const payload = {
    linkedCardId: form.linkedCardId || null,
    linkedDocumentId: form.linkedDocumentId || null,
    title: form.title.trim(),
    description: form.description.trim(),
    startsAtLabel: form.startsAtLabel.trim() || null,
    endsAtLabel: form.endsAtLabel.trim() || null,
    sortKey: Number.isFinite(Number(form.sortKey)) ? Math.max(0, Math.round(Number(form.sortKey))) : 0,
    locationCardId: form.locationCardId || null,
    metadataJson: '{}'
  }

  if (props.event) {
    emit('update', {
      timelineEventId: props.event.id,
      ...payload
    })
  } else {
    emit('create', {
      projectId: props.projectId,
      ...payload
    })
  }
}

function confirmDelete() {
  if (!props.event) return
  if (confirm('删除这个时间线事件？参与者绑定也会一起删除。')) {
    emit('delete', props.event.id)
  }
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
