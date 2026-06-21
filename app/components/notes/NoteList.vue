<template>
  <section class="note-list-panel glass-panel">
    <div class="note-filter-row">
      <button
          v-for="item in typeTabs"
          :key="item.value"
          :class="['note-filter-button', { 'is-active': typeFilter === item.value }]"
          type="button"
          @click="$emit('update:typeFilter', item.value)"
      >
        {{ item.label }} <strong>{{ counts[item.value] ?? 0 }}</strong>
      </button>
    </div>

    <div class="note-filter-row compact">
      <button
          v-for="item in statusTabs"
          :key="item.value"
          :class="['note-filter-button', { 'is-active': statusFilter === item.value }]"
          type="button"
          @click="$emit('update:statusFilter', item.value)"
      >
        {{ item.label }}
      </button>
    </div>

    <div v-if="notes.length === 0" class="empty-panel">
      暂无创作事项。可以先创建一个章节待办，或在写作时给段落添加伏笔。
    </div>

    <div v-else class="note-list">
      <button
          v-for="note in notes"
          :key="note.id"
          :class="['note-list-item', { 'is-active': note.id === activeNoteId }]"
          type="button"
          @click="$emit('select', note.id)"
      >
        <span :class="['note-type-pill', `is-${note.type}`]">{{ noteTypeLabel(note.type) }}</span>
        <strong class="note-list-title">{{ note.title }}</strong>
        <span class="note-list-meta">
          {{ noteStatusLabel(note.status) }} · {{ notePriorityLabel(note.priority) }}
          <template v-if="note.documentTitle"> · {{ note.documentTitle }}</template>
          <template v-if="note.paragraphId"> · 段落</template>
        </span>
        <span v-if="note.body" class="note-list-body">{{ note.body }}</span>
      </button>
    </div>
  </section>
</template>

<script lang="ts" setup>
import type {CreativeNote, NoteStatus, NoteType} from '~/types/note'

defineProps<{
  notes: CreativeNote[]
  activeNoteId: string | null
  typeFilter: NoteType
  statusFilter: NoteStatus
  counts: Record<string, number>
}>()

defineEmits<{
  select: [noteId: string]
  'update:typeFilter': [value: NoteType]
  'update:statusFilter': [value: NoteStatus]
}>()

const typeTabs: Array<{ value: NoteType, label: string }> = [
  {value: 'all', label: '全部'},
  {value: 'memo', label: '备忘'},
  {value: 'todo', label: '待办'},
  {value: 'foreshadow', label: '伏笔'},
  {value: 'issue', label: '问题'},
  {value: 'idea', label: '灵感'}
]

const statusTabs: Array<{ value: NoteStatus, label: string }> = [
  {value: 'all', label: '全部状态'},
  {value: 'open', label: '未完成'},
  {value: 'doing', label: '进行中'},
  {value: 'done', label: '已完成'}
]

function noteTypeLabel(type: string) {
  switch (type) {
    case 'memo':
      return '备忘'
    case 'todo':
      return '待办'
    case 'foreshadow':
      return '伏笔'
    case 'issue':
      return '问题'
    case 'idea':
      return '灵感'
    default:
      return type
  }
}

function noteStatusLabel(status: string) {
  switch (status) {
    case 'open':
      return '未完成'
    case 'doing':
      return '进行中'
    case 'done':
      return '已完成'
    case 'archived':
      return '归档'
    default:
      return status
  }
}

function notePriorityLabel(priority: string) {
  switch (priority) {
    case 'high':
      return '高优先级'
    case 'low':
      return '低优先级'
    default:
      return '普通'
  }
}
</script>
