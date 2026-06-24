<template>
  <section class="project-dashboard glass-panel">
    <header class="workspace-section-header project-dashboard-header">
      <div>
        <p class="eyebrow">Project Dashboard</p>
        <h2>项目概览</h2>
        <p>集中管理作品信息、目标、进度、备份和常用入口。</p>
      </div>
      <div class="dashboard-header-actions">
        <button class="secondary-button" type="button" @click="$emit('backup')">手动备份</button>
        <button class="danger-button" type="button" @click="confirmDelete">删除项目</button>
      </div>
    </header>

    <ErrorBanner :message="errorMessage" title="项目操作失败" @dismiss="errorMessage = null"/>

    <div class="dashboard-grid">
      <article class="dashboard-card dashboard-card-wide">
        <h3>作品信息</h3>
        <form class="project-form-grid" @submit.prevent="save">
          <label>
            标题
            <input v-model.trim="draft.title" class="form-field" required>
          </label>
          <label>
            作者 / 笔名
            <input v-model.trim="draft.author" class="form-field">
          </label>
          <label>
            语言
            <select v-model="draft.language" class="form-field">
              <option value="zh-CN">简体中文</option>
              <option value="en">English</option>
            </select>
          </label>
          <label>
            状态
            <select v-model="draft.status" class="form-field">
              <option value="active">进行中</option>
              <option value="archived">已归档</option>
            </select>
          </label>
          <label>
            总目标字数
            <input v-model.number="draft.targetCharacterCount" class="form-field" min="0" type="number">
          </label>
          <label>
            每日目标字数
            <input v-model.number="draft.dailyTargetCount" class="form-field" min="0" type="number">
          </label>
          <label class="project-form-full">
            简介
            <textarea v-model.trim="draft.description" class="form-field min-h-32 resize-none"></textarea>
          </label>
          <div class="project-form-actions project-form-full">
            <button :disabled="saving" class="primary-button" type="submit">
              {{ saving ? '保存中…' : '保存项目信息' }}
            </button>
            <button class="secondary-button" type="button" @click="resetDraft">重置</button>
          </div>
        </form>
      </article>

      <article class="dashboard-card">
        <h3>写作进度</h3>
        <dl class="dashboard-stat-list">
          <div>
            <dt>项目总字数</dt>
            <dd>{{ stats?.characterCount ?? 0 }} 字</dd>
          </div>
          <div>
            <dt>文档数量</dt>
            <dd>{{ stats?.documentCount ?? 0 }} 个</dd>
          </div>
          <div>
            <dt>总目标</dt>
            <dd>{{ project.targetCharacterCount ?? '未设置' }}{{ project.targetCharacterCount ? ' 字' : '' }}</dd>
          </div>
          <div>
            <dt>每日目标</dt>
            <dd>{{ project.dailyTargetCount ?? '未设置' }}{{ project.dailyTargetCount ? ' 字' : '' }}</dd>
          </div>
        </dl>
      </article>

      <article class="dashboard-card">
        <h3>快速入口</h3>
        <div class="dashboard-action-grid">
          <button class="secondary-button" type="button" @click="$emit('openMode', 'writing')">继续写作</button>
          <button class="secondary-button" type="button" @click="$emit('openMode', 'cards')">管理设定</button>
          <button class="secondary-button" type="button" @click="$emit('openMode', 'notes')">查看事项</button>
          <button class="secondary-button" type="button" @click="$emit('openMode', 'search')">全文搜索</button>
          <button class="secondary-button" type="button" @click="$emit('openMode', 'export')">导入导出</button>
          <button class="secondary-button" type="button" @click="$emit('openMode', 'stats')">查看统计</button>
        </div>
      </article>

      <article class="dashboard-card dashboard-card-wide">
        <div class="dashboard-card-title-row">
          <h3>最近备份</h3>
          <span>{{ backups.length }} 个</span>
        </div>
        <EmptyState v-if="backups.length === 0" description="点击“手动备份”保存一份本地项目副本。" title="暂无备份"/>
        <div v-else class="dashboard-backup-list">
          <article v-for="backup in backups.slice(0, 5)" :key="backup.id" class="backup-item">
            <strong>{{ formatDate(backup.createdAt) }}</strong>
            <span>{{ formatSize(backup.sizeBytes) }}</span>
            <small>本地备份</small>
          </article>
        </div>
      </article>
    </div>
  </section>
</template>

<script lang="ts" setup>
import ErrorBanner from '~/components/common/ErrorBanner.vue'
import EmptyState from '~/components/common/EmptyState.vue'
import {toAppErrorMessage} from '~/utils/appError'
import type {Project, ProjectStatus, UpdateProjectInput} from '~/types/project'
import type {ProjectStats} from '~/types/stats'
import type {BackupItem} from '~/types/export'

const props = defineProps<{
  project: Project
  stats: ProjectStats | null
  backups: BackupItem[]
  saving?: boolean
}>()

const emit = defineEmits<{
  updateProject: [payload: UpdateProjectInput]
  deleteProject: []
  backup: []
  openMode: [mode: import('~/constants/projectViews').ProjectWorkspaceMode]
}>()

const errorMessage = ref<string | null>(null)
const draft = reactive({
  title: '',
  author: '',
  description: '',
  language: 'zh-CN',
  status: 'active' as ProjectStatus,
  targetCharacterCount: null as number | null,
  dailyTargetCount: null as number | null
})

watch(() => props.project, resetDraft, {immediate: true})

function resetDraft() {
  draft.title = props.project.title
  draft.author = props.project.author ?? ''
  draft.description = props.project.description ?? ''
  draft.language = props.project.language || 'zh-CN'
  draft.status = props.project.status === 'archived' ? 'archived' : 'active'
  draft.targetCharacterCount = props.project.targetCharacterCount ?? null
  draft.dailyTargetCount = props.project.dailyTargetCount ?? null
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
      language: draft.language || 'zh-CN',
      status: draft.status,
      targetCharacterCount: normalizeOptionalCount(draft.targetCharacterCount),
      dailyTargetCount: normalizeOptionalCount(draft.dailyTargetCount)
    })
  } catch (error) {
    errorMessage.value = toAppErrorMessage(error, '保存项目信息失败')
  }
}

function confirmDelete() {
  const confirmed = window.confirm(`确定删除项目“${props.project.title}”吗？此操作会删除项目内的文档、设定和事项。`)
  if (confirmed) emit('deleteProject')
}

function formatDate(timestamp: number) {
  return new Intl.DateTimeFormat('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(timestamp))
}

function formatSize(size: number) {
  if (size > 1024 * 1024) return `${(size / 1024 / 1024).toFixed(1)} MB`
  if (size > 1024) return `${(size / 1024).toFixed(1)} KB`
  return `${size} B`
}
</script>
