<template>
  <UModal
      v-model:open="modalOpen"
      :ui="{ content: 'max-w-md' }"
      description="修改项目标题不会影响已有文档。"
      title="重命名项目"
  >
    <template #body>
      <UForm :state="form" class="space-y-4" @submit="submit">
        <UFormField label="项目标题" name="title" required>
          <UInput v-model="form.title" autofocus class="w-full" size="sm"/>
        </UFormField>

        <UAlert
            v-if="localError || error"
            :description="localError || error || ''"
            color="error"
            icon="i-lucide-circle-alert"
            title="无法重命名项目"
            variant="subtle"
        />

        <div class="flex justify-end gap-2">
          <UButton color="neutral" label="取消" size="sm" variant="ghost" @click="modalOpen = false"/>
          <UButton :loading="submitting" label="保存" size="sm" type="submit"/>
        </div>
      </UForm>
    </template>
  </UModal>
</template>

<script lang="ts" setup>
import type {Project} from '~/types/project'

const props = withDefaults(defineProps<{
  open: boolean
  project?: Project | null
  submitting?: boolean
  error?: string | null
}>(), {
  project: null,
  submitting: false,
  error: null
})

const emit = defineEmits<{
  close: []
  submit: [title: string]
}>()

const form = reactive({title: ''})
const localError = ref<string | null>(null)

const modalOpen = computed({
  get: () => props.open,
  set: value => {
    if (!value) emit('close')
  }
})

watch(
    () => [props.open, props.project?.id],
    () => {
      form.title = props.project?.title ?? ''
      localError.value = null
    },
    {immediate: true}
)

function submit() {
  const title = form.title.trim()
  if (!title) {
    localError.value = '请输入项目标题。'
    return
  }
  emit('submit', title)
}
</script>
