<template>
  <section class="relation-workspace" data-phase2-area="relation-workspace">
    <header class="relation-workspace-header glass-panel">
      <div>
        <p class="relation-workspace-kicker">Phase 2 Week 4</p>
        <h1>设定关系图</h1>
        <p>把人物、地点、组织、道具和事件连接成可查看、可编辑的世界观网络。</p>
      </div>
      <div class="relation-workspace-actions">
        <button class="secondary-button" type="button" @click="refresh">刷新</button>
        <button class="primary-button" type="button" @click="startCreate">新建关系</button>
      </div>
    </header>

    <div class="relation-toolbar paper-card">
      <label>
        节点类型
        <select v-model="relationStore.cardTypeFilter" class="compact-select-field">
          <option v-for="option in cardTypeOptions" :key="option.value" :value="option.value">
            {{ option.label }}
          </option>
        </select>
      </label>
      <RelationTypePicker v-model="relationStore.relationTypeFilter" :counts="relationStore.relationCounts"/>
    </div>

    <div class="relation-layout">
      <RelationGraph
          :active-relation-id="relationStore.activeRelationId"
          :card-type-filter="graphCardTypeFilter"
          :cards="relationStore.graph.cards"
          :relation-type-filter="relationStore.relationTypeFilter"
          :relations="relationStore.relations"
          :selected-card-id="relationStore.selectedCardId"
          @open-card="openCard"
          @select-relation="relationStore.selectRelation"
      />

      <aside class="relation-side-stack">
        <RelationList
            :active-relation-id="relationStore.activeRelationId"
            :cards="relationStore.graph.cards"
            :loading="relationStore.loading"
            :relations="relationStore.filteredRelations"
            @create="startCreate"
            @select="relationStore.selectRelation"
        />

        <RelationEditor
            :cards="relationStore.graph.cards"
            :project-id="projectId"
            :relation="relationStore.activeRelation"
            :saving="relationStore.saving"
            :selected-card-id="relationStore.selectedCardId"
            @cancel="relationStore.clearSelection"
            @create="createRelation"
            @delete="deleteRelation"
            @update="updateRelation"
        />
      </aside>
    </div>

    <ErrorBanner :message="error" title="关系图加载失败" @dismiss="error = null"/>
  </section>
</template>

<script lang="ts" setup>
import RelationEditor from '~/components/relations/RelationEditor.vue'
import RelationGraph from '~/components/relations/RelationGraph.vue'
import RelationList from '~/components/relations/RelationList.vue'
import RelationTypePicker from '~/components/relations/RelationTypePicker.vue'
import ErrorBanner from '~/components/common/ErrorBanner.vue'
import {cardTypeOptions} from '~/types/card'
import type {CardGraphCardTypeFilter, CreateCardRelationInput, UpdateCardRelationInput} from '~/types/relation'
import {toAppErrorMessage} from '~/utils/appError'
import {useRelationStore} from '~/stores/relation.store'

const props = defineProps<{
  projectId: string
}>()

const emit = defineEmits<{
  'open-target': [target: import('~/types/navigation').OpenTarget]
}>()

const relationStore = useRelationStore()
const cardStore = useCardStore()
const error = ref<string | null>(null)
const graphCardTypeFilter = computed(() => relationStore.cardTypeFilter as CardGraphCardTypeFilter)

watch(() => props.projectId, async projectId => {
  if (projectId) await refresh()
}, {immediate: true})

async function refresh() {
  error.value = null
  try {
    await Promise.all([
      relationStore.loadGraph(props.projectId),
      cardStore.loadCards(props.projectId, cardStore.typeFilter)
    ])
  } catch (err) {
    error.value = toAppErrorMessage(err, '加载关系图失败')
  }
}

function startCreate() {
  relationStore.selectRelation(null)
}

async function createRelation(payload: CreateCardRelationInput) {
  error.value = null
  try {
    await relationStore.createRelation(payload)
  } catch (err) {
    error.value = toAppErrorMessage(err, '创建关系失败')
  }
}

async function updateRelation(payload: UpdateCardRelationInput) {
  error.value = null
  try {
    await relationStore.updateRelation(payload)
  } catch (err) {
    error.value = toAppErrorMessage(err, '保存关系失败')
  }
}

async function deleteRelation(relationId: string) {
  error.value = null
  try {
    await relationStore.deleteRelation(relationId)
  } catch (err) {
    error.value = toAppErrorMessage(err, '删除关系失败')
  }
}

function openCard(cardId: string) {
  relationStore.selectCard(cardId)
  cardStore.selectCard(cardId)
  emit('open-target', {type: 'card', targetId: cardId})
}
</script>
