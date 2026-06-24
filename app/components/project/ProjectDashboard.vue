<template>
  <section class="project-overview">
    <UPageHeader
        :description="overviewDescription"
        :headline="metadata.genre || '小说项目'"
        :title="project.title"
    >
      <template #links>
        <UBadge :color="project.status === 'archived' ? 'neutral' : 'success'" variant="subtle">
          {{ project.status === 'archived' ? '已归档' : '进行中' }}
        </UBadge>
        <UButton
            color="neutral"
            icon="i-lucide-pencil"
            label="编辑项目"
            size="sm"
            variant="soft"
            @click="editing = !editing"
        />
        <UDropdownMenu :items="projectMenu">
          <UButton
              aria-label="更多项目操作"
              color="neutral"
              icon="i-lucide-ellipsis"
              size="sm"
              square
              variant="ghost"
          />
        </UDropdownMenu>
      </template>
    </UPageHeader>

    <UAlert
        v-if="errorMessage"
        :description="errorMessage"
        close
        color="error"
        icon="i-lucide-circle-alert"
        title="项目操作失败"
        variant="subtle"
        @close="errorMessage = null"
    />

    <UCard v-if="editing" :ui="{ body: 'p-4 sm:p-5' }">
      <UForm :state="draft" class="project-overview-form" @submit="save">
        <div class="project-overview-form-heading">
          <div>
            <h2>项目信息与设置</h2>
            <p>基础资料、创作 metadata、目标和项目状态。</p>
          </div>
          <UButton color="neutral" label="取消编辑" size="sm" variant="ghost" @click="cancelEditing"/>
        </div>

        <div class="project-overview-form-grid">
          <UFormField label="标题" name="title" required>
            <UInput v-model="draft.title" class="w-full" size="sm"/>
          </UFormField>
          <UFormField label="作者 / 笔名" name="author">
            <UInput v-model="draft.author" class="w-full" size="sm"/>
          </UFormField>
          <UFormField label="状态" name="status">
            <USelect v-model="draft.status" :items="statusOptions" class="w-full" label-key="label" size="sm"
                     value-key="value"/>
          </UFormField>
          <UFormField label="语言" name="language">
            <USelect v-model="draft.language" :items="languageOptions" class="w-full" label-key="label" size="sm"
                     value-key="value"/>
          </UFormField>
          <UFormField label="题材 / 类型" name="genre">
            <UInput v-model="draft.metadata.genre" class="w-full" placeholder="奇幻、悬疑、科幻…" size="sm"/>
          </UFormField>
          <UFormField label="目标读者" name="audience">
            <UInput v-model="draft.metadata.audience" class="w-full" placeholder="例如：青年读者" size="sm"/>
          </UFormField>
          <UFormField label="叙事视角" name="pointOfView">
            <USelect v-model="draft.metadata.pointOfView" :items="pointOfViewOptions" class="w-full" size="sm"/>
          </UFormField>
          <UFormField label="叙事时态" name="tense">
            <USelect v-model="draft.metadata.tense" :items="tenseOptions" class="w-full" size="sm"/>
          </UFormField>
          <UFormField label="总目标字数" name="targetCharacterCount">
            <UInputNumber v-model="draft.targetCharacterCount" :min="0" :step="1000" class="w-full" size="sm"/>
          </UFormField>
          <UFormField label="每日目标字数" name="dailyTargetCount">
            <UInputNumber v-model="draft.dailyTargetCount" :min="0" :step="100" class="w-full" size="sm"/>
          </UFormField>
          <UFormField class="project-overview-form-wide" label="标签" name="tags">
            <UInputTags v-model="draft.metadata.tags" class="w-full" placeholder="输入标签后回车" size="sm"/>
          </UFormField>
          <UFormField class="project-overview-form-wide" label="一句话梗概" name="premise">
            <UTextarea v-model="draft.metadata.premise" :rows="2" autoresize class="w-full" size="sm"/>
          </UFormField>
          <UFormField class="project-overview-form-wide" label="项目简介" name="description">
            <UTextarea v-model="draft.description" :rows="3" autoresize class="w-full" size="sm"/>
          </UFormField>
          <UFormField class="project-overview-form-wide" label="创作备注" name="notes">
            <UTextarea v-model="draft.metadata.notes" :rows="3" autoresize class="w-full" size="sm"/>
          </UFormField>
        </div>

        <div class="project-overview-form-actions">
          <UButton color="neutral" label="重置" size="sm" variant="ghost" @click="resetDraft"/>
          <UButton :loading="saving" icon="i-lucide-save" label="保存项目" size="sm" type="submit"/>
        </div>
      </UForm>
    </UCard>

    <div class="project-overview-metrics">
      <div class="project-overview-metric">
        <span>今日字数</span>
        <strong>{{ formatNumber(overview?.todayCharacterCount ?? 0) }}</strong>
        <small>{{ formatDuration(overview?.todayElapsedMs ?? 0) }}</small>
      </div>
      <div class="project-overview-metric">
        <span>项目总字数</span>
        <strong>{{ formatNumber(overview?.characterCount ?? stats?.characterCount ?? 0) }}</strong>
        <small>{{ overview?.documentCount ?? stats?.documentCount ?? 0 }} 个文档</small>
      </div>
      <div class="project-overview-metric">
        <span>完成度</span>
        <strong>{{ completionPercent }}%</strong>
        <UProgress :model-value="completionPercent" size="xs"/>
      </div>
      <div class="project-overview-metric">
        <span>连续写作</span>
        <strong>{{ overview?.writingStreakDays ?? 0 }} 天</strong>
        <small>{{ completedDocumentsLabel }}</small>
      </div>
    </div>

    <div class="project-overview-grid">
      <UCard :ui="{ body: 'p-0' }">
        <template #header>
          <OverviewCardHeader icon="i-lucide-history" title="最近文档"/>
        </template>
        <OverviewEmpty v-if="!overview?.recentDocuments.length" title="还没有最近文档"/>
        <div v-else class="project-overview-list">
          <UButton
              v-for="document in overview.recentDocuments"
              :key="document.id"
              class="project-overview-list-row"
              color="neutral"
              variant="ghost"
              @click="$emit('openTarget', {type: 'document', targetId: document.id, source: 'direct'})"
          >
            <UIcon :name="document.documentType === 'scene' ? 'i-lucide-clapperboard' : 'i-lucide-file-text'"/>
            <span>
              <strong>{{ document.title }}</strong>
              <small>{{ formatNumber(document.characterCount) }} 字 · {{
                  formatRelativeDate(document.updatedAt)
                }}</small>
            </span>
            <UBadge color="neutral" size="sm" variant="subtle">{{ documentStatusLabel(document.status) }}</UBadge>
          </UButton>
        </div>
      </UCard>

      <UCard :ui="{ body: 'p-0' }">
        <template #header>
          <OverviewCardHeader icon="i-lucide-list-checks" title="未完成事项"/>
        </template>
        <OverviewEmpty v-if="!overview?.openNotes.length" title="没有未完成事项"/>
        <div v-else class="project-overview-list">
          <UButton
              v-for="note in overview.openNotes"
              :key="note.id"
              class="project-overview-list-row"
              color="neutral"
              variant="ghost"
              @click="$emit('openTarget', {type: 'note', targetId: note.id})"
          >
            <UIcon name="i-lucide-square-check-big"/>
            <span>
              <strong>{{ note.title }}</strong>
              <small>{{ note.documentTitle || '项目级事项' }} · {{ noteTypeLabel(note.noteType) }}</small>
            </span>
            <UBadge :color="note.priority === 'high' ? 'warning' : 'neutral'" size="sm" variant="subtle">
              {{ priorityLabel(note.priority) }}
            </UBadge>
          </UButton>
        </div>
      </UCard>

      <UCard :ui="{ body: 'p-0' }">
        <template #header>
          <OverviewCardHeader icon="i-lucide-sparkles" title="未回收伏笔"/>
        </template>
        <OverviewEmpty v-if="!overview?.unresolvedForeshadows.length" title="没有未回收伏笔"/>
        <div v-else class="project-overview-list">
          <UButton
              v-for="thread in overview.unresolvedForeshadows"
              :key="thread.id"
              class="project-overview-list-row"
              color="neutral"
              variant="ghost"
              @click="$emit('openTarget', {type: 'foreshadow', targetId: thread.id})"
          >
            <UIcon name="i-lucide-sparkles"/>
            <span>
              <strong>{{ thread.title }}</strong>
              <small>{{ thread.setupDocumentTitle || '尚未绑定提出章节' }}</small>
            </span>
            <UBadge :color="thread.priority === 'high' ? 'warning' : 'neutral'" size="sm" variant="subtle">
              {{ priorityLabel(thread.priority) }}
            </UBadge>
          </UButton>
        </div>
      </UCard>

      <UCard :ui="{ body: 'p-0' }">
        <template #header>
          <OverviewCardHeader icon="i-lucide-users" title="人物出场"/>
        </template>
        <OverviewEmpty v-if="!overview?.topCharacters.length" title="正文中还没有人物引用"/>
        <div v-else class="project-overview-list">
          <UButton
              v-for="character in overview.topCharacters"
              :key="character.cardId"
              class="project-overview-list-row"
              color="neutral"
              variant="ghost"
              @click="$emit('openTarget', {type: 'card', targetId: character.cardId})"
          >
            <UIcon name="i-lucide-user-round"/>
            <span>
              <strong>{{ character.cardName }}</strong>
              <small>{{ character.documentCount }} 个文档</small>
            </span>
            <span class="project-overview-count">{{ character.mentionCount }} 次</span>
          </UButton>
        </div>
      </UCard>
    </div>

    <UCard>
      <template #header>
        <OverviewCardHeader icon="i-lucide-chart-no-axes-column" title="最近 14 天写作趋势"/>
      </template>
      <div aria-label="最近十四天写作字数柱状图" class="project-writing-trend" role="img">
        <div v-for="point in trendPoints" :key="point.dayStart" class="project-writing-trend-day">
          <span class="project-writing-trend-value">{{ point.characterCount || '' }}</span>
          <div class="project-writing-trend-track">
            <div
                :style="{height: `${trendHeight(point.characterCount)}%`}"
                class="project-writing-trend-bar"
            />
          </div>
          <small>{{ formatTrendDate(point.dayStart) }}</small>
        </div>
      </div>
    </UCard>
  </section>
