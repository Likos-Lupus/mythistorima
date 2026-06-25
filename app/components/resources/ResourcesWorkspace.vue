<template>
  <section class="story-workspace resources-workspace">
    <UAlert
        v-if="error"
        :description="error"
        class="story-alert"
        color="error"
        icon="i-lucide-circle-alert"
        title="资料操作失败"
        variant="subtle"
        @close="error = null"
    />

    <div v-if="activeTab === 'cards'" class="story-master-detail resources-card-layout">
      <aside class="story-side-panel">
        <div class="story-panel-header">
          <div>
            <strong>设定列表</strong>
            <span>{{ cardStore.filteredCards.length }} / {{ cardStore.cards.length }}</span>
          </div>
          <div class="story-panel-actions">
            <UTooltip text="刷新资料">
              <UButton color="neutral" icon="i-lucide-refresh-cw" size="xs" variant="ghost" @click="refresh"/>
            </UTooltip>
            <UTooltip text="新建设定">
              <UButton color="neutral" icon="i-lucide-plus" size="xs" variant="ghost" @click="createCard"/>
            </UTooltip>
          </div>
        </div>
        <UInput v-model="cardStore.searchQuery" icon="i-lucide-search" placeholder="搜索名称、别名、简介" size="sm"/>
        <USelect v-model="cardStore.typeFilter" :items="definitionStore.cardTypeFilterItems" class="w-full"
                 label-key="label" size="sm" value-key="value" @update:model-value="syncActiveCardForFilter"/>
        <div class="resource-type-list">
          <UButton
              v-for="type in definitionStore.cardTypeDefinitions"
              :key="type.id"
              :color="cardStore.typeFilter === type.id ? 'primary' : 'neutral'"
              :icon="type.icon"
              :label="`${type.name} ${cardCount(type.id)}`"
              :variant="cardStore.typeFilter === type.id ? 'soft' : 'ghost'"
              class="justify-start"
              size="sm"
              @click="cardStore.setTypeFilter(type.id)"
          />
        </div>
        <div class="story-list-scroll">
          <UButton
              v-for="card in cardStore.filteredCards"
              :key="card.id"
              :aria-pressed="card.id === cardStore.activeCardId"
              :class="['story-list-item', {'is-active': card.id === cardStore.activeCardId}]"
              color="neutral"
              type="button"
              variant="ghost"
              @click="cardStore.selectCard(card.id)"
          >
            <span class="story-list-title">
              <UIcon :name="cardDefinition(card.type).icon"/>
              {{ card.name }}
            </span>
            <small>{{ cardDefinition(card.type).name }} · {{ card.description || '暂无简介' }}</small>
          </UButton>
          <UEmpty
              v-if="!cardStore.loading && !cardStore.filteredCards.length"
              description="创建人物、地点、组织、道具、事件、概念或自定义类型。"
              icon="i-lucide-contact-round"
              title="暂无设定卡"
          />
        </div>
      </aside>

      <main class="story-main-panel resource-detail-panel">
        <SettingCardDetail
            :card="cardStore.activeCard"
            :card-type-definitions="definitionStore.cardTypeDefinitions"
            :references="cardStore.references"
            :saving="cardStore.saving"
            @create="createCard"
            @delete="deleteCard"
            @save="saveCard"
        />
      </main>

      <aside class="story-inspector-panel">
        <CardTypeDefinitionInspector
            :definitions="definitionStore.cardTypeDefinitions"
            @create="definitionStore.createCardType"
            @delete="definitionStore.deleteCardType"
            @update="definitionStore.updateCardType"
        />
      </aside>
    </div>

    <div v-else class="story-master-detail resources-relation-layout">
      <aside class="story-side-panel">
        <div class="story-panel-header">
          <div>
            <strong>关系列表</strong>
            <span>{{ relationStore.filteredRelations.length }} / {{ relationStore.relations.length }}</span>
          </div>
          <div class="story-panel-actions">
            <UTooltip text="刷新关系">
              <UButton color="neutral" icon="i-lucide-refresh-cw" size="xs" variant="ghost" @click="refreshRelations"/>
            </UTooltip>
            <UTooltip text="新建关系">
              <UButton color="neutral" icon="i-lucide-plus" size="xs" variant="ghost" @click="startCreateRelation"/>
            </UTooltip>
          </div>
        </div>
        <USelect v-model="relationStore.relationTypeFilter" :items="definitionStore.relationTypeFilterItems"
                 class="w-full" label-key="label" size="sm" value-key="value"/>
        <USelect v-model="relationStore.cardTypeFilter" :items="definitionStore.cardTypeFilterItems" class="w-full"
                 label-key="label" size="sm" value-key="value"/>
        <div class="story-list-scroll">
          <UButton
              v-for="relation in relationStore.filteredRelations"
              :key="relation.id"
              :aria-pressed="relation.id === relationStore.activeRelationId"
              :class="['story-list-item', {'is-active': relation.id === relationStore.activeRelationId}]"
              color="neutral"
              type="button"
              variant="ghost"
              @click="relationStore.selectRelation(relation.id)"
          >
            <span class="story-list-title">{{ cardName(relation.sourceCardId) }} → {{
                cardName(relation.targetCardId)
              }}</span>
            <small>{{ relationDefinition(relation.relationType).name }} · {{
                relation.description || '暂无说明'
              }}</small>
          </UButton>
          <UEmpty
              v-if="!relationStore.loading && !relationStore.filteredRelations.length"
              description="选择两张设定卡后即可在右侧详情面板中创建关系。"
              icon="i-lucide-share-2"
              title="暂无关系"
          />
        </div>
      </aside>

      <main class="story-main-panel relation-canvas-main">
        <div class="story-panel-toolbar">
          <UButton color="neutral" icon="i-lucide-crosshair" label="清空选择" size="sm" variant="ghost"
                   @click="relationStore.clearSelection"/>
          <UButton color="neutral" icon="i-lucide-refresh-cw" label="刷新图谱" size="sm" variant="ghost"
                   @click="refreshRelations"/>
        </div>
        <RelationGraph
            :active-relation-id="relationStore.activeRelationId"
            :card-type-filter="relationStore.cardTypeFilter"
            :cards="relationStore.graph.cards"
            :relation-type-filter="relationStore.relationTypeFilter"
            :relations="relationStore.relations"
            :selected-card-id="relationStore.selectedCardId"
            @open-card="openCard"
            @select-relation="relationStore.selectRelation"
        />
      </main>

      <aside class="story-inspector-panel resources-relation-inspector">
        <RelationInspector
            :cards="relationStore.graph.cards"
            :project-id="projectId"
            :relation="relationStore.activeRelation"
            :relation-type-definitions="definitionStore.relationTypeDefinitions"
            :saving="relationStore.saving"
            :selected-card-id="relationStore.selectedCardId"
            @create="createRelation"
            @delete="deleteRelation"
            @update="updateRelation"
        />
        <RelationTypeDefinitionInspector
            :card-type-definitions="definitionStore.cardTypeDefinitions"
            :definitions="definitionStore.relationTypeDefinitions"
            @create="definitionStore.createRelationType"
            @delete="definitionStore.deleteRelationType"
            @update="definitionStore.updateRelationType"
        />
      </aside>
    </div>
  </section>
