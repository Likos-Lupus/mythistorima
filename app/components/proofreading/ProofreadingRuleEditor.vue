<template>
  <section class="proofreading-panel proofreading-rule-editor glass-panel">
    <header class="proofreading-panel-header">
      <div>
        <h2>{{ rule ? '规则详情' : '新建规则' }}</h2>
        <p v-if="rule && isBuiltin">内置规则只读；可以复制为项目规则后调整。</p>
        <p v-else>配置项目级校对规则，保存后即可用于当前章节或全项目检查。</p>
      </div>
      <button
          v-if="rule && !isBuiltin"
          :disabled="saving"
          class="danger-button"
          type="button"
          @click="$emit('delete', rule.id)"
      >
        删除
      </button>
    </header>

    <form class="proofreading-form" @submit.prevent="submit">
      <label>
        规则名称
        <input v-model.trim="form.name" :disabled="isReadonly" placeholder="例如：禁用口头禅" type="text"/>
      </label>

      <label>
        规则类型
        <select v-model="form.type" :disabled="isReadonly">
          <option v-for="option in proofreadingRuleTypeOptions" :key="option.value" :value="option.value">
            {{ option.label }}
          </option>
        </select>
        <small>{{ selectedTypeDescription }}</small>
      </label>

      <label>
        严重级别
        <select v-model="form.severity" :disabled="isReadonly">
          <option v-for="option in proofreadingSeverityOptions" :key="option.value" :value="option.value">
            {{ option.label }}
          </option>
        </select>
      </label>

      <label class="proofreading-checkbox-row">
        <input v-model="form.enabled" :disabled="isReadonly" type="checkbox"/>
        <span>启用规则</span>
      </label>

      <label>
        Pattern / 词表
        <textarea
            v-model="form.pattern"
            :disabled="isReadonly"
            placeholder="敏感词可用逗号或换行分隔；自定义正则填写正则表达式。"
            rows="4"
        />
      </label>

      <label>
        Config JSON
        <textarea
            v-model="form.configJson"
            :disabled="isReadonly"
            placeholder='{"maxCharacters":120}'
            rows="5"
        />
      </label>

      <div class="proofreading-editor-actions">
        <button
            v-if="rule && isBuiltin"
            :disabled="saving"
            class="primary-button"
            type="button"
            @click="copyBuiltin"
        >
          复制为项目规则
        </button>
        <button
            v-else
            :disabled="saving || !form.name.trim()"
            class="primary-button"
            type="submit"
        >
          {{ rule ? '保存规则' : '创建规则' }}
        </button>
      </div>
    </form>
  </section>
</template>

<script lang="ts" setup>
import type {
  CreateProofreadingRuleInput,
  ProofreadingRule,
  ProofreadingRuleType,
  ProofreadingSeverity,
  UpdateProofreadingRuleInput
} from '~/types/proofreading'
import {isBuiltinProofreadingRule, proofreadingRuleTypeOptions, proofreadingSeverityOptions} from '~/types/proofreading'

const props = defineProps<{
  projectId: string
  rule?: ProofreadingRule | null
  saving?: boolean
}>()

const emit = defineEmits<{
  create: [payload: CreateProofreadingRuleInput]
  update: [payload: UpdateProofreadingRuleInput]
  delete: [ruleId: string]
}>()

const form = reactive({
  name: '',
  type: 'custom_regex' as ProofreadingRuleType,
  severity: 'warning' as ProofreadingSeverity,
  enabled: true,
  pattern: '',
  configJson: '{}'
})

const isBuiltin = computed(() => props.rule ? isBuiltinProofreadingRule(props.rule) : false)
const isReadonly = computed(() => Boolean(props.rule && isBuiltin.value))
const selectedTypeDescription = computed(() => proofreadingRuleTypeOptions.find(option => option.value === form.type)?.description ?? '')

watch(() => props.rule, rule => {
  form.name = rule?.name ?? ''
  form.type = (rule?.type as ProofreadingRuleType | undefined) ?? 'custom_regex'
  form.severity = (rule?.severity as ProofreadingSeverity | undefined) ?? 'warning'
  form.enabled = rule ? rule.enabled !== 0 : true
  form.pattern = rule?.pattern ?? ''
  form.configJson = rule?.configJson ?? defaultConfigForType(form.type)
}, {immediate: true})

watch(() => form.type, type => {
  if (!props.rule && (!form.configJson || form.configJson === '{}')) {
    form.configJson = defaultConfigForType(type)
  }
})

function defaultConfigForType(type: string) {
  if (type === 'long_sentence') return '{"maxCharacters":120}'
  if (type === 'long_paragraph') return '{"maxCharacters":500}'
  if (type === 'sensitive_word') return '{"words":[]}'
  if (type === 'name_consistency') return '{"maxDistance":1,"minNameCharacters":2}'
  if (type === 'duplicate_word') return '{"maxUnitCharacters":4}'
  if (type === 'custom_regex') return '{"message":"命中自定义正则规则","suggestion":""}'
  return '{}'
}

function payloadBase(): CreateProofreadingRuleInput {
  return {
    projectId: props.projectId,
    name: form.name.trim(),
    type: form.type,
    severity: form.severity,
    enabled: form.enabled ? 1 : 0,
    pattern: form.pattern.trim() || null,
    configJson: form.configJson.trim() || '{}'
  }
}

function submit() {
  if (props.rule && !isBuiltin.value) {
    emit('update', {
      ruleId: props.rule.id,
      name: form.name.trim(),
      type: form.type,
      severity: form.severity,
      enabled: form.enabled ? 1 : 0,
      pattern: form.pattern.trim() || null,
      configJson: form.configJson.trim() || '{}'
    })
    return
  }

  emit('create', payloadBase())
}

function copyBuiltin() {
  if (!props.rule) return
  emit('create', {
    projectId: props.projectId,
    name: `${props.rule.name}（项目）`,
    type: props.rule.type,
    severity: props.rule.severity,
    enabled: props.rule.enabled,
    pattern: props.rule.pattern,
    configJson: props.rule.configJson
  })
}
</script>
