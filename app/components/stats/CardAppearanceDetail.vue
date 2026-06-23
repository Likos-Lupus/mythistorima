<template>
  <section class="appearance-detail-panel paper-card">
    <div class="list-panel-header">
      <div>
        <p class="section-kicker">人物详情</p>
        <h2>{{ card?.cardName || '选择人物' }}</h2>
      </div>
      <button v-if="card" class="secondary-button" type="button" @click="$emit('open-card', card.cardId)">打开设定卡
      </button>
    </div>

    <div v-if="!card" class="empty-panel">从左侧选择一个人物，查看它出现在哪些章节。</div>
    <template v-else>
      <div class="appearance-detail-summary">
        <div>
          <span>章节数</span>
          <strong>{{ card.documentCount }}</strong>
        </div>
        <div>
          <span>提及次数</span>
          <strong>{{ card.totalMentions }}</strong>
        </div>
        <div>
          <span>首次出现</span>
          <strong>{{ card.firstDocumentTitle || '尚未出现' }}</strong>
        </div>
      </div>

      <div v-if="appearances.length === 0" class="empty-panel">这个人物还没有在正文中被 @ 引用。</div>
      <div v-else class="appearance-detail-list">
        <article v-for="item in appearances" :key="item.id" class="appearance-detail-item">
          <div>
            <strong>{{ item.documentTitle }}</strong>
            <small>{{ documentTypeLabel(item.documentType) }} · 提及 {{ item.mentionCount }} 次 · 位置
              {{ item.firstSeenPosition ?? 0 }}</small>
          </div>
          <button class="secondary-button" type="button" @click="$emit('open-document', item.documentId)">跳转章节
          </button>
        </article>
      </div>
    </template>
  </section>
</template>

<script lang="ts" setup>
import type {CardAppearance, CardAppearanceSummary} from '~/types/appearance'

defineProps<{
  card?: CardAppearanceSummary | null
  appearances: CardAppearance[]
}>()

defineEmits<{
  'open-document': [documentId: string]
  'open-card': [cardId: string]
}>()

function documentTypeLabel(type: string) {
  switch (type) {
    case 'scene':
      return '场景'
    case 'chapter':
      return '章节'
    case 'volume':
      return '卷'
    default:
      return '文档'
  }
}
</script>
