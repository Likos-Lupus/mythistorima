<template>
  <div v-if="open" class="fixed inset-0 z-50 flex items-center justify-center bg-[#1f160f]/35 p-4 backdrop-blur-sm">
    <section class="paper-card w-full max-w-xl rounded-[2rem] p-6">
      <div class="mb-5 flex items-start justify-between gap-4">
        <div>
          <h2 class="text-2xl font-bold">新建小说项目</h2>
          <p class="mt-1 text-sm text-muted-paper">Phase 0 会自动为项目创建“第一章”。</p>
        </div>
        <button class="rounded-full px-3 py-1 text-lg hover:bg-(--accent-soft)" type="button"
                @click="$emit('close')">×
        </button>
      </div>

      <form class="space-y-4" @submit.prevent="submit">
        <label class="block">
          <span class="mb-2 block text-sm font-semibold">项目标题</span>
          <input v-model.trim="title" autofocus class="form-field" placeholder="例如：雾中的王国"/>
        </label>

        <label class="block">
          <span class="mb-2 block text-sm font-semibold">作者 / 笔名</span>
          <input v-model.trim="author" class="form-field" placeholder="可选"/>
        </label>

        <label class="block">
          <span class="mb-2 block text-sm font-semibold">简介</span>
          <textarea v-model.trim="description" class="form-field min-h-28 resize-none"
                    placeholder="可选，记录这个项目的核心想法"/>
        </label>

        <p v-if="error" class="rounded-2xl bg-red-50 px-4 py-3 text-sm text-red-700">{{ error }}</p>

        <div class="flex justify-end gap-3 pt-2">
          <button class="rounded-full px-5 py-2 text-muted-paper hover:bg-(--accent-soft)" type="button"
                  @click="$emit('close')">
            取消
          </button>
          <button
              :disabled="submitting"
              class="rounded-full bg-[#6d4325] px-5 py-2 font-semibold text-white shadow-lg shadow-[#6d4325]/20 disabled:opacity-60"
              type="submit">
            {{ submitting ? '创建中…' : '创建项目' }}
          </button>
        </div>
      </form>
    </section>
  </div>
</template>

<script lang="ts" setup>
const props = defineProps<{
  open: boolean
  submitting?: boolean
  error?: string | null
}>()

const emit = defineEmits<{
  close: []
  submit: [payload: { title: string, author?: string | null, description?: string | null }]
}>()

const title = ref('')
const author = ref('')
const description = ref('')

watch(() => props.open, (open) => {
  if (!open) {
    title.value = ''
    author.value = ''
    description.value = ''
  }
})

function submit() {
  if (!title.value) return
  emit('submit', {
    title: title.value,
    author: author.value || null,
    description: description.value || null
  })
}
</script>
