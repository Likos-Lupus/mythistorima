<template>
  <section class="story-inspector-content relation-type-inspector">
    <div class="story-panel-header">
      <div>
        <strong>关系类型</strong>
        <span>自定义关系类型</span>
      </div>
      <UButton icon="i-lucide-plus" size="xs" variant="ghost" @click="startCreate"/>
    </div>

    <div class="resource-definition-list compact">
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
          <UInput v-model="form.icon" class="w-full" size="sm"/>
        </UFormField>
        <UFormField label="默认方向" name="direction">
          <USelect v-model="form.direction" :items="directionItems" class="w-full" label-key="label" size="sm"
                   value-key="value"/>
        </UFormField>
      </div>
      <UFormField label="来源类型约束" name="sourceTypeConstraints">
        <UInputTags v-model="form.sourceTypeConstraints" class="w-full" placeholder="输入类型 ID 后回车，如 character"
                    size="sm"/>
      </UFormField>
      <UFormField label="目标类型约束" name="targetTypeConstraints">
        <UInputTags v-model="form.targetTypeConstraints" class="w-full" placeholder="输入类型 ID 后回车，如 location"
                    size="sm"/>
      </UFormField>
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
import type {CardTypeDefinition} from '~/types/card'
import type {RelationTypeDefinition} from '~/types/relation'

const props = defineProps<{
  definitions: RelationTypeDefinition[]
  cardTypeDefinitions: CardTypeDefinition[]
}>()

const emit = defineEmits<{
  create: [input: {
    name: string;
    icon?: string;
    color?: string;
    direction?: string;
    sourceTypeConstraints?: string[];
    targetTypeConstraints?: string[]
  }]
  update: [id: string, input: {
    name: string;
    icon?: string;
    color?: string;
    direction?: string;
    sourceTypeConstraints?: string[];
    targetTypeConstraints?: string[]
  }]
  delete: [id: string]
}>()

const selectedId = ref<string | null>(null)
const form = reactive({
  name: '',
  icon: 'i-lucide-git-commit-horizontal',
  color: 'neutral',
  direction: 'directed',
  sourceTypeConstraints: [] as string[],
  targetTypeConstraints: [] as string[]
})
const selectedDefinition = computed(() => props.definitions.find(definition => definition.id === selectedId.value) ?? null)
const directionItems = [
  {label: '单向', value: 'directed'},
  {label: '双向', value: 'bidirectional'},
  {label: '无方向', value: 'undirected'}
]

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
  form.direction = definition.direction
  form.sourceTypeConstraints = [...definition.sourceTypeConstraints]
  form.targetTypeConstraints = [...definition.targetTypeConstraints]
}

function startCreate() {
  selectedId.value = null
  form.name = '自定义关系'
  form.icon = 'i-lucide-git-commit-horizontal'
  form.color = 'neutral'
  form.direction = 'directed'
  form.sourceTypeConstraints = []
  form.targetTypeConstraints = []
}

function submit() {
  if (selectedDefinition.value?.isBuiltin) return
  const payload = {
    name: form.name.trim() || '自定义关系',
    icon: form.icon,
    color: form.color,
    direction: form.direction,
    sourceTypeConstraints: form.sourceTypeConstraints,
    targetTypeConstraints: form.targetTypeConstraints
  }
  if (selectedId.value) emit('update', selectedId.value, payload)
  else emit('create', payload)
}
</script>
