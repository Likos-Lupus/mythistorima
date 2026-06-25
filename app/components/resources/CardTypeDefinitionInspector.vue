<template>
  <section class="story-inspector-content">
    <div class="story-panel-header">
      <div>
        <strong>设定类型</strong>
        <span>自定义设定类型</span>
      </div>
      <UButton icon="i-lucide-plus" size="xs" variant="ghost" @click="startCreate"/>
    </div>

    <div class="resource-definition-list">
      <UButton
          v-for="definition in definitions"
          :key="definition.id"
          :aria-pressed="selectedId === definition.id"
          :class="['resource-definition-item', {'is-active': selectedId === definition.id}]"
          color="neutral"
          type="button"
          variant="ghost"
          @click="select(definition.id)"
      >
        <UIcon :name="definition.icon"/>
        <span>{{ definition.name }}</span>
        <UBadge :color="definition.isBuiltin ? 'neutral' : 'primary'" size="sm" variant="soft">
          {{ definition.isBuiltin ? '内置' : '自定义' }}
        </UBadge>
      </UButton>
    </div>

    <UForm :state="form" class="story-form" @submit="submit">
      <UFormField label="名称" name="name" required>
        <UInput v-model="form.name" class="w-full" size="sm"/>
      </UFormField>
      <div class="story-form-grid">
        <UFormField label="图标" name="icon">
          <UInput v-model="form.icon" class="w-full" placeholder="i-lucide-tag" size="sm"/>
        </UFormField>
        <UFormField label="颜色" name="color">
          <UInput v-model="form.color" class="w-full" placeholder="neutral" size="sm"/>
        </UFormField>
      </div>

      <section class="resource-schema-editor">
        <header>
          <strong>字段 Schema</strong>
          <UButton color="neutral" icon="i-lucide-plus" label="字段" size="xs" variant="ghost" @click="addField"/>
        </header>
        <div v-for="(field, index) in form.fields" :key="field.localId" class="resource-schema-row">
          <UInput v-model="field.key" aria-label="字段 Key" placeholder="key" size="sm"/>
          <UInput v-model="field.label" aria-label="字段标签" placeholder="标签" size="sm"/>
          <USelect v-model="field.type" :items="fieldTypeItems" aria-label="字段类型" label-key="label" size="sm"
                   value-key="value"/>
          <UButton color="error" icon="i-lucide-x" size="xs" variant="ghost" @click="removeField(index)"/>
        </div>
      </section>

      <div class="story-inspector-actions">
        <UButton :disabled="selectedDefinition?.isBuiltin" :label="selectedDefinition?.isBuiltin ? '内置类型只读' : selectedId ? '保存类型' : '创建类型'"
                 icon="i-lucide-save"
                 size="sm" type="submit"/>
        <UButton v-if="selectedDefinition && !selectedDefinition.isBuiltin" color="error" icon="i-lucide-trash-2"
                 label="删除" size="sm" variant="ghost" @click="$emit('delete', selectedDefinition.id)"/>
      </div>
    </UForm>
  </section>
</template>

<script lang="ts" setup>
import {
  type CardSchemaField,
  type CardSchemaFieldType,
  type CardTypeDefinition,
  parseCardSchemaFields
} from '~/types/card'

const props = defineProps<{
  definitions: CardTypeDefinition[]
}>()

const emit = defineEmits<{
  create: [input: { name: string; icon?: string; color?: string; fields: CardSchemaField[] }]
  update: [id: string, input: { name: string; icon?: string; color?: string; fields: CardSchemaField[] }]
  delete: [id: string]
}>()

interface DraftField extends CardSchemaField {
  localId: string
}

const selectedId = ref<string | null>(null)
const form = reactive({
  name: '',
  icon: 'i-lucide-tag',
  color: 'neutral',
  fields: [] as DraftField[]
})
const fieldTypeItems = [
  {label: '文本', value: 'text'},
  {label: '多行文本', value: 'textarea'},
  {label: '数字', value: 'number'},
  {label: '标签', value: 'tags'}
]
const selectedDefinition = computed(() => props.definitions.find(definition => definition.id === selectedId.value) ?? null)

watch(() => props.definitions, definitions => {
  if (!selectedId.value && definitions.length) select(definitions[0]!.id)
}, {immediate: true})

function select(id: string) {
  selectedId.value = id
  const definition = props.definitions.find(item => item.id === id)
  if (!definition) return
  form.name = definition.name
  form.icon = definition.icon
  form.color = definition.color
  form.fields = parseCardSchemaFields(definition.schemaJson).map(toDraftField)
}

function startCreate() {
  selectedId.value = null
  form.name = '自定义设定'
  form.icon = 'i-lucide-tag'
  form.color = 'neutral'
  form.fields = [toDraftField({key: 'notes', label: '备注', type: 'textarea'})]
}

function toDraftField(field: CardSchemaField): DraftField {
  return {...field, localId: `${field.key}-${Math.random().toString(36).slice(2)}`}
}

function addField() {
  form.fields.push(toDraftField({key: `field_${form.fields.length + 1}`, label: '新字段', type: 'text'}))
}

function removeField(index: number) {
  form.fields.splice(index, 1)
}

function sanitizeFields(): CardSchemaField[] {
  return form.fields
      .map(field => ({
        key: field.key.trim().replace(/[^a-zA-Z0-9_\u4e00-\u9fa5]/g, '_'),
        label: field.label.trim(),
        type: field.type as CardSchemaFieldType,
        placeholder: field.placeholder ?? ''
      }))
      .filter(field => field.key && field.label)
}

function submit() {
  if (selectedDefinition.value?.isBuiltin) return
  const payload = {
    name: form.name.trim() || '自定义设定',
    icon: form.icon,
    color: form.color,
    fields: sanitizeFields()
  }
  if (selectedId.value) emit('update', selectedId.value, payload)
  else emit('create', payload)
}
</script>
