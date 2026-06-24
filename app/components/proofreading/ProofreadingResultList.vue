<template>
  <section class="proofreading-panel proofreading-result-list glass-panel">
    <header class="proofreading-panel-header">
      <div>
        <h2>校对结果</h2>
        <p>{{ issues.length }} 个问题{{ scopeLabel }}</p>
      </div>
      <slot name="actions"/>
    </header>

    <div v-if="running" class="proofreading-empty">正在检查文本…</div>
    <div v-else-if="!issues.length" class="proofreading-empty">暂无问题。运行检查后会在这里显示结果。</div>

    <div v-else class="proofreading-result-groups">
      <article v-for="group in groupedIssues" :key="group.documentId" class="proofreading-result-group">
        <header>
          <strong>{{ group.documentTitle }}</strong>
          <span>{{ group.issues.length }} 项</span>
          <button
              class="text-button"
              type="button"
              @click="$emit('open-target', {type: 'document', targetId: group.documentId, source: 'proofreading'})"
          >
            打开章节
          </button>
        </header>

        <button
            v-for="issue in group.issues"
            :key="issue.id"
            :class="['proofreading-issue-item', `is-${issue.severity}`, { 'is-active': issue.id === activeIssueId }]"
            type="button"
            @click="openIssue(issue)"
        >
          <span class="proofreading-issue-meta">
            {{ proofreadingSeverityLabel(issue.severity) }}
            · {{ ruleLabel(issue.ruleId) }}
            · {{ offsetLabel(issue) }}
          </span>
          <strong>{{ issue.message }}</strong>
          <small v-if="issue.suggestion">建议：{{ issue.suggestion }}</small>
        </button>
      </article>
    </div>
  </section>
</template>

<script lang="ts" setup>
import type {NovelDocument} from '~/types/document'
import type {ProofreadingIssue, ProofreadingRule} from '~/types/proofreading'
import {proofreadingRuleTypeLabel, proofreadingSeverityLabel} from '~/types/proofreading'

const props = defineProps<{
  issues: ProofreadingIssue[]
  rules: ProofreadingRule[]
  documents: NovelDocument[]
  activeIssueId?: string | null
  running?: boolean
  scope?: 'document' | 'project' | null
}>()

const emit = defineEmits<{
  select: [issueId: string]
  'open-target': [target: import('~/types/navigation').OpenTarget]
}>()

const documentById = computed(() => new Map(props.documents.map(document => [document.id, document])))
const ruleById = computed(() => new Map(props.rules.map(rule => [rule.id, rule])))

const scopeLabel = computed(() => {
  if (props.scope === 'document') return '（当前章节）'
  if (props.scope === 'project') return '（全项目）'
  return ''
})

const groupedIssues = computed(() => {
  const groups = new Map<string, ProofreadingIssue[]>()
  for (const issue of props.issues) {
    const list = groups.get(issue.documentId) ?? []
    list.push(issue)
    groups.set(issue.documentId, list)
  }

  return Array.from(groups.entries()).map(([documentId, issues]) => ({
    documentId,
    documentTitle: documentById.value.get(documentId)?.title ?? '未知章节',
    issues
  }))
})

function openIssue(issue: ProofreadingIssue) {
  emit('select', issue.id)
  emit('open-target', {type: 'proofreadingIssue', issue})
}

function ruleLabel(ruleId?: string | null) {
  if (!ruleId) return '未知规则'
  const rule = ruleById.value.get(ruleId)
  return rule ? proofreadingRuleTypeLabel(rule.type) : '未知规则'
}

function offsetLabel(issue: ProofreadingIssue) {
  if (issue.startOffset == null || issue.endOffset == null) return issue.paragraphId ?? '全文'
  return `${issue.paragraphId ?? '段落'} · ${issue.startOffset}-${issue.endOffset}`
}
</script>
