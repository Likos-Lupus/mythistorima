<template>
  <UModal
      v-model:open="modalOpen"
      :ui="{ content: 'max-w-xl' }"
      description="创建后会自动生成第一章。"
      title="新建小说项目"
  >
    <template #body>
      <UForm :state="form" class="space-y-4" @submit="submit">
        <UFormField label="项目标题" name="title" required>
          <UInput
              v-model="form.title"
              autofocus
              class="w-full"
              placeholder="例如：雾中的王国"
              size="sm"
          />
        </UFormField>

        <div class="grid gap-4 sm:grid-cols-2">
          <UFormField label="作者 / 笔名" name="author">
            <UInput v-model="form.author" class="w-full" placeholder="可选" size="sm"/>
          </UFormField>

          <UFormField label="语言" name="language">
            <USelect
                v-model="form.language"
                :items="languageOptions"
                class="w-full"
                label-key="label"
                size="sm"
                value-key="value"
            />
          </UFormField>
        </div>

        <UFormField label="简介" name="description">
          <UTextarea
              v-model="form.description"
              :rows="4"
              autoresize
              class="w-full"
              placeholder="记录这个项目的核心想法"
              size="sm"
          />
        </UFormField>

        <UAlert
            v-if="localError || error"
            :description="localError || error || ''"
            color="error"
            icon="i-lucide-circle-alert"
            title="无法创建项目"
            variant="subtle"
        />

        <div class="flex justify-end gap-2">
          <UButton color="neutral" label="取消" size="sm" variant="ghost" @click="modalOpen = false"/>
          <UButton
              :loading="submitting"
              icon="i-lucide-plus"
              label="创建项目"
              size="sm"
              type="submit"
          />
        </div>
      </UForm>
    </template>
  </UModal>
</template>

<script lang="ts" setup>
import type {CreateProjectInput} from '~/types/project'

const props = withDefaults(defineProps<{
  open: boolean
  submitting?: boolean
  error?: string | null
}>(), {
  submitting: false,
  error: null
})

const emit = defineEmits<{
  close: []
  submit: [payload: CreateProjectInput]
}>()

const form = reactive({
  title: '',
  author: '',
  description: '',
  language: 'zh-CN'
})
const localError = ref<string | null>(null)

const modalOpen = computed({
  get: () => props.open,
  set: value => {
    if (!value) emit('close')
  }
})

const languageOptions = [
  {label: '简体中文', value: 'zh-CN'},
  {label: 'English', value: 'en'}
]

watch(() => props.open, open => {
  if (open) {
    localError.value = null
    return
  }
  form.title = ''
  form.author = ''
  form.description = ''
  form.language = 'zh-CN'
})

function submit() {
  localError.value = null
  const title = form.title.trim()
  if (!title) {
    localError.value = '请输入项目标题。'
    return
  }
  emit('submit', {
    title,
    author: form.author.trim() || null,
    description: form.description.trim() || null,
    language: form.language
  })
}
</script>
