<template>
  <section class="stats-workspace">
    <header class="stats-workspace-header glass-panel">
      <div>
        <p class="stats-workspace-kicker">概览</p>
        <h1>角色出场统计</h1>
        <p>保存正文时自动更新 appearance_stats，也可手动重建，查看每个角色出现在哪些章节。</p>
      </div>
      <div class="stats-workspace-actions">
        <button class="secondary-button" type="button" @click="refresh">刷新</button>
        <button :disabled="appearanceStore.rebuilding" class="primary-button" type="button" @click="rebuild">
          {{ appearanceStore.rebuilding ? '重建中…' : '重建统计' }}
        </button>
      </div>
    </header>

    <div class="stats-summary-grid">
      <article class="stats-summary-card paper-card">
        <span>人物设定</span>
        <strong>{{ summary?.totalCards ?? 0 }}</strong>
      </article>
      <article class="stats-summary-card paper-card">
        <span>已出场人物</span>
        <strong>{{ summary?.appearedCards ?? 0 }}</strong>
      </article>
      <article class="stats-summary-card paper-card">
        <span>章节 / 场景</span>
        <strong>{{ summary?.totalDocuments ?? 0 }}</strong>
      </article>
      <article class="stats-summary-card paper-card">
        <span>角色提及</span>
        <strong>{{ summary?.totalMentions ?? 0 }}</strong>
      </article>
    </div>

    <div class="stats-layout">
      <CharacterAppearanceTable
          :active-card-id="appearanceStore.activeCardId"
          :cards="summary?.cards ?? []"
          :loading="appearanceStore.loading"
          @open-card="$emit('open-target', {type: 'card', targetId: $event})"
          @select-card="selectCard"
      />

      <CardAppearanceDetail
          :appearances="appearanceStore.cardAppearances"
          :card="appearanceStore.activeCardSummary"
          @open-target="$emit('open-target', $event)"
      />
    </div>

    <DocumentAppearanceMatrix
        :summary="summary"
        @open-document="$emit('open-target', {type: 'document', targetId: $event, source: 'appearance'})"
    />

    <ErrorBanner :message="error" title="出场统计加载失败" @dismiss="error = null"/>
  </section>
</template>

<script lang="ts" setup>
import CardAppearanceDetail from '~/components/stats/CardAppearanceDetail.vue'
import CharacterAppearanceTable from '~/components/stats/CharacterAppearanceTable.vue'
import DocumentAppearanceMatrix from '~/components/stats/DocumentAppearanceMatrix.vue'
import ErrorBanner from '~/components/common/ErrorBanner.vue'
import type {NovelDocument} from '~/types/document'
import {toAppErrorMessage} from '~/utils/appError'

const props = defineProps<{
  projectId: string
  documents: NovelDocument[]
}>()

defineEmits<{
  'open-target': [target: import('~/types/navigation').OpenTarget]
}>()

const appearanceStore = useAppearanceStore()
const error = ref<string | null>(null)
const summary = computed(() => appearanceStore.summary)

watch(() => props.projectId, async projectId => {
  if (projectId) await refresh()
}, {immediate: true})

async function refresh() {
  error.value = null
  try {
    await appearanceStore.loadSummary(props.projectId)
  } catch (err) {
    error.value = toAppErrorMessage(err, '加载出场统计失败')
  }
}

async function rebuild() {
  error.value = null
  try {
    await appearanceStore.rebuildStats(props.projectId)
  } catch (err) {
    error.value = toAppErrorMessage(err, '重建出场统计失败')
  }
}

async function selectCard(cardId: string) {
  error.value = null
  try {
    await appearanceStore.selectCard(props.projectId, cardId)
  } catch (err) {
    error.value = toAppErrorMessage(err, '加载人物出场详情失败')
  }
}
</script>
