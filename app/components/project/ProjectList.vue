<template>
  <div class="project-compact-list">
    <template v-if="loading">
      <div v-for="index in 5" :key="index" class="project-compact-skeleton">
        <USkeleton class="h-9 w-9 rounded-md"/>
        <div class="min-w-0 flex-1 space-y-2">
          <USkeleton class="h-4 w-1/3"/>
          <USkeleton class="h-3 w-2/3"/>
        </div>
      </div>
    </template>

    <div
        v-for="project in projects"
        v-else
        :key="project.id"
        class="project-compact-row"
    >
      <UButton
          class="project-compact-main"
          color="neutral"
          variant="ghost"
          @click="$emit('open', project.id)"
      >
        <div class="project-compact-icon">
          <UIcon name="i-lucide-book-open-text"/>
        </div>

        <div class="project-compact-copy">
          <div class="project-compact-title-line">
            <strong>{{ project.title }}</strong>
            <UBadge
                :color="project.status === 'archived' ? 'neutral' : 'success'"
                size="sm"
                variant="subtle"
            >
              {{ project.status === 'archived' ? '已归档' : '进行中' }}
            </UBadge>
          </div>
          <p>
            {{ project.author || '未填写作者' }}
            <span>·</span>
            {{ statsById[project.id]?.documentCount ?? 0 }} 个文档
            <span>·</span>
            {{ formatCount(statsById[project.id]?.characterCount ?? 0) }} 字
            <span>·</span>
            {{ formatRelativeDate(project.updatedAt) }}
          </p>
        </div>
      </UButton>

      <UDropdownMenu :items="projectMenu(project)">
        <UButton
            aria-label="项目操作"
            color="neutral"
            icon="i-lucide-ellipsis"
            size="sm"
            square
            variant="ghost"
        />
      </UDropdownMenu>
    </div>

    <UEmpty
        v-if="!loading && !projects.length"
        description="创建一个项目，开始规划和写作。"
        icon="i-lucide-library-big"
        title="还没有项目"
    />
  </div>
</template>

<script lang="ts" setup>
import type {Project} from '~/types/project'
import type {ProjectStats} from '~/types/stats'

withDefaults(defineProps<{
  projects: Project[]
  statsById?: Record<string, ProjectStats>
  loading?: boolean
}>(), {
  statsById: () => ({}),
  loading: false
})

const emit = defineEmits<{
  open: [projectId: string]
  rename: [project: Project]
  backup: [projectId: string]
  delete: [projectId: string]
}>()

function projectMenu(project: Project) {
  return [[
    {
      label: '打开',
      icon: 'i-lucide-folder-open',
      onSelect: () => emit('open', project.id)
    },
    {
      label: '重命名',
      icon: 'i-lucide-pencil',
      onSelect: () => emit('rename', project)
    },
    {
      label: '创建备份',
      icon: 'i-lucide-archive',
      onSelect: () => emit('backup', project.id)
    }
  ], [
    {
      label: '删除项目',
      icon: 'i-lucide-trash-2',
      color: 'error' as const,
      onSelect: () => emit('delete', project.id)
    }
  ]]
}

function formatCount(value: number) {
  return new Intl.NumberFormat('zh-CN').format(value)
}

function formatRelativeDate(timestamp: number) {
  const diff = Date.now() - timestamp
  const minute = 60_000
  const hour = 60 * minute
  const day = 24 * hour

  if (diff < minute) return '刚刚'
  if (diff < hour) return `${Math.floor(diff / minute)} 分钟前`
  if (diff < day) return `${Math.floor(diff / hour)} 小时前`
  if (diff < 2 * day) return '昨天'
  if (diff < 7 * day) return `${Math.floor(diff / day)} 天前`

  return new Intl.DateTimeFormat('zh-CN', {
    month: '2-digit',
    day: '2-digit'
  }).format(new Date(timestamp))
}
</script>
