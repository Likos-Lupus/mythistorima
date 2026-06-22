<template>
  <section class="card-editor-panel paper-card">
    <div v-if="!card" class="card-editor-empty">
      <h2>选择或创建一张设定卡</h2>
      <p>创建人物、地点、组织、道具、事件或概念后，可以在正文中输入 @ 快捷插入引用。</p>
      <button class="primary-button" type="button" @click="$emit('create')">创建设定卡</button>
    </div>

    <form v-else class="card-editor-form" @submit.prevent="save">
      <header class="card-editor-header">
        <div>
          <p class="card-editor-kicker">{{ cardTypeLabel(form.type) }}设定卡</p>
          <h2>{{ form.name || '未命名设定' }}</h2>
        </div>
        <div class="card-editor-actions">
          <button :disabled="saving" class="secondary-button" type="submit">{{ saving ? '保存中…' : '保存' }}</button>
          <button class="tree-danger-button" type="button" @click="confirmDelete">删除</button>
        </div>
      </header>

      <label class="card-form-row">
        类型
        <select v-model="form.type" class="form-field">
          <option value="character">人物</option>
          <option value="location">地点</option>
          <option value="organization">组织</option>
          <option value="item">道具</option>
          <option value="event">事件</option>
          <option value="concept">概念</option>
        </select>
      </label>

      <label class="card-form-row">
        名称
        <input v-model.trim="form.name" class="form-field" placeholder="例如：林澈、旧城区、灵能" required>
      </label>

      <label class="card-form-row">
        别名
        <input v-model="aliasesDraft" class="form-field" placeholder="用逗号或换行分隔，例如：小澈, 林同学">
      </label>

      <label class="card-form-row card-form-row-full">
        简介
        <textarea v-model="form.description" class="form-field card-textarea"
                  placeholder="用几句话说明这个设定的核心信息。" rows="5"></textarea>
      </label>

      <section class="card-fields-section">
        <h3>基础字段</h3>
        <div v-for="field in fieldDefinitions" :key="field.key" class="card-form-row card-form-row-full">
          <label>
            {{ field.label }}
            <textarea
                v-if="field.multiline"
                v-model="fieldsDraft[field.key]"
                :placeholder="field.placeholder"
                class="form-field card-textarea"
                rows="3"
            ></textarea>
            <input
                v-else
                v-model="fieldsDraft[field.key]"
                :placeholder="field.placeholder"
                class="form-field"
            >
          </label>
        </div>
      </section>

      <section class="card-reference-section">
        <h3>引用章节</h3>
        <p v-if="references.length === 0" class="text-muted-paper">暂未被正文引用。在编辑器中输入 @
          并选择此设定后，这里会显示引用章节。</p>
        <ul v-else>
          <li v-for="reference in references" :key="reference.id">
            {{ reference.documentTitle || reference.documentId }} · {{ reference.displayText }}<span
              v-if="reference.paragraphId"> · {{ reference.paragraphId }}</span>
          </li>
        </ul>
      </section>

      <p v-if="error" class="editor-error">{{ error }}</p>
    </form>
  </section>
</template>

<script lang="ts" setup>
import {
  type CardFieldDefinition,
  type CardReference,
  cardTypeLabel,
  normalizeCardType,
  type SettingCard
} from '~/types/card'

const props = defineProps<{
  card: SettingCard | null
  references: CardReference[]
  saving?: boolean
}>()

const emit = defineEmits<{
  save: [payload: {
    cardId: string
    type: string
    name: string
    aliasesJson: string
    description: string
    fieldsJson: string
  }]
  delete: [cardId: string]
  create: []
}>()

const error = ref<string | null>(null)
const aliasesDraft = ref('')
const fieldsDraft = reactive<Record<string, string>>({})
const form = reactive({
  type: 'character',
  name: '',
  description: ''
})

const fieldDefinitions = computed<CardFieldDefinition[]>(() => definitionsForType(form.type))

watch(() => props.card, card => {
  error.value = null
  if (!card) return
  form.type = normalizeCardType(card.type)
  form.name = card.name
  form.description = card.description
  aliasesDraft.value = parseAliases(card.aliasesJson).join('\n')
  resetFields(card.fieldsJson, form.type)
}, {immediate: true})

watch(() => form.type, type => {
  for (const field of definitionsForType(type)) {
    if (typeof fieldsDraft[field.key] !== 'string') fieldsDraft[field.key] = ''
  }
})