</template>

<script lang="ts" setup>
import RelationGraph from '~/components/relations/RelationGraph.vue'
import SettingCardDetail from '~/components/resources/SettingCardDetail.vue'
import CardTypeDefinitionInspector from '~/components/resources/CardTypeDefinitionInspector.vue'
import RelationInspector from '~/components/resources/RelationInspector.vue'
import RelationTypeDefinitionInspector from '~/components/resources/RelationTypeDefinitionInspector.vue'
import {fieldsJsonFromDefinition, type UpdateCardInput} from '~/types/card'
import type {CreateCardRelationInput, UpdateCardRelationInput} from '~/types/relation'
import type {ProjectWorkspaceMode} from '~/constants/projectViews'
import {toAppErrorMessage} from '~/utils/appError'

const props = defineProps<{
  projectId: string
  mode: ProjectWorkspaceMode
}>()

const emit = defineEmits<{
  selectMode: [mode: ProjectWorkspaceMode]
  openTarget: [target: import('~/types/navigation').OpenTarget]
}>()

const cardStore = useCardStore()
const relationStore = useRelationStore()
const definitionStore = useResourceDefinitionsStore()
const error = ref<string | null>(null)
const activeTab = ref<'cards' | 'relations'>(props.mode === 'relations' ? 'relations' : 'cards')

