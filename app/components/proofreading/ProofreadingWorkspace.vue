<template>
  <section class="proofreading-workspace" data-phase2-area="proofreading-workspace">
    <header class="proofreading-workspace-header glass-panel">
      <div>
        <p class="proofreading-workspace-kicker">Phase 2 Week 7</p>
        <h1>校对规则引擎</h1>
        <p>运行本地、可配置、非 AI 的基础校对规则，检查重复词、标点、超长句、敏感词和设定名疑似错误。</p>
      </div>
      <div class="proofreading-workspace-actions">
        <button :disabled="proofreadingStore.loading" class="secondary-button" type="button" @click="refreshRules">
          刷新规则
        </button>
        <button :disabled="!activeDocumentId || proofreadingStore.running" class="secondary-button" type="button"
                @click="runCurrentDocument">
          检查当前章节
        </button>
        <button :disabled="proofreadingStore.running" class="primary-button" type="button" @click="runProject">
          检查全项目
        </button>
      </div>
    </header>

    <div class="proofreading-summary-row">
      <article class="proofreading-summary-card glass-panel">
        <strong>{{ proofreadingStore.rules.length }}</strong>
        <span>规则</span>
      </article>
      <article class="proofreading-summary-card glass-panel">
        <strong>{{ proofreadingStore.enabledRules.length }}</strong>
        <span>已启用</span>
      </article>
      <article class="proofreading-summary-card glass-panel">
        <strong>{{ proofreadingStore.issues.length }}</strong>
        <span>问题</span>
      </article>
      <article class="proofreading-summary-card glass-panel">
        <strong>{{ activeDocumentIssueCount }}</strong>
        <span>当前章节</span>
      </article>
    </div>

    <div class="proofreading-layout">
      <ProofreadingRuleList
          :active-rule-id="proofreadingStore.activeRuleId"
          :loading="proofreadingStore.loading"
          :rules="proofreadingStore.rules"
          @create="startCreateRule"
          @select="proofreadingStore.selectRule"
      />

      <main class="proofreading-main-stack">
        <ProofreadingResultList
            :active-issue-id="proofreadingStore.activeIssueId"
            :documents="documents"
            :issues="proofreadingStore.issues"
            :rules="proofreadingStore.rules"
            :running="proofreadingStore.running"
            :scope="proofreadingStore.lastRunScope"
            @select="proofreadingStore.selectIssue"
            @open-target="$emit('open-target', $event)"
        >
          <template #actions>
            <button :disabled="!proofreadingStore.issues.length" class="text-button" type="button"
                    @click="proofreadingStore.clearIssues">
              清空结果
            </button>
          </template>
        </ProofreadingResultList>

        <ProofreadingInlinePanel
            :active-document-id="activeDocumentId"
            :active-issue-id="proofreadingStore.activeIssueId"
            :issues="proofreadingStore.issues"
            :running="proofreadingStore.running"
            @select="proofreadingStore.selectIssue"
            @open-target="$emit('open-target', $event)"
            @run-current="runCurrentDocument"
        />
      </main>

      <ProofreadingRuleEditor
          :project-id="projectId"
          :rule="proofreadingStore.activeRule"
          :saving="proofreadingStore.saving"
          @create="createRule"
          @delete="deleteRule"
          @update="updateRule"
      />
    </div>

    <ErrorBanner :message="error" title="校对操作失败" @dismiss="error = null"/>
  </section>
</template>

<script lang="ts" setup>
import ProofreadingInlinePanel from '~/components/proofreading/ProofreadingInlinePanel.vue'
import ProofreadingResultList from '~/components/proofreading/ProofreadingResultList.vue'
import ProofreadingRuleEditor from '~/components/proofreading/ProofreadingRuleEditor.vue'
import ProofreadingRuleList from '~/components/proofreading/ProofreadingRuleList.vue'
import ErrorBanner from '~/components/common/ErrorBanner.vue'
import type {NovelDocument} from '~/types/document'
import type {CreateProofreadingRuleInput, UpdateProofreadingRuleInput} from '~/types/proofreading'
import {toAppErrorMessage} from '~/utils/appError'

const props = defineProps<{
  projectId: string
  activeDocumentId?: string | null
  documents: NovelDocument[]
}>()

defineEmits<{
  'open-target': [target: import('~/types/navigation').OpenTarget]
}>()

const proofreadingStore = useProofreadingStore()
const error = ref<string | null>(null)

const activeDocumentIssueCount = computed(() => props.activeDocumentId
    ? proofreadingStore.issueCountByDocument.get(props.activeDocumentId) ?? 0
    : 0)

watch(() => props.projectId, async projectId => {
  if (projectId) await refreshRules()
}, {immediate: true})

async function refreshRules() {
  error.value = null
  try {
    await proofreadingStore.loadRules({
      projectId: props.projectId,
      includeBuiltin: true
    })
  } catch (err) {
    error.value = toAppErrorMessage(err, '加载校对规则失败')
  }
}

function startCreateRule() {
  proofreadingStore.selectRule(null)
}

async function createRule(payload: CreateProofreadingRuleInput) {
  error.value = null
  try {
    await proofreadingStore.createRule({...payload, projectId: props.projectId})
  } catch (err) {
    error.value = toAppErrorMessage(err, '创建校对规则失败')
  }
}

async function updateRule(payload: UpdateProofreadingRuleInput) {
  error.value = null
  try {
    await proofreadingStore.updateRule(payload)
  } catch (err) {
    error.value = toAppErrorMessage(err, '保存校对规则失败')
  }
}

async function deleteRule(ruleId: string) {
  error.value = null
  try {
    await proofreadingStore.deleteRule(ruleId)
  } catch (err) {
    error.value = toAppErrorMessage(err, '删除校对规则失败')
  }
}

async function runCurrentDocument() {
  if (!props.activeDocumentId) {
    error.value = '请先在写作区选择一个章节。'
    return
  }

  error.value = null
  try {
    await proofreadingStore.runOnDocument({
      documentId: props.activeDocumentId,
      enabledOnly: true
    })
  } catch (err) {
    error.value = toAppErrorMessage(err, '检查当前章节失败')
  }
}

async function runProject() {
  error.value = null
  try {
    await proofreadingStore.runOnProject({
      projectId: props.projectId,
      enabledOnly: true
    })
  } catch (err) {
    error.value = toAppErrorMessage(err, '检查全项目失败')
  }
}
</script>
