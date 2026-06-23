<template>
  <section class="appearance-matrix-panel paper-card">
    <div class="list-panel-header">
      <div>
        <p class="section-kicker">章节矩阵</p>
        <h2>角色 × 章节</h2>
      </div>
    </div>

    <div v-if="!summary || summary.documents.length === 0" class="empty-panel">暂无章节或场景。</div>
    <div v-else-if="summary.cards.length === 0" class="empty-panel">暂无人物设定卡。</div>
    <div v-else class="appearance-matrix-wrap">
      <table class="appearance-matrix">
        <thead>
        <tr>
          <th>章节</th>
          <th v-for="card in visibleCards" :key="card.cardId" :title="card.cardName">
            {{ shortName(card.cardName) }}
          </th>
        </tr>
        </thead>
        <tbody>
        <tr v-for="document in summary.documents" :key="document.documentId">
          <th>
            <button class="link-button" type="button" @click="$emit('open-document', document.documentId)">
              {{ document.documentTitle }}
            </button>
          </th>
          <td
              v-for="card in visibleCards"
              :key="`${document.documentId}:${card.cardId}`"
              :class="{ 'has-appearance': cellCount(document.documentId, card.cardId) > 0 }"
          >
            {{ cellCount(document.documentId, card.cardId) || '' }}
          </td>
        </tr>
        </tbody>
      </table>
      <p v-if="summary.cards.length > visibleCards.length" class="muted-text">
        当前仅显示提及次数最高的 {{ visibleCards.length }} 位人物。
      </p>
    </div>
  </section>
</template>

<script lang="ts" setup>
import type {ProjectAppearanceSummary} from '~/types/appearance'

const props = defineProps<{
  summary?: ProjectAppearanceSummary | null
  maxCards?: number
}>()

defineEmits<{
  'open-document': [documentId: string]
}>()

const visibleCards = computed(() => (props.summary?.cards ?? []).slice(0, props.maxCards ?? 12))
const cellMap = computed(() => {
  const map = new Map<string, number>()
  for (const item of props.summary?.appearances ?? []) {
    map.set(`${item.documentId}:${item.cardId}`, item.mentionCount)
  }
  return map
})

function cellCount(documentId: string, cardId: string) {
  return cellMap.value.get(`${documentId}:${cardId}`) ?? 0
}

function shortName(name: string) {
  return name.length > 5 ? `${name.slice(0, 5)}…` : name
}
</script>