watch(() => props.mode, mode => {
  activeTab.value = mode === 'relations' ? 'relations' : 'cards'
})

watch(activeTab, tab => {
  const next = tab === 'relations' ? 'relations' : 'cards'
  if (props.mode !== next) emit('selectMode', next)
  if (tab === 'relations') void refreshRelations()
})

watch(() => props.projectId, async projectId => {
  definitionStore.load(projectId)
  await refresh()
}, {immediate: true})

async function run(action: () => Promise<unknown>, fallback: string) {
  error.value = null
  try {
    await action()
  } catch (err) {
    error.value = toAppErrorMessage(err, fallback)
  }
}

async function refresh() {
  await run(async () => {
    await cardStore.loadCards(props.projectId, 'all')
    if (activeTab.value === 'relations') await relationStore.loadGraph(props.projectId)
  }, '加载资料工作区失败')
}

async function refreshRelations() {
  await run(async () => {
    await Promise.all([
      cardStore.loadCards(props.projectId, 'all'),
      relationStore.loadGraph(props.projectId)
    ])
  }, '加载关系图失败')
}

function cardDefinition(type: string) {
  return definitionStore.cardDefinitionFor(type)
}

function relationDefinition(type: string) {
  return definitionStore.relationDefinitionFor(type)
}

function cardCount(type: string) {
  return cardStore.cards.filter(card => card.type === type).length
}

function syncActiveCardForFilter() {
  if (cardStore.activeCard && cardStore.filteredCards.some(card => card.id === cardStore.activeCardId)) return
  cardStore.selectCard(cardStore.filteredCards[0]?.id ?? null)
}

async function createCard() {
  const typeId = cardStore.typeFilter === 'all' ? definitionStore.cardTypeDefinitions[0]?.id ?? 'character' : String(cardStore.typeFilter)
  const definition = definitionStore.cardDefinitionFor(typeId)
  await run(() => cardStore.createCard({
    projectId: props.projectId,
    type: definition.id,
    name: `未命名${definition.name}`,
    aliasesJson: '[]',
    description: '',
    fieldsJson: fieldsJsonFromDefinition(definition)
  }), '创建设定卡失败')
}

async function saveCard(payload: UpdateCardInput) {
  await run(() => cardStore.updateCard(payload), '保存设定卡失败')
}

async function deleteCard(cardId: string) {
  await run(() => cardStore.deleteCard(cardId), '删除设定卡失败')
}

function cardName(cardId: string) {
  return relationStore.cardsById.get(cardId)?.name ?? cardId
}

function startCreateRelation() {
  relationStore.selectRelation(null)
}

async function createRelation(payload: CreateCardRelationInput) {
  await run(() => relationStore.createRelation(payload), '创建关系失败')
}

async function updateRelation(payload: UpdateCardRelationInput) {
  await run(() => relationStore.updateRelation(payload), '保存关系失败')
}

async function deleteRelation(relationId: string) {
  await run(() => relationStore.deleteRelation(relationId), '删除关系失败')
}

function openCard(cardId: string) {
  relationStore.selectCard(cardId)
  cardStore.selectCard(cardId)
  emit('openTarget', {type: 'card', targetId: cardId})
}
</script>