</template>

<script lang="ts" setup>
import {defineComponent, h, resolveComponent} from 'vue'
import type {BackupItem} from '~/types/export'
import type {OpenTarget} from '~/types/navigation'
import {
  emptyProjectMetadata,
  parseProjectMetadata,
  type Project,
  type ProjectMetadata,
  type ProjectStatus,
  serializeProjectMetadata,
  type UpdateProjectInput
} from '~/types/project'
import type {ProjectOverview, ProjectStats} from '~/types/stats'
import {toAppErrorMessage} from '~/utils/appError'

const props = defineProps<{
  project: Project
  stats: ProjectStats | null
  overview: ProjectOverview | null
  backups: BackupItem[]
  saving?: boolean
}>()

const emit = defineEmits<{
  updateProject: [payload: UpdateProjectInput]
  deleteProject: []
  backup: []
  openMode: [mode: import('~/constants/projectViews').ProjectWorkspaceMode]
  openTarget: [target: OpenTarget]
}>()

const editing = ref(false)
const errorMessage = ref<string | null>(null)

const draft = reactive({
  title: '',
  author: '',
  description: '',
  language: 'zh-CN',
  status: 'active' as ProjectStatus,
  targetCharacterCount: null as number | null,
  dailyTargetCount: null as number | null,
  metadata: emptyProjectMetadata() as ProjectMetadata
})

