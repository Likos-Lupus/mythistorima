<template>
  <section class="proofreading-panel proofreading-rule-list glass-panel">
    <header class="proofreading-panel-header">
      <div>
        <h2>校对规则</h2>
        <p>{{ rules.length }} 条规则，{{ enabledCount }} 条启用</p>
      </div>
      <button class="secondary-button" type="button" @click="$emit('create')">新建规则</button>
    </header>

    <div v-if="loading" class="proofreading-empty">正在加载规则…</div>
    <div v-else-if="!rules.length" class="proofreading-empty">暂无校对规则。</div>

    <template v-else>
      <button
          v-for="rule in rules"
          :key="rule.id"
          :class="['proofreading-rule-item', { 'is-active': rule.id === activeRuleId, 'is-disabled': rule.enabled === 0 }]"
          type="button"
          @click="$emit('select', rule.id)"
      >
        <span class="proofreading-rule-main">
          <strong>{{ rule.name }}</strong>
          <small>{{ proofreadingRuleTypeLabel(rule.type) }} · {{ proofreadingSeverityLabel(rule.severity) }}</small>
        </span>
        <span class="proofreading-rule-badges">
          <span v-if="isBuiltinProofreadingRule(rule)" class="proofreading-badge">内置</span>
          <span :class="['proofreading-status-dot', rule.enabled === 0 ? 'is-off' : 'is-on']">
            {{ rule.enabled === 0 ? '停用' : '启用' }}
          </span>
        </span>
      </button>
    </template>
  </section>
</template>

<script lang="ts" setup>
import type {ProofreadingRule} from '~/types/proofreading'
import {isBuiltinProofreadingRule, proofreadingRuleTypeLabel, proofreadingSeverityLabel} from '~/types/proofreading'

const props = defineProps<{
  rules: ProofreadingRule[]
  activeRuleId?: string | null
  loading?: boolean
}>()

defineEmits<{
  select: [ruleId: string]
  create: []
}>()

const enabledCount = computed(() => props.rules.filter(rule => rule.enabled !== 0).length)
</script>
