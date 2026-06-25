<template>
  <section class="story-inspector-content">
    <div class="story-panel-header">
      <div>
        <strong>关系详情</strong>
        <span>{{ relation ? '编辑关系' : '新建关系' }}</span>
      </div>
    </div>

    <UEmpty
        v-if="cards.length < 2"
        description="至少需要两张设定卡才能建立关系。"
        icon="i-lucide-share-2"
        title="设定卡不足"
    />

    <UForm v-else :state="form" class="story-form" @submit="submit">
      <UFormField label="来源设定" name="sourceCardId">
        <USelect v-model="form.sourceCardId" :items="sourceCardItems" class="w-full" label-key="label" size="sm"
                 value-key="value"/>
      </UFormField>
      <UFormField label="目标设定" name="targetCardId">
        <USelect v-model="form.targetCardId" :items="targetCardItems" class="w-full" label-key="label" size="sm"
                 value-key="value"/>
      </UFormField>
      <UFormField label="关系类型" name="relationType">
        <USelect v-model="form.relationType" :items="relationTypeItems" class="w-full" label-key="label" size="sm"
                 value-key="value"/>
      </UFormField>
      <UFormField label="方向" name="direction">
        <USelect v-model="form.direction" :items="directionItems" class="w-full" label-key="label" size="sm"
                 value-key="value"/>
      </UFormField>
      <UFormField label="说明" name="description">
        <UTextarea v-model="form.description" :rows="5" autoresize class="w-full" size="sm"/>
      </UFormField>
      <UAlert v-if="localError" :description="localError" color="error" icon="i-lucide-circle-alert"
              title="无法保存关系" variant="subtle"/>
      <div class="story-inspector-actions">
        <UButton :label="relation ? '保存关系' : '创建关系'" :loading="saving" icon="i-lucide-save" size="sm"
                 type="submit"/>
        <UButton v-if="relation" color="error" icon="i-lucide-trash-2" label="删除" size="sm" variant="ghost"
                 @click="$emit('delete', relation.id)"/>
      </div>
    </UForm>
  </section>
</template>

<script lang="ts" setup>
import {cardTypeLabel, type SettingCard} from '~/types/card'
import type {
  CardRelation,
  CardRelationDirection,
  CreateCardRelationInput,
  RelationTypeDefinition,
  UpdateCardRelationInput
} from '~/types/relation'

const props = defineProps<{
  projectId: string
  cards: SettingCard[]
  relation: CardRelation | null
  relationTypeDefinitions: RelationTypeDefinition[]
  selectedCardId?: string | null
  saving?: boolean
}>()

const emit = defineEmits<{
  create: [payload: CreateCardRelationInput]
  update: [payload: UpdateCardRelationInput]
  delete: [relationId: string]
}>()

const localError = ref<string | null>(null)
const form = reactive({
  sourceCardId: '',
  targetCardId: '',
  relationType: 'related_to',
  direction: 'directed' as CardRelationDirection,
  description: ''
})
const selectedRelationDefinition = computed(() => props.relationTypeDefinitions.find(type => type.id === form.relationType) ?? null)
const sourceCards = computed(() => filterCardsByConstraints(selectedRelationDefinition.value?.sourceTypeConstraints ?? []))
const targetCards = computed(() => filterCardsByConstraints(selectedRelationDefinition.value?.targetTypeConstraints ?? []))
const sourceCardItems = computed(() => sourceCards.value.map(card => ({
  label: `${card.name} · ${cardTypeLabel(card.type)}`,
  value: card.id
})))
const targetCardItems = computed(() => targetCards.value.map(card => ({
  label: `${card.name} · ${cardTypeLabel(card.type)}`,
  value: card.id
})))
const relationTypeItems = computed(() => props.relationTypeDefinitions.map(type => ({
  label: type.name,
  value: type.id,
  icon: type.icon
})))
const directionItems = [
  {label: '单向：来源 → 目标', value: 'directed'},
  {label: '双向：来源 ↔ 目标', value: 'bidirectional'},
  {label: '无方向', value: 'undirected'}
]

watch(() => [props.relation, props.cards, props.selectedCardId] as const, reset, {immediate: true, deep: true})
watch(() => form.relationType, value => {
  const definition = props.relationTypeDefinitions.find(type => type.id === value)
  if (definition && !props.relation) form.direction = normalizeDirection(definition.direction)
  if (!sourceCards.value.some(card => card.id === form.sourceCardId)) form.sourceCardId = sourceCards.value[0]?.id ?? ''
  if (!targetCards.value.some(card => card.id === form.targetCardId)) form.targetCardId = targetCards.value.find(card => card.id !== form.sourceCardId)?.id ?? targetCards.value[0]?.id ?? ''
})


function filterCardsByConstraints(constraints: string[]) {
  if (!constraints.length) return props.cards
  return props.cards.filter(card => constraints.includes(card.type))
}

function reset() {
  localError.value = null
  if (props.relation) {
    form.sourceCardId = props.relation.sourceCardId
    form.targetCardId = props.relation.targetCardId
    form.relationType = props.relation.relationType
    form.direction = normalizeDirection(props.relation.direction)
    form.description = props.relation.description
    return
  }
  const first = props.selectedCardId && props.cards.some(card => card.id === props.selectedCardId)
      ? props.selectedCardId
      : props.cards[0]?.id ?? ''
  form.sourceCardId = first || ''
  form.targetCardId = props.cards.find(card => card.id !== first)?.id ?? ''
  form.relationType = props.relationTypeDefinitions[0]?.id ?? 'related_to'
  form.direction = normalizeDirection(props.relationTypeDefinitions[0]?.direction ?? 'directed')
  form.description = ''
}

function normalizeDirection(direction: string): CardRelationDirection {
  if (direction === 'undirected' || direction === 'bidirectional') return direction
  return 'directed'
}

function submit() {
  localError.value = null
  if (!form.sourceCardId || !form.targetCardId) {
    localError.value = '请选择关系两端的设定卡。'
    return
  }
  if (form.sourceCardId === form.targetCardId) {
    localError.value = '关系两端不能是同一张设定卡。'
    return
  }
  const payload = {
    sourceCardId: form.sourceCardId,
    targetCardId: form.targetCardId,
    relationType: form.relationType,
    direction: form.direction,
    description: form.description.trim(),
    metadataJson: '{}'
  }
  if (props.relation) emit('update', {relationId: props.relation.id, ...payload})
  else emit('create', {projectId: props.projectId, ...payload})
}
</script>
