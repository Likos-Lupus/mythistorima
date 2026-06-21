<template>
  <section class="card-workspace" data-phase1-area="card-workspace">
    <header class="card-workspace-header glass-panel">
      <div>
        <p class="card-workspace-kicker">Phase 1 Week 5</p>
        <h1>设定卡</h1>
        <p>管理人物、地点和概念，并查看它们在正文中的 @ 引用章节。</p>
      </div>
      <button class="primary-button" type="button" @click="createCard">新建设定卡</button>
    </header>

    <CardTypeTabs v-model="cardStore.typeFilter" :counts="cardStore.counts"/>

    <div class="card-workspace-grid">
      <CardList
          v-model:query="cardStore.searchQuery"
          :active-card-id="cardStore.activeCardId"
          :cards="cardStore.filteredCards"
          :loading="cardStore.loading"
          @create="createCard"
          @select="cardStore.selectCard"
      />

      <CardEditor
          :card="cardStore.activeCard"
          :references="cardStore.references"
          :saving="cardStore.saving"
          @create="createCard"
          @delete="deleteCard"
          @save="saveCard"
      />
    </div>

    <p v-if="error" class="editor-error">{{ error }}</p>
  </section>
</template>

<script lang="ts" setup>
import CardEditor from '~/components/cards/CardEditor.vue'
import CardList from '~/components/cards/CardList.vue'
import CardTypeTabs from '~/components/cards/CardTypeTabs.vue'
import type {CardType, UpdateCardInput} from '~/types/card'

const props = defineProps<{
  projectId: string
}>()

const cardStore = useCardStore()
const error = ref<string | null>(null)

watch(() => props.projectId, async projectId => {
  if (projectId) await loadCards()
}, {immediate: true})

async function loadCards() {
  error.value = null
  try {
    await cardStore.loadCards(props.projectId, cardStore.typeFilter)
  } catch (err) {
    error.value = errorMessage(err, '加载设定卡失败')
  }
}

async function createCard() {
  error.value = null
  const type = defaultCreateType(cardStore.typeFilter)
  try {
    await cardStore.createCard({
      projectId: props.projectId,
      type,
      name: defaultCardName(type),
      aliasesJson: '[]',
      description: '',
      fieldsJson: defaultFieldsJson(type)
    })
  } catch (err) {
    error.value = errorMessage(err, '创建设定卡失败')
  }
}

async function saveCard(payload: UpdateCardInput) {
  error.value = null
  try {
    await cardStore.updateCard(payload)
  } catch (err) {
    error.value = errorMessage(err, '保存设定卡失败')
  }
}

async function deleteCard(cardId: string) {
  error.value = null
  try {
    await cardStore.deleteCard(cardId)
  } catch (err) {
    error.value = errorMessage(err, '删除设定卡失败')
  }
}

function defaultCreateType(type: CardType) {
  return type === 'location' || type === 'concept' ? type : 'character'
}

function defaultCardName(type: string) {
  switch (type) {
    case 'location':
      return '未命名地点'
    case 'concept':
      return '未命名概念'
    default:
      return '未命名人物'
  }
}

function defaultFieldsJson(type: string) {
  switch (type) {
    case 'location':
      return JSON.stringify({atmosphere: '', notes: ''})
    case 'concept':
      return JSON.stringify({rules: '', limits: '', notes: ''})
    default:
      return JSON.stringify({role: '', motivation: '', notes: ''})
  }
}

function errorMessage(error: unknown, fallback: string) {
  if (error && typeof error === 'object' && 'message' in error) {
    return String((error as { message?: unknown }).message || fallback)
  }
  return fallback
}
</script>
