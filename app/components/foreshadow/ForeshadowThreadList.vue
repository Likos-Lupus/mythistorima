<template>
  <section class="foreshadow-thread-list paper-card">
    <div class="list-panel-header">
      <div>
        <p class="section-kicker">线程列表</p>
        <h2>伏笔线程</h2>
      </div>
      <button class="primary-button" type="button" @click="$emit('create')">新建</button>
    </div>

    <div v-if="loading" class="empty-panel">正在加载伏笔线程…</div>
    <div v-else-if="threads.length === 0" class="empty-panel">还没有伏笔线程。可以新建，或从伏笔事项一键生成。</div>

    <div v-else class="foreshadow-thread-stack">
      <button
          v-for="thread in threads"
          :key="thread.id"
          :class="['foreshadow-thread-item', { 'is-active': thread.id === activeThreadId }]"
          type="button"
          @click="$emit('select', thread.id)"
      >
        <span class="thread-status-pill">{{ foreshadowStatusLabel(thread.status) }}</span>
        <strong>{{ thread.title }}</strong>
        <small>{{ foreshadowPriorityLabel(thread.priority) }}优先级 · {{ thread.description || '暂无说明' }}</small>
        <span class="thread-route">
          {{ thread.setupDocumentTitle || '未设置提出章节' }}
          <span>→</span>
          {{ thread.payoffDocumentTitle || '未设置回收章节' }}
        </span>
      </button>
    </div>
  </section>
</template>

<script lang="ts" setup>
import {foreshadowPriorityLabel, foreshadowStatusLabel, type ForeshadowThread} from '~/types/foreshadow'

defineProps<{
  threads: ForeshadowThread[]
  activeThreadId?: string | null
  loading?: boolean
}>()

defineEmits<{
  select: [threadId: string]
  create: []
}>()
</script>
