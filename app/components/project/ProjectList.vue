<template>
  <div class="space-y-3">
    <article
        v-for="project in projects"
        :key="project.id"
        class="project-list-card group"
        @click="$emit('open', project.id)"
    >
      <div class="flex items-start justify-between gap-3">
        <div class="min-w-0">
          <h3 class="truncate text-lg font-bold text-[#33251b]">{{ project.title }}</h3>
          <p class="mt-1 text-sm text-muted-paper">
            {{ project.author || '未填写作者' }} · {{ formatDate(project.updatedAt) }}
          </p>
        </div>
        <div class="project-list-actions" @click.stop>
          <span class="rounded-full bg-(--accent-soft) px-3 py-1 text-xs font-semibold text-[#6d4325]">
            {{ project.status === 'archived' ? '已归档' : '进行中' }}
          </span>
          <button class="tree-action-button" type="button" @click="$emit('delete', project.id)">删除</button>
        </div>
      </div>
      <p v-if="project.description" class="mt-3 line-clamp-2 text-sm leading-6 text-[#5f5348]">{{
          project.description
        }}</p>
    </article>

    <div v-if="!projects.length"
         class="rounded-3xl border border-dashed border-(--line-soft) bg-white/35 p-8 text-center text-muted-paper">
      还没有项目。创建一个项目，开始小说工作台 MVP。
    </div>
  </div>
</template>

<script lang="ts" setup>
import type {Project} from '~/types/project'

defineProps<{
  projects: Project[]
}>()

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
