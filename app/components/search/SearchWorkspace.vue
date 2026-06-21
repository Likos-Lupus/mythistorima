<template>
  <section class="search-workspace glass-panel">
    <header class="workspace-section-header">
      <div>
        <p class="eyebrow">Search</p>
        <h2>全文搜索</h2>
        <p>搜索正文、设定卡和创作事项。保存内容后会自动更新索引。</p>
      </div>
      <button :disabled="searchStore.loading" class="secondary-button" type="button" @click="rebuild">
        重建索引
      </button>
    </header>

    <form class="search-command-bar" @submit.prevent="runSearch">
      <input v-model="queryDraft" class="search-input" placeholder="输入角色名、地点、伏笔或正文片段…" type="search">
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
    <p v-if="searchStore.error" class="editor-error">{{ searchStore.error }}</p>

    <div v-if="searchStore.results.length === 0" class="empty-search-state">
      <strong>还没有搜索结果</strong>
      <span>可以先创建设定卡、写正文或添加事项，再搜索关键词。</span>
    </div>

    <div v-else class="search-result-list">
      <article v-for="result in searchStore.results" :key="`${result.targetType}-${result.targetId}`"
               class="search-result-item">
        <button class="search-result-main" type="button" @click="$emit('open-result', result)">
          <span class="search-result-type">{{ targetTypeLabel(result.targetType) }}</span>
          <strong>{{ result.title }}</strong>
          <p>{{ cleanSnippet(result.snippet) }}</p>
        </button>
      </article>
    </div>
  </section>
</template>

<script lang="ts" setup>
import type {SearchResult, SearchScope} from '~/types/search'

const props = defineProps<{ projectId: string }>()
defineEmits<{ (event: 'open-result', result: SearchResult): void }>()

const searchStore = useSearchStore()
const queryDraft = ref(searchStore.query)
const selectedScopes = ref<SearchScope[]>(['all'])
const message = ref<string | null>(null)

const scopeOptions: Array<{ label: string; value: SearchScope }> = [
  {label: '全部', value: 'all'},
  {label: '正文', value: 'documents'},
  {label: '设定', value: 'cards'},
  {label: '事项', value: 'notes'}
]

async function runSearch() {
  message.value = null
  await searchStore.searchProject({
    projectId: props.projectId,
    query: queryDraft.value,
    scopes: selectedScopes.value,
    limit: 80
  })
}

async function rebuild() {
  message.value = null
  await searchStore.rebuildIndex(props.projectId)
  message.value = '搜索索引已重建。'
  if (queryDraft.value.trim()) await runSearch()
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
