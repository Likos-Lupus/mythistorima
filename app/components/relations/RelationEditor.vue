<template>
  <section class="relation-editor-panel paper-card">
    <header class="relation-panel-header">
      <div>
        <p class="relation-panel-kicker">Relation Editor</p>
        <h2>{{ relation ? '编辑关系' : '新建关系' }}</h2>
      </div>
      <button v-if="relation" class="tree-danger-button" type="button" @click="confirmDelete">删除</button>
    </header>

    <form v-if="cards.length >= 2" class="relation-editor-form" @submit.prevent="submit">
      <label class="card-form-row">
        来源设定卡
        <select v-model="form.sourceCardId" class="form-field">
          <option v-for="card in cards" :key="card.id" :value="card.id">
            {{ cardTypeLabel(card.type) }} · {{ card.name }}
          </option>
        </select>
      </label>

      <label class="card-form-row">
        目标设定卡
        <select v-model="form.targetCardId" class="form-field">
          <option v-for="card in cards" :key="card.id" :value="card.id">
            {{ cardTypeLabel(card.type) }} · {{ card.name }}
          </option>
        </select>
      </label>

      <label class="card-form-row">
        关系类型
        <select v-model="form.relationType" class="form-field">
          <option v-for="option in editableRelationTypes" :key="option.value" :value="option.value">
            {{ option.label }} — {{ option.hint }}
          </option>
        </select>
      </label>

      <label class="card-form-row">
        方向
        <select v-model="form.direction" class="form-field">
          <option value="directed">单向：来源 → 目标</option>
          <option value="bidirectional">双向：来源 ↔ 目标</option>
          <option value="undirected">无方向：仅表示相关</option>
        </select>
      </label>

      <label class="card-form-row card-form-row-full">
        说明
        <textarea
            v-model="form.description"
            class="form-field card-textarea"
            placeholder="例如：林澈名义上属于旧城区治安队，但实际受北塔组织牵制。"
            rows="5"
        ></textarea>
      </label>

      <p v-if="error" class="editor-error">{{ error }}</p>

      <div class="relation-editor-actions">
        <button :disabled="saving" class="primary-button" type="submit">
          {{ saving ? '保存中…' : relation ? '保存关系' : '创建关系' }}
        </button>
        <button class="secondary-button" type="button" @click="$emit('cancel')">清空选择</button>
      </div>
    </form>

    <div v-else class="empty-panel">
      至少需要两张设定卡才能建立关系。请先在“设定”工作区创建人物、地点、组织、道具或事件。
    </div>
  </section>
</template>

<script lang="ts" setup>
import {cardTypeLabel, type SettingCard} from '~/types/card'
import {
  type CardRelation,
  type CardRelationDirection,
  type CardRelationType,
  type CreateCardRelationInput,
  relationTypeOptions,
  type UpdateCardRelationInput
} from '~/types/relation'

const props = defineProps<{
  projectId: string
  cards: SettingCard[]
  relation: CardRelation | null
  selectedCardId?: string | null
  saving?: boolean
}>()

const emit = defineEmits<{
  create: [payload: CreateCardRelationInput]
  update: [payload: UpdateCardRelationInput]
  delete: [relationId: string]
  cancel: []
}>()

const editableRelationTypes = relationTypeOptions.filter(option => option.value !== 'all')
const error = ref<string | null>(null)
const form = reactive({
  sourceCardId: '',
  targetCardId: '',
  relationType: 'belongs_to' as CardRelationType,
  direction: 'directed' as CardRelationDirection,
  description: ''
})

watch(() => [props.relation, props.cards, props.selectedCardId] as const, () => resetForm(), {
  immediate: true,
  deep: true
})

function resetForm() {
  error.value = null
  if (props.relation) {
    form.sourceCardId = props.relation.sourceCardId
    form.targetCardId = props.relation.targetCardId
    form.relationType = props.relation.relationType
    form.direction = normalizeDirection(props.relation.direction)
    form.description = props.relation.description
    return
  }

  const firstCard = props.selectedCardId && props.cards.some(card => card.id === props.selectedCardId)
      ? props.selectedCardId
      : props.cards[0]?.id ?? ''
  const secondCard = props.cards.find(card => card.id !== firstCard)?.id ?? ''
  form.sourceCardId = firstCard || ''
  form.targetCardId = secondCard
  form.relationType = 'belongs_to'
  form.direction = 'directed'
  form.description = ''
}

function normalizeDirection(direction: string): CardRelationDirection {
  if (direction === 'undirected' || direction === 'bidirectional') return direction
  return 'directed'
}

function submit() {
  error.value = null
  if (!form.sourceCardId || !form.targetCardId) {
    error.value = '请选择关系两端的设定卡。'
    return
  }
  if (form.sourceCardId === form.targetCardId) {
    error.value = '关系两端不能是同一张设定卡。'
    return
  }

  if (props.relation) {
    emit('update', {
      relationId: props.relation.id,
      sourceCardId: form.sourceCardId,
      targetCardId: form.targetCardId,
      relationType: form.relationType,
      direction: form.direction,
      description: form.description.trim(),
      metadataJson: '{}'
    })
  } else {
    emit('create', {
      projectId: props.projectId,
      sourceCardId: form.sourceCardId,
      targetCardId: form.targetCardId,
      relationType: form.relationType,
      direction: form.direction,
      description: form.description.trim(),
      metadataJson: '{}'
    })
  }
}

function confirmDelete() {
  if (!props.relation) return
  if (confirm('删除这条设定关系？')) {
    emit('delete', props.relation.id)
  }
}
</script>