const languageOptions = [
  {label: '简体中文', value: 'zh-CN'},
  {label: 'English', value: 'en'}
]
const statusOptions = [
  {label: '进行中', value: 'active'},
  {label: '已归档', value: 'archived'}
]
const pointOfViewOptions = ['第一人称', '第三人称限知', '第三人称全知', '多视角', '第二人称']
const tenseOptions = ['过去时', '现在时', '混合时态']

const metadata = computed(() => parseProjectMetadata(props.project.metadataJson))
const overviewDescription = computed(() =>
    metadata.value.premise || props.project.description || `${props.project.author || '未填写作者'}的小说项目`
)
const completionPercent = computed(() => {
  const target = props.project.targetCharacterCount ?? 0
  if (!target) return 0
  return Math.min(100, Math.round(((props.overview?.characterCount ?? props.stats?.characterCount ?? 0) / target) * 100))
})
const completedDocumentsLabel = computed(() =>
    `${props.overview?.completedDocumentCount ?? 0} 个文档已完成`
)

const trendPoints = computed(() => {
  const source = new Map((props.overview?.writingTrend ?? []).map(point => [point.dayStart, point]))
  const end = startOfDay(Date.now())
  return Array.from({length: 14}, (_, index) => {
    const dayStart = end - (13 - index) * 86_400_000
    return source.get(dayStart) ?? {dayStart, characterCount: 0, elapsedMs: 0}
  })
})
const trendMaximum = computed(() => Math.max(1, ...trendPoints.value.map(point => point.characterCount)))

