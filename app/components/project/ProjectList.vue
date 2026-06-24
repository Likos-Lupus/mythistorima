<template>
  <div class="grid gap-2">
    <USkeleton v-for="index in 4" v-if="loading" :key="index" class="h-20 w-full"/>

    <UCard
        v-for="project in projects"
        v-else
        :key="project.id"
        :ui="{ body: 'p-3' }"
        class="project-list-card cursor-pointer"
        @click="$emit('open', project.id)"
    >
      <div class="flex items-start justify-between gap-3">
        <div class="min-w-0">
          <h3 class="truncate text-sm font-semibold text-highlighted">{{ project.title }}</h3>
          <p class="mt-1 text-xs text-muted">
            {{ project.author || '未填写作者' }} · {{ formatDate(project.updatedAt) }}
          </p>
          <p v-if="project.description" class="mt-2 line-clamp-2 text-sm leading-5 text-muted">
            {{ project.description }}
          </p>
        </div>

        <div class="flex shrink-0 items-center gap-2" @click.stop>
          <UBadge
              :color="project.status === 'archived' ? 'neutral' : 'success'"
              size="sm"
              variant="subtle"
          >
            {{ project.status === 'archived' ? '已归档' : '进行中' }}
          </UBadge>
          <UTooltip text="删除项目">
            <UButton
                aria-label="删除项目"
                color="error"
                icon="i-lucide-trash-2"
                size="xs"
                variant="ghost"
                @click="$emit('delete', project.id)"
            />
          </UTooltip>
        </div>
      </div>
    </UCard>

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

withDefaults(defineProps<{
  projects: Project[]
  loading?: boolean
}>(), {
  loading: false
})

defineEmits<{
  open: [projectId: string]
  delete: [projectId: string]
}>()

function formatDate(timestamp: number) {
  return new Intl.DateTimeFormat('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(timestamp))
}
</script>
