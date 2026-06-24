<template>
  <section class="note-workspace">
    <header class="note-workspace-header paper-card">
      <div>
        <p class="note-workspace-kicker">写作工具</p>
        <h1>创作事项</h1>
        <p>统一管理备忘、待办、伏笔、问题和灵感，可绑定项目、章节或段落。</p>
      </div>
      <div class="note-workspace-actions">
        <button class="secondary-button" type="button" @click="createTemplate('memo')">备忘</button>
        <button class="secondary-button" type="button" @click="createTemplate('todo')">待办</button>
        <button class="primary-button" type="button" @click="createTemplate('foreshadow')">伏笔</button>
      </div>
    </header>

    <div class="note-workspace-grid">
      <NoteList
          :active-note-id="noteStore.activeNoteId"
          :counts="noteStore.counts"
          :notes="noteStore.filteredNotes"
          :status-filter="noteStore.statusFilter"
          :type-filter="noteStore.typeFilter"
          @select="noteStore.selectNote"
          @update:type-filter="updateTypeFilter"
          @update:status-filter="updateStatusFilter"
      />

      <NoteEditor
          :documents="documentStore.documents"
          :note="editorNote"
          @delete="deleteNote"
          @done="markDone"
          @new="createTemplate('memo')"
          @save="saveNote"
      />
    </div>

    <p v-if="errorMessage" class="editor-error">{{ errorMessage }}</p>
  </section>
</template>

<script lang="ts" setup>
import NoteList from '~/components/notes/NoteList.vue'
import NoteEditor from '~/components/notes/NoteEditor.vue'
import type {CreativeNote, NoteStatus, NoteType} from '~/types/note'

const props = defineProps<{
  projectId: string
  activeDocumentId?: string | null
}>()

const noteStore = useNoteStore()
const documentStore = useDocumentStore()
const errorMessage = ref<string | null>(null)
const draftNote = ref<CreativeNote | null>(null)

const editorNote = computed(() => draftNote.value ?? noteStore.activeNote)

watch(() => props.projectId, async projectId => {
  if (!projectId) return
  await loadNotes()
}, {immediate: true})

watch(() => noteStore.activeNoteId, () => {
  draftNote.value = null
})

async function loadNotes() {
  errorMessage.value = null
  try {
    await noteStore.loadProjectNotes(props.projectId)
  } catch (error) {
    errorMessage.value = errorText(error, '加载事项失败')
  }
}

async function updateTypeFilter(type: NoteType) {
  noteStore.setTypeFilter(type)
  await loadNotes()
}

async function updateStatusFilter(status: NoteStatus) {
  noteStore.setStatusFilter(status)
  await loadNotes()
}

function createTemplate(type: Exclude<NoteType, 'all'>) {
  const now = Date.now()
  draftNote.value = {
    id: '',
    projectId: props.projectId,
    documentId: props.activeDocumentId ?? null,
    documentTitle: documentStore.activeDocument?.title ?? null,
    paragraphId: null,
    type,
    title: defaultTitle(type),
    body: '',
    status: 'open',
    priority: type === 'foreshadow' ? 'high' : 'normal',
    dueAt: null,
    createdAt: now,
    updatedAt: now
  }
  noteStore.selectNote(null)
}

async function saveNote(note: CreativeNote) {
  errorMessage.value = null
  try {
    if (note.id) {
      await noteStore.updateNote({
        noteId: note.id,
        documentId: note.documentId ?? null,
        paragraphId: note.paragraphId ?? null,
        type: note.type,
        title: note.title,
        body: note.body,
        status: note.status,
        priority: note.priority,
        dueAt: note.dueAt ?? null
      })
    } else {
      await noteStore.createNote({
        projectId: props.projectId,
        documentId: note.documentId ?? null,
        paragraphId: note.paragraphId ?? null,
        type: note.type,
        title: note.title,
        body: note.body,
        priority: note.priority,
        dueAt: note.dueAt ?? null
      })
      draftNote.value = null
    }
    await loadNotes()
  } catch (error) {
    errorMessage.value = errorText(error, '保存事项失败')
  }
}

async function markDone(noteId: string) {
  errorMessage.value = null
  try {
    await noteStore.updateNoteStatus(noteId, 'done')
    await loadNotes()
  } catch (error) {
    errorMessage.value = errorText(error, '更新事项状态失败')
  }
}

async function deleteNote(noteId: string) {
  if (!confirm('确定删除这个事项吗？')) return
  errorMessage.value = null
  try {
    await noteStore.deleteNote(noteId)
    await loadNotes()
  } catch (error) {
    errorMessage.value = errorText(error, '删除事项失败')
  }
}

function defaultTitle(type: string) {
  switch (type) {
    case 'todo':
      return '新的待办'
    case 'foreshadow':
      return '新的伏笔'
    case 'issue':
      return '新的问题'
    case 'idea':
      return '新的灵感'
    default:
      return '新的备忘'
  }
}

function errorText(error: unknown, fallback: string) {
  return typeof error === 'object' && error && 'message' in error
      ? String((error as { message?: string }).message)
      : fallback
}
</script>
