<template>
  <section class="foreshadow-status-board paper-card">
    <div class="foreshadow-status-header">
      <div>
        <p class="section-kicker">未回收面板</p>
        <h2>伏笔状态</h2>
      </div>
      <label class="inline-checkbox">
        <input :checked="onlyUnpaid" type="checkbox"
               @change="$emit('update:only-unpaid', ($event.target as HTMLInputElement).checked)">
        只看未回收
      </label>
    </div>

    <div class="foreshadow-filter-row">
      <label>
        状态
        <select :value="statusFilter" class="compact-select-field"
                @change="$emit('update:status-filter', ($event.target as HTMLSelectElement).value)">
          <option v-for="option in foreshadowStatusOptions" :key="option.value" :value="option.value">
            {{ option.label }}（{{ counts[option.value] ?? 0 }}）
          </option>
        </select>
      </label>
      <label>
        优先级
        <select :value="priorityFilter" class="compact-select-field"
                @change="$emit('update:priority-filter', ($event.target as HTMLSelectElement).value)">
          <option v-for="option in foreshadowPriorityOptions" :key="option.value" :value="option.value">
            {{ option.label }}（{{ counts[option.value] ?? 0 }}）
          </option>
        </select>
      </label>
    </div>

    <div class="foreshadow-status-grid">
      <button
          v-for="status in visibleStatuses"
          :key="status.value"
          class="foreshadow-status-card"
          type="button"
          @click="$emit('update:status-filter', status.value)"
      >
        <span>{{ status.label }}</span>
        <strong>{{ counts[status.value] ?? 0 }}</strong>
      </button>
    </div>

    <div class="foreshadow-unpaid-list">
      <article
          v-for="thread in topUnresolved"
          :key="thread.id"
          :class="['foreshadow-mini-thread', { 'is-active': thread.id === activeThreadId }]"
          @click="$emit('select', thread.id)"
      >
        <span>{{ foreshadowPriorityLabel(thread.priority) }}</span>
        <strong>{{ thread.title }}</strong>
        <small>{{ thread.setupDocumentTitle || '未设置提出章节' }} → {{
            thread.payoffDocumentTitle || '未设置回收章节'
          }}</small>
      </article>
      <p v-if="topUnresolved.length === 0" class="muted-text">暂无未回收伏笔。</p>
    </div>
  </section>
</template>

<script lang="ts" setup>
import {
  type ForeshadowPriority,
  foreshadowPriorityLabel,
  foreshadowPriorityOptions,
  type ForeshadowStatus,
  foreshadowStatusOptions,
  type ForeshadowThread
} from '~/types/foreshadow'

const props = defineProps<{
  threads: ForeshadowThread[]
  counts: Record<string, number>
  activeThreadId?: string | null
  statusFilter: ForeshadowStatus | string
  priorityFilter: ForeshadowPriority | string
  onlyUnpaid: boolean
}>()

defineEmits<{
  select: [threadId: string]
  'update:status-filter': [status: string]
  'update:priority-filter': [priority: string]
  'update:only-unpaid': [onlyUnpaid: boolean]
}>()

const visibleStatuses = foreshadowStatusOptions.filter(option => option.value !== 'all')
const topUnresolved = computed(() =>
    [...props.threads]
        .filter(thread => thread.status !== 'paid_off' && thread.status !== 'archived')
        .slice(0, 6)
)
</script>
