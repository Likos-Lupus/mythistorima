<template>
  <section class="resource-detail-content">
    <UEmpty
        v-if="!card"
        description="从左侧选择设定卡，或创建人物、地点、组织、道具、事件、概念和自定义类型。"
        icon="i-lucide-contact-round"
        title="选择或创建设定卡"
    >
      <template #actions>
        <UButton icon="i-lucide-plus" label="新建设定" size="sm" @click="$emit('create')"/>
      </template>
    </UEmpty>

    <UForm v-else :state="form" class="story-form resource-card-form" @submit="save">
      <div class="story-detail-header">
        <div>
          <p class="story-kicker">{{ activeDefinition.name }}</p>
          <h2>{{ form.name || '未命名设定' }}</h2>
        </div>
        <div class="story-inspector-actions">
          <UButton :loading="saving" icon="i-lucide-save" label="保存" size="sm" type="submit"/>
          <UButton color="error" icon="i-lucide-trash-2" label="删除" size="sm" variant="ghost"
                   @click="$emit('delete', card.id)"/>
        </div>
      </div>

      <div class="story-form-grid">
        <UFormField label="类型" name="type">
          <USelect v-model="form.type" :items="cardTypeItems" class="w-full" label-key="label" size="sm"
                   value-key="value"/>
        </UFormField>
        <UFormField label="名称" name="name" required>
          <UInput v-model="form.name" class="w-full" size="sm"/>
        </UFormField>
      </div>

      <UFormField label="别名" name="aliases">
        <UInputTags v-model="form.aliases" class="w-full" placeholder="输入别名后回车" size="sm"/>
      </UFormField>

      <UFormField label="简介" name="description">
        <UTextarea v-model="form.description" :rows="4" autoresize class="w-full" size="sm"/>
      </UFormField>

      <section class="resource-fields-section">
        <header>
          <strong>类型字段</strong>
          <span>由 {{ activeDefinition.name }} 的 schema_json 生成</span>
        </header>
        <div class="resource-fields-grid">
          <UFormField v-for="field in activeFields" :key="field.key" :label="field.label" :name="field.key">
            <UTextarea
                v-if="field.type === 'textarea'"
                v-model="fieldDraft[field.key]"
                :placeholder="field.placeholder"
                :rows="3"
                autoresize
                class="w-full"
                size="sm"
            />
            <UInputNumber
                v-else-if="field.type === 'number'"
                v-model="numericFieldDraft[field.key]"
                class="w-full"
                size="sm"
            />
            <UInputTags
                v-else-if="field.type === 'tags'"
                v-model="tagFieldDraft[field.key]"
                class="w-full"
                size="sm"
            />
            <UInput
                v-else
                v-model="fieldDraft[field.key]"
                :placeholder="field.placeholder"
                class="w-full"
                size="sm"
            />
          </UFormField>
        </div>
      </section>

      <section class="resource-reference-section">
        <header>
          <strong>正文引用</strong>
          <UBadge color="neutral" size="sm" variant="soft">{{ references.length }}</UBadge>
        </header>
        <UEmpty v-if="!references.length" description="在编辑器中输入 @ 并选择此设定后，这里会显示引用章节。"
                icon="i-lucide-at-sign" title="暂无引用"/>
        <ul v-else>
          <li v-for="reference in references" :key="reference.id">
            <span>{{ reference.documentTitle || reference.documentId }}</span>
            <small>{{ reference.displayText }}</small>
          </li>
        </ul>
      </section>
    </UForm>
  </section>
</template>

<script lang="ts" setup>
import {
  type CardReference,
  type CardSchemaField,
  type CardTypeDefinition,
  parseCardSchemaFields,
  type SettingCard,
  type UpdateCardInput
} from '~/types/card'

const props = defineProps<{
  card: SettingCard | null
  references: CardReference[]
  cardTypeDefinitions: CardTypeDefinition[]
  saving?: boolean
}>()

const emit = defineEmits<{
  save: [payload: UpdateCardInput]
  delete: [cardId: string]
  create: []
}>()

const form = reactive({
  type: 'character',
  name: '',
  aliases: [] as string[],
  description: ''
})
const fieldDraft = reactive<Record<string, string>>({})
const numericFieldDraft = reactive<Record<string, number>>({})
const tagFieldDraft = reactive<Record<string, string[]>>({})

const cardTypeItems = computed(() => props.cardTypeDefinitions.map(type => ({
  label: type.name,
  value: type.id,
  icon: type.icon
})))
const activeDefinition = computed(() => props.cardTypeDefinitions.find(type => type.id === form.type) ?? props.cardTypeDefinitions[0]!)
const activeFields = computed<CardSchemaField[]>(() => parseCardSchemaFields(activeDefinition.value.schemaJson))

watch(() => props.card, card => reset(card), {immediate: true})
watch(() => form.type, () => ensureFieldDrafts())

function reset(card: SettingCard | null) {
  if (!card) return
  form.type = card.type
  form.name = card.name
  form.aliases = parseAliases(card.aliasesJson)
  form.description = card.description
  resetFields(card.fieldsJson)
}

function parseAliases(raw: string) {
  try {
    const parsed = JSON.parse(raw)
    return Array.isArray(parsed) ? parsed.map(String).filter(Boolean) : []
  } catch {
    return []
  }
}

function parseFields(raw: string) {
  try {
    const parsed = JSON.parse(raw)
    return parsed && typeof parsed === 'object' && !Array.isArray(parsed) ? parsed as Record<string, unknown> : {}
  } catch {
    return {}
  }
}

function resetFields(raw: string) {
  for (const key of Object.keys(fieldDraft)) delete fieldDraft[key]
  for (const key of Object.keys(numericFieldDraft)) delete numericFieldDraft[key]
  for (const key of Object.keys(tagFieldDraft)) delete tagFieldDraft[key]
  const parsed = parseFields(raw)
  for (const field of activeFields.value) {
    if (field.type === 'number') numericFieldDraft[field.key] = Number(parsed[field.key] ?? 0)
    else if (field.type === 'tags') tagFieldDraft[field.key] = Array.isArray(parsed[field.key]) ? (parsed[field.key] as unknown[]).map(String) : []
    else fieldDraft[field.key] = String(parsed[field.key] ?? '')
  }
}

function ensureFieldDrafts() {
  for (const field of activeFields.value) {
    if (field.type === 'number' && typeof numericFieldDraft[field.key] !== 'number') numericFieldDraft[field.key] = 0
    else if (field.type === 'tags' && !Array.isArray(tagFieldDraft[field.key])) tagFieldDraft[field.key] = []
    else if (!['number', 'tags'].includes(field.type) && typeof fieldDraft[field.key] !== 'string') fieldDraft[field.key] = ''
  }
}

function fieldsToJson() {
  const result: Record<string, string | number | string[]> = {}
  for (const field of activeFields.value) {
    if (field.type === 'number') result[field.key] = numericFieldDraft[field.key] ?? 0
    else if (field.type === 'tags') result[field.key] = tagFieldDraft[field.key] ?? []
    else result[field.key] = fieldDraft[field.key] ?? ''
  }
  return JSON.stringify(result)
}

function save() {
  if (!props.card) return
  emit('save', {
    cardId: props.card.id,
    type: form.type,
    name: form.name.trim() || props.card.name,
    aliasesJson: JSON.stringify(form.aliases.map(alias => alias.trim()).filter(Boolean)),
    description: form.description.trim(),
    fieldsJson: fieldsToJson()
  })
}
</script>
