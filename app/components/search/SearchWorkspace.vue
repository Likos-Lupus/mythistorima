<template>
  <section class="search-workspace glass-panel">
    <header class="workspace-section-header">
      <div>
        <p class="eyebrow">Search</p>
        <h2>全文搜索</h2>
        <p>搜索正文、设定、关系、大纲、时间线、伏笔、校对规则和导出模板。</p>
      </div>
      <button :disabled="searchStore.loading" class="secondary-button" type="button" @click="rebuild">
        重建索引
      </button>
    </header>

    <form class="search-command-bar" @submit.prevent="runSearch">
      <input v-model="queryDraft" class="search-input" placeholder="输入角色名、地点、关系、伏笔或正文片段…" type="search">
      <button :disabled="searchStore.loading || !queryDraft.trim()" class="primary-button" type="submit">
        {{ searchStore.loading ? '搜索中…' : '搜索' }}
      </button>
    </form>

    <div class="scope-filter-row">
      <label v-for="item in scopeOptions" :key="item.value" class="scope-pill">
        <input v-model="selectedScopes" :value="item.value" type="checkbox" @change="normalizeScopes(item.value)">
        {{ item.label }}
      </label>
    </div>

    <p v-if="message" class="subtle-message">{{ message }}</p>
    <ErrorBanner
        :message="searchStore.error"
        title="搜索失败"
        @dismiss="searchStore.clear"
    />

    <EmptyState
        v-if="!searchStore.loading && searchStore.results.length === 0"
        description="可以先写正文、创建设定和关系，或点击“重建索引”后再次搜索。"
        icon="⌕"
        title="还没有搜索结果"
    />

    <div v-else class="search-result-list">
      <article
          v-for="result in visibleResults"
          :key="`${result.targetType}-${result.targetId}`"
          class="search-result-item"
      >
        <button class="search-result-main" type="button" @click="openResult(result)">
          <span class="search-result-type">{{ targetTypeLabel(result.targetType) }}</span>
          <strong>{{ result.title }}</strong>
          <p>{{ cleanSnippet(result.snippet) }}</p>
        </button>
      </article>

      <button
          v-if="visibleResults.length < searchStore.results.length"
          class="secondary-button search-load-more"
          type="button"
          @click="visibleLimit += RESULT_PAGE_SIZE"
      >
        加载更多（{{ searchStore.results.length - visibleResults.length }}）
      </button>
    </div>
  </section>
</template>

<script lang="ts" setup>
import EmptyState from '~/components/common/EmptyState.vue'
import ErrorBanner from '~/components/common/ErrorBanner.vue'
import type {OpenTarget} from '~/types/navigation'
import type {SearchResult, SearchScope} from '~/types/search'
import {searchResultToOpenTarget} from '~/utils/openTarget'

const props = defineProps<{ projectId: string }>()
const emit = defineEmits<{
  'open-target': [target: OpenTarget]
}>()

const RESULT_PAGE_SIZE = 80
const searchStore = useSearchStore()
const queryDraft = ref(searchStore.query)
const selectedScopes = ref<SearchScope[]>(searchStore.scopes.length ? [...searchStore.scopes] : ['all'])
const message = ref<string | null>(null)
const visibleLimit = ref(RESULT_PAGE_SIZE)

const visibleResults = computed(() => searchStore.results.slice(0, visibleLimit.value))

const scopeOptions: Array<{ label: string; value: SearchScope }> = [
  {label: '全部', value: 'all'},
  {label: '正文', value: 'documents'},
  {label: '设定', value: 'cards'},
  {label: '事项', value: 'notes'},
  {label: '大纲', value: 'outline'},
  {label: '时间线', value: 'timeline'},
  {label: '关系', value: 'relations'},
  {label: '伏笔', value: 'foreshadow'},
  {label: '校对', value: 'proofreading'},
  {label: '导出模板', value: 'exportTemplates'}
]

watch(() => searchStore.results, () => {
  visibleLimit.value = RESULT_PAGE_SIZE
})

async function runSearch() {
  message.value = null
  visibleLimit.value = RESULT_PAGE_SIZE
  await searchStore.searchProject({
    projectId: props.projectId,
    query: queryDraft.value,
    scopes: selectedScopes.value,
    limit: 200
  })
}

async function rebuild() {
  message.value = null
  await searchStore.rebuildIndex(props.projectId)
  message.value = '搜索索引已重建。'
  if (queryDraft.value.trim()) await runSearch()
}

function openResult(result: SearchResult) {
  emit('open-target', searchResultToOpenTarget(result))
}

function normalizeScopes(changed: SearchScope) {
  if (changed === 'all' && selectedScopes.value.includes('all')) {
    selectedScopes.value = ['all']
    return
  }
  if (changed !== 'all') {
    selectedScopes.value = selectedScopes.value.filter(scope => scope !== 'all')
    if (selectedScopes.value.length === 0) selectedScopes.value = ['all']
  }
}

function targetTypeLabel(type: string) {
  switch (type) {
    case 'card':
      return '设定'
    case 'note':
      return '事项'
    case 'outline':
      return '大纲'
    case 'timeline':
      return '时间线'
    case 'relation':
      return '关系'
    case 'foreshadow':
      return '伏笔'
    case 'proofreadingRule':
      return '校对规则'
    case 'exportTemplate':
      return '导出模板'
    case 'volume':
      return '卷'
    case 'scene':
      return '场景'
    case 'chapter':
      return '章节'
    default:
      return '正文'
  }
}

function cleanSnippet(value: string) {
  return value.replaceAll('‹', '').replaceAll('›', '').replace(/\s+/g, ' ').trim() || '无预览内容'
}
</script>