function definitionsForType(type: string): CardFieldDefinition[] {
  switch (type) {
    case 'location':
      return [
        {key: 'atmosphere', label: '氛围', placeholder: '这个地点给人的感觉、视觉特征或气味。', multiline: true},
        {key: 'notes', label: '备注', placeholder: '与剧情、角色或伏笔相关的备注。', multiline: true}
      ]
    case 'organization':
      return [
        {key: 'scope', label: '范围', placeholder: '组织影响的地域、行业、阶层或势力范围。', multiline: true},
        {key: 'goal', label: '目标', placeholder: '组织公开或隐藏的核心目标。', multiline: true},
        {key: 'structure', label: '结构', placeholder: '首领、派系、层级、成员关系。', multiline: true},
        {key: 'notes', label: '备注', placeholder: '与人物、事件或地点相关的备注。', multiline: true}
      ]
    case 'item':
      return [
        {key: 'owner', label: '持有者', placeholder: '当前持有者或曾经拥有者。'},
        {key: 'power', label: '作用', placeholder: '道具能力、象征意义或剧情用途。', multiline: true},
        {key: 'limitations', label: '限制', placeholder: '使用条件、代价或弱点。', multiline: true},
        {key: 'notes', label: '备注', placeholder: '出现章节、伏笔或回收安排。', multiline: true}
      ]
    case 'event':
      return [
        {key: 'time', label: '时间', placeholder: '故事内时间标，例如：三年前、月蚀夜。'},
        {key: 'cause', label: '起因', placeholder: '事件为何发生？', multiline: true},
        {key: 'consequence', label: '结果', placeholder: '事件造成了什么改变？', multiline: true},
        {key: 'notes', label: '备注', placeholder: '参与者、地点、后续影响。', multiline: true}
      ]
    case 'concept':
      return [
        {key: 'rules', label: '规则', placeholder: '这个概念如何运作？', multiline: true},
        {key: 'limits', label: '限制', placeholder: '它不能做什么？代价是什么？', multiline: true},
        {key: 'notes', label: '备注', placeholder: '额外说明。', multiline: true}
      ]
    default:
      return [
        {key: 'role', label: '定位', placeholder: '主角、配角、反派、导师…'},
        {key: 'motivation', label: '动机', placeholder: '这个人物最想得到什么？', multiline: true},
        {key: 'notes', label: '备注', placeholder: '人物关系、口癖、秘密等。', multiline: true}
      ]
  }
}


function parseAliases(raw: string) {
  try {
    const value = JSON.parse(raw)
    return Array.isArray(value) ? value.map(String).filter(Boolean) : []
  } catch {
    return []
  }
}

function parseFields(raw: string) {
  try {
    const value = JSON.parse(raw)
    return value && typeof value === 'object' && !Array.isArray(value) ? value as Record<string, unknown> : {}
  } catch {
    return {}
  }
}

function resetFields(raw: string, type: string) {
  for (const key of Object.keys(fieldsDraft)) {
    delete fieldsDraft[key]
  }
  const parsed = parseFields(raw)
  for (const field of definitionsForType(type)) {
    fieldsDraft[field.key] = typeof parsed[field.key] === 'string' ? parsed[field.key] as string : ''
  }
}

function aliasesToJson() {
  const aliases = aliasesDraft.value
      .split(/[\n,，]/)
      .map(item => item.trim())
      .filter(Boolean)
  return JSON.stringify([...new Set(aliases)])
}

function fieldsToJson() {
  const result: Record<string, string> = {}
  for (const field of definitionsForType(form.type)) {
    result[field.key] = fieldsDraft[field.key] ?? ''
  }
  return JSON.stringify(result)
}

function save() {
  error.value = null
  if (!props.card) return
  if (!form.name.trim()) {
    error.value = '设定卡名称不能为空。'
    return
  }
  emit('save', {
    cardId: props.card.id,
    type: form.type,
    name: form.name.trim(),
    aliasesJson: aliasesToJson(),
    description: form.description.trim(),
    fieldsJson: fieldsToJson()
  })
}

function confirmDelete() {
  if (!props.card) return
  if (confirm(`删除设定卡“${props.card.name}”？这不会删除正文文字，但会删除引用记录。`)) {
    emit('delete', props.card.id)
  }
}
</script>
