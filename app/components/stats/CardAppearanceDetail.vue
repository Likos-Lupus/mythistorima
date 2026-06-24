<template>
  <section class="appearance-detail-panel paper-card">
    <div class="list-panel-header">
      <div>
        <p class="section-kicker">人物详情</p>
        <h2>{{ card?.cardName || '选择人物' }}</h2>
      </div>
      <button
          v-if="card"
          class="secondary-button"
          type="button"
          @click="$emit('open-target', {type: 'card', targetId: card.cardId})"
      >
        打开设定卡
      </button>
    </div>

    <EmptyState
        v-if="!card"
        description="从左侧选择一个人物，查看它出现在哪些章节。"
        icon="◉"
        title="尚未选择人物"
    />
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

      <EmptyState
          v-if="appearances.length === 0"
          description="在正文中通过 @ 插入该人物后，出场信息会自动记录。"
          icon="@"
          title="还没有正文引用"
      />
      <div v-else class="appearance-detail-list">
        <article v-for="item in appearances" :key="item.id" class="appearance-detail-item">
          <div>
            <strong>{{ item.documentTitle }}</strong>
            <small>
              {{ documentTypeLabel(item.documentType) }} · 提及 {{ item.mentionCount }} 次 · 位置
              {{ item.firstSeenPosition ?? 0 }}
            </small>
          </div>
          <button
              class="secondary-button"
              type="button"
              @click="$emit('open-target', {
                type: 'document',
                targetId: item.documentId,
                startOffset: item.firstSeenPosition,
                source: 'appearance',
                label: `${card.cardName}首次出现`
              })"
          >
            跳转章节
          </button>
        </article>
      </div>
    </template>
  </section>
</template>

<script lang="ts" setup>
import EmptyState from '~/components/common/EmptyState.vue'
import type {CardAppearance, CardAppearanceSummary} from '~/types/appearance'
import type {OpenTarget} from '~/types/navigation'

defineProps<{
  card?: CardAppearanceSummary | null
  appearances: CardAppearance[]
}>()

defineEmits<{
  'open-target': [target: OpenTarget]
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