const projectMenu = computed(() => [[
  {
    label: '继续写作',
    icon: 'i-lucide-pen-line',
    onSelect: () => emit('openMode', 'writing')
  },
  {
    label: '创建备份',
    icon: 'i-lucide-archive',
    onSelect: () => emit('backup')
  }
], [
  {
    label: '删除项目',
    icon: 'i-lucide-trash-2',
    color: 'error' as const,
    onSelect: confirmDelete
  }
]])

const OverviewCardHeader = defineComponent({
  props: {
    title: {type: String, required: true},
    icon: {type: String, required: true}
  },
  setup(headerProps) {
    return () => h('div', {class: 'project-overview-card-header'}, [
      h(resolveComponent('UIcon'), {name: headerProps.icon}),
      h('h2', headerProps.title)
    ])
  }
})

const OverviewEmpty = defineComponent({
  props: {title: {type: String, required: true}},
  setup(emptyProps) {
    return () => h('div', {class: 'project-overview-empty'}, emptyProps.title)
  }
})

watch(() => props.project, resetDraft, {immediate: true, deep: true})

function resetDraft() {
  draft.title = props.project.title
  draft.author = props.project.author ?? ''
  draft.description = props.project.description ?? ''
  draft.language = props.project.language || 'zh-CN'
  draft.status = props.project.status === 'archived' ? 'archived' : 'active'
  draft.targetCharacterCount = props.project.targetCharacterCount ?? null
  draft.dailyTargetCount = props.project.dailyTargetCount ?? null
  draft.metadata = parseProjectMetadata(props.project.metadataJson)
}

function cancelEditing() {
  resetDraft()
  editing.value = false
}

function normalizeOptionalCount(value: number | null) {
  if (typeof value !== 'number' || !Number.isFinite(value) || value <= 0) return null
  return Math.round(value)
}

function save() {
  errorMessage.value = null
  try {
    if (!draft.title.trim()) throw new Error('项目标题不能为空')
    emit('updateProject', {
      projectId: props.project.id,
      title: draft.title.trim(),
      author: draft.author.trim() || null,
      description: draft.description.trim() || null,
      language: draft.language,
      status: draft.status,
      targetCharacterCount: normalizeOptionalCount(draft.targetCharacterCount),
      dailyTargetCount: normalizeOptionalCount(draft.dailyTargetCount),
      metadataJson: serializeProjectMetadata(draft.metadata)
    })
    editing.value = false
  } catch (error) {
    errorMessage.value = toAppErrorMessage(error, '保存项目信息失败')
  }
}

function confirmDelete() {
  const confirmed = window.confirm(`确定删除项目“${props.project.title}”吗？此操作会删除项目内的全部内容。`)
  if (confirmed) emit('deleteProject')
}

function trendHeight(value: number) {
  if (value <= 0) return 2
  return Math.max(8, Math.round((value / trendMaximum.value) * 100))
}

function startOfDay(timestamp: number) {
  const date = new Date(timestamp)
  return new Date(date.getFullYear(), date.getMonth(), date.getDate()).getTime()
}

function formatTrendDate(timestamp: number) {
  return new Intl.DateTimeFormat('zh-CN', {month: 'numeric', day: 'numeric'}).format(new Date(timestamp))
}

function formatNumber(value: number) {
  return new Intl.NumberFormat('zh-CN').format(value)
}

function formatDuration(ms: number) {
  const minutes = Math.floor(ms / 60_000)
  if (minutes < 60) return `${minutes} 分钟`
  return `${Math.floor(minutes / 60)} 小时 ${minutes % 60} 分`
}

function formatRelativeDate(timestamp: number) {
  const days = Math.floor((Date.now() - timestamp) / 86_400_000)
  if (days <= 0) return '今天'
  if (days === 1) return '昨天'
  if (days < 7) return `${days} 天前`
  return new Intl.DateTimeFormat('zh-CN', {month: '2-digit', day: '2-digit'}).format(new Date(timestamp))
}

function documentStatusLabel(status: string) {
  if (status === 'writing') return '写作中'
  if (status === 'done') return '完成'
  if (status === 'revised') return '已修订'
  return '草稿'
}

function noteTypeLabel(type: string) {
  if (type === 'todo') return '待办'
  if (type === 'foreshadow') return '伏笔'
  if (type === 'idea') return '灵感'
  if (type === 'issue') return '问题'
  return '备忘'
}

function priorityLabel(priority: string) {
  if (priority === 'high') return '高'
  if (priority === 'low') return '低'
  return '普通'
}
</script>
