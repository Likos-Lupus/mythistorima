<template>
  <section class="story-inspector-content">
    <div class="story-panel-header">
      <div>
        <strong>事件详情</strong>
        <span>{{ event ? '编辑时间线事件' : '新建事件' }}</span>
      </div>
    </div>

    <UForm :state="form" class="story-form" @submit="submit">
      <UFormField label="标题" name="title" required>
        <UInput v-model="form.title" class="w-full" placeholder="例如：冬至夜伏击" size="sm"/>
      </UFormField>

      <div class="story-form-grid">
        <UFormField label="开始显示时间" name="startsAtLabel">
          <UInput v-model="form.startsAtLabel" class="w-full" placeholder="三年前" size="sm"/>
        </UFormField>
        <UFormField label="结束显示时间" name="endsAtLabel">
          <UInput v-model="form.endsAtLabel" class="w-full" placeholder="可选" size="sm"/>
        </UFormField>
      </div>

      <UFormField label="排序键" name="sortKey">
        <UInputNumber v-model="form.sortKey" :step="10" class="w-full" size="sm"/>
      </UFormField>

      <UFormField label="绑定章节" name="linkedDocumentId">
        <USelect v-model="form.linkedDocumentId" :items="documentItems" class="w-full" label-key="label" size="sm"
                 value-key="value"/>
      </UFormField>

      <UFormField label="地点设定" name="locationCardId">
        <USelect v-model="form.locationCardId" :items="locationItems" class="w-full" label-key="label" size="sm"
                 value-key="value"/>
      </UFormField>

      <UFormField label="说明" name="description">
        <UTextarea v-model="form.description" :rows="7" autoresize class="w-full" size="sm"/>
      </UFormField>

      <div class="story-inspector-actions">
        <UButton :label="event ? '保存事件' : '创建事件'" :loading="saving" icon="i-lucide-save" size="sm"
                 type="submit"/>
        <UButton v-if="event" color="error" icon="i-lucide-trash-2" label="删除" size="sm" variant="ghost"
                 @click="$emit('delete', event.id)"/>
      </div>
    </UForm>
  </section>
</template>

<script lang="ts" setup>
import type {SettingCard} from '~/types/card'
import type {NovelDocument} from '~/types/document'
import type {CreateTimelineEventInput, TimelineEvent, UpdateTimelineEventInput} from '~/types/timeline'

const props = defineProps<{
  projectId: string
  event: TimelineEvent | null
  documents: NovelDocument[]
  cards: SettingCard[]
  saving?: boolean
}>()

const emit = defineEmits<{
  create: [payload: CreateTimelineEventInput]
  update: [payload: UpdateTimelineEventInput]
  delete: [timelineEventId: string]
}>()

const form = reactive({
  title: '',
  startsAtLabel: '',
  endsAtLabel: '',
  sortKey: 100,
  linkedDocumentId: '__none__',
  locationCardId: '__none__',
  description: ''
})

const documentItems = computed(() => [
  {label: '不绑定章节', value: '__none__'},
  ...props.documents.map(document => ({label: document.title, value: document.id}))
])
const locationItems = computed(() => [
  {label: '不绑定地点', value: '__none__'},
  ...props.cards.filter(card => card.type === 'location').map(card => ({label: card.name, value: card.id}))
])

watch(() => props.event, event => {
  form.title = event?.title ?? ''
  form.startsAtLabel = event?.startsAtLabel ?? ''
  form.endsAtLabel = event?.endsAtLabel ?? ''
  form.sortKey = event?.sortKey ?? 100
  form.linkedDocumentId = event?.linkedDocumentId ?? '__none__'
  form.locationCardId = event?.locationCardId ?? '__none__'
  form.description = event?.description ?? ''
}, {immediate: true})

function nullable(value: string) {
  return value && value !== '__none__' ? value : null
}

function submit() {
  const base = {
    linkedDocumentId: nullable(form.linkedDocumentId),
    locationCardId: nullable(form.locationCardId),
    title: form.title.trim() || '未命名事件',
    description: form.description.trim(),
    startsAtLabel: form.startsAtLabel.trim() || null,
    endsAtLabel: form.endsAtLabel.trim() || null,
    sortKey: form.sortKey,
    metadataJson: '{}'
  }
  if (props.event) {
    emit('update', {timelineEventId: props.event.id, ...base})
  } else {
    emit('create', {projectId: props.projectId, ...base})
  }
}
</script>
