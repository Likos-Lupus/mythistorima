<template>
  <section class="proofreading-panel proofreading-inline-panel glass-panel">
    <header class="proofreading-panel-header">
      <div>
        <h2>当前章节问题</h2>
        <p>{{ currentIssues.length }} 个问题</p>
      </div>
      <button :disabled="!activeDocumentId || running" class="secondary-button" type="button"
              @click="$emit('run-current')">
        检查当前章节
      </button>
    </header>

    <div v-if="!activeDocumentId" class="proofreading-empty">先选择一个章节，再运行当前章节检查。</div>
    <div v-else-if="!currentIssues.length" class="proofreading-empty">当前章节暂无问题。</div>

    <template v-else>
      <button
          v-for="issue in currentIssues"
          :key="issue.id"
          :class="['proofreading-inline-issue', `is-${issue.severity}`, { 'is-active': issue.id === activeIssueId }]"
          type="button"
          @click="$emit('select', issue.id)"
      >
        <strong>{{ issue.message }}</strong>
        <small>{{ proofreadingSeverityLabel(issue.severity) }} · {{ issue.paragraphId ?? '段落' }}</small>
        <span v-if="issue.suggestion">建议：{{ issue.suggestion }}</span>
      </button>
    </template>
  </section>
</template>

<script lang="ts" setup>
import type {ProofreadingIssue} from '~/types/proofreading'
import {proofreadingSeverityLabel} from '~/types/proofreading'

const props = defineProps<{
  issues: ProofreadingIssue[]
  activeDocumentId?: string | null
  activeIssueId?: string | null
  running?: boolean
}>()

defineEmits<{
  select: [issueId: string]
  'run-current': []
}>()

const currentIssues = computed(() => props.activeDocumentId
    ? props.issues.filter(issue => issue.documentId === props.activeDocumentId)
    : [])
</script>
