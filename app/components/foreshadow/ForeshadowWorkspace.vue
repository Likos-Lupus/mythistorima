<template>
  <section class="foreshadow-workspace">
    <header class="foreshadow-workspace-header glass-panel">
      <div>
        <p class="foreshadow-workspace-kicker">写作工具</p>
        <h1>伏笔线程</h1>
        <p>把零散伏笔事项升级为“提出 / 回收 / 状态 / 优先级”可追踪线程。</p>
      </div>
      <div class="foreshadow-workspace-actions">
        <button class="secondary-button" type="button" @click="refresh">刷新</button>
        <button class="primary-button" type="button" @click="startCreate">新建伏笔</button>
      </div>
    </header>

    <ForeshadowStatusBoard
        :active-thread-id="foreshadowStore.activeThreadId"
        :counts="foreshadowStore.counts"
        :only-unpaid="foreshadowStore.onlyUnpaid"
        :priority-filter="foreshadowStore.priorityFilter"
        :status-filter="foreshadowStore.statusFilter"
        :threads="foreshadowStore.threads"
        @select="foreshadowStore.selectThread"
        @update:only-unpaid="updateOnlyUnpaid"
        @update:priority-filter="updatePriorityFilter"
        @update:status-filter="updateStatusFilter"
    />

    <div class="foreshadow-layout">
      <ForeshadowThreadList
          :active-thread-id="foreshadowStore.activeThreadId"
          :loading="foreshadowStore.loading"
          :threads="foreshadowStore.threads"
          @create="startCreate"
          @select="foreshadowStore.selectThread"
      />

      <ForeshadowThreadEditor
          :documents="documents"
          :foreshadow-notes="foreshadowNotes"
          :project-id="projectId"
          :saving="foreshadowStore.saving"
          :thread="foreshadowStore.activeThread"
          @create="createThread"
          @delete="deleteThread"
          @update="updateThread"
          @create-from-note="createThreadFromNote"
          @mark-paid-off="markPaidOff"
          @open-target="$emit('open-target', $event)"
      />
    </div>

    <ErrorBanner :message="error" title="伏笔线程加载失败" @dismiss="error = null"/>
  </section>
</template>

<script lang="ts" setup>
import ForeshadowStatusBoard from '~/components/foreshadow/ForeshadowStatusBoard.vue'
import ForeshadowThreadEditor from '~/components/foreshadow/ForeshadowThreadEditor.vue'
import ForeshadowThreadList from '~/components/foreshadow/ForeshadowThreadList.vue'
import ErrorBanner from '~/components/common/ErrorBanner.vue'
import type {NovelDocument} from '~/types/document'
import type {CreativeNote} from '~/types/note'
import type {
  CreateForeshadowThreadInput,
  ForeshadowPriority,
  ForeshadowStatus,
  UpdateForeshadowThreadInput
} from '~/types/foreshadow'
import {toAppErrorMessage} from '~/utils/appError'

const props = defineProps<{
  projectId: string
  documents: NovelDocument[]
}>()

defineEmits<{
  'open-target': [target: import('~/types/navigation').OpenTarget]
}>()

const foreshadowStore = useForeshadowStore()
const {call} = useTauriInvoke()
const foreshadowNotes = ref<CreativeNote[]>([])
const error = ref<string | null>(null)

watch(() => props.projectId, async projectId => {
  if (projectId) await refresh()
}, {immediate: true})

async function refresh() {
  error.value = null
  try {
    await Promise.all([
      foreshadowStore.loadThreads(props.projectId),
      loadForeshadowNotes()
    ])
  } catch (err) {
    error.value = toAppErrorMessage(err, '加载伏笔线程失败')
  }
}

async function loadForeshadowNotes() {
  foreshadowNotes.value = await call<CreativeNote[]>('list_notes', {
    input: {
      projectId: props.projectId,
      type: 'foreshadow',
      status: 'open'
    }
  })
}

function startCreate() {
  foreshadowStore.selectThread(null)
}

async function updateStatusFilter(status: string) {
  foreshadowStore.statusFilter = status as ForeshadowStatus
  await refresh()
}

async function updatePriorityFilter(priority: string) {
  foreshadowStore.priorityFilter = priority as ForeshadowPriority
  await refresh()
}

async function updateOnlyUnpaid(onlyUnpaid: boolean) {
  foreshadowStore.onlyUnpaid = onlyUnpaid
  await refresh()
}

async function createThread(payload: CreateForeshadowThreadInput) {
  error.value = null
  try {
    await foreshadowStore.createThread(payload)
    await refresh()
  } catch (err) {
    error.value = toAppErrorMessage(err, '创建伏笔线程失败')
  }
}

async function createThreadFromNote(noteId: string) {
  error.value = null
  try {
    await foreshadowStore.createThreadFromNote({noteId})
    await refresh()
  } catch (err) {
    error.value = toAppErrorMessage(err, '从事项生成伏笔线程失败')
  }
}

async function updateThread(payload: UpdateForeshadowThreadInput) {
  error.value = null
  try {
    await foreshadowStore.updateThread(payload)
    await refresh()
  } catch (err) {
    error.value = toAppErrorMessage(err, '保存伏笔线程失败')
  }
}

async function markPaidOff(threadId: string, payoffDocumentId?: string | null) {
  error.value = null
  try {
    await foreshadowStore.markPaidOff(threadId, payoffDocumentId)
    await refresh()
  } catch (err) {
    error.value = toAppErrorMessage(err, '标记伏笔回收失败')
  }
}

async function deleteThread(threadId: string) {
  error.value = null
  try {
    await foreshadowStore.deleteThread(threadId)
    await refresh()
  } catch (err) {
    error.value = toAppErrorMessage(err, '删除伏笔线程失败')
  }
}
</script>
