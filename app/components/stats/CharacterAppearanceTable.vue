<template>
  <section class="appearance-table-panel paper-card">
    <div class="list-panel-header">
      <div>
        <p class="section-kicker">角色出场</p>
        <h2>人物统计</h2>
      </div>
    </div>

    <div v-if="loading" class="empty-panel">正在加载出场统计…</div>
    <div v-else-if="cards.length === 0" class="empty-panel">还没有人物设定卡，或尚未在正文中 @ 引用角色。</div>

    <div v-else class="appearance-table-wrap">
      <table class="appearance-table">
        <thead>
        <tr>
          <th>人物</th>
          <th>章节数</th>
          <th>提及次数</th>
          <th>首次出现</th>
        </tr>
        </thead>
        <tbody>
        <tr
            v-for="card in cards"
            :key="card.cardId"
            :class="{ 'is-active': card.cardId === activeCardId }"
            @click="$emit('select-card', card.cardId)"
        >
          <td>
            <button class="link-button" type="button" @click.stop="$emit('open-card', card.cardId)">
              {{ card.cardName }}
            </button>
          </td>
          <td>{{ card.documentCount }}</td>
          <td>{{ card.totalMentions }}</td>
          <td>{{ card.firstDocumentTitle || '尚未出现' }}</td>
        </tr>
        </tbody>
      </table>
    </div>
  </section>
</template>

<script lang="ts" setup>
import type {CardAppearanceSummary} from '~/types/appearance'

defineProps<{
  cards: CardAppearanceSummary[]
  activeCardId?: string | null
  loading?: boolean
}>()

defineEmits<{
  'select-card': [cardId: string]
  'open-card': [cardId: string]
}>()
</script>
