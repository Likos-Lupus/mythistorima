<template>
  <section class="note-editor-panel paper-card">
    <div v-if="!draft" class="note-editor-empty">
      <h2>选择或创建事项</h2>
      <p>备忘、待办和伏笔可以绑定到项目、章节或具体段落。</p>
    </div>

    <form v-else class="note-editor-form" @submit.prevent="save">
      <header class="note-editor-header">
        <div>
          <p class="note-editor-kicker">{{ draft.id ? '编辑事项' : '新建事项' }}</p>
          <h2>{{ draft.title || '未命名事项' }}</h2>
          <p v-if="draft.documentTitle" class="note-bound-meta">绑定：{{ draft.documentTitle }}
            <template v-if="draft.paragraphId"> · 段落 {{ shortParagraphId(draft.paragraphId) }}</template>
          </p>
        </div>
        <div class="note-editor-actions">
          <button class="secondary-button" type="button" @click="$emit('new')">新建</button>
          <button v-if="draft.id" class="secondary-button" type="button" @click="markDone">标记完成</button>
          <button v-if="draft.id" class="ghost-button danger-button" type="button" @click="$emit('delete', draft.id)">
            删除
          </button>
          <button class="primary-button" type="submit">保存</button>
        </div>
      </header>

      <div class="note-editor-grid">
        <label class="note-form-row">
          类型
          <select v-model="draft.type" class="form-field">
            <option value="memo">备忘</option>
            <option value="todo">待办</option>
            <option value="foreshadow">伏笔</option>
            <option value="issue">问题</option>
            <option value="idea">灵感</option>
          </select>
        </label>

        <label class="note-form-row">
          状态
          <select v-model="draft.status" class="form-field">
            <option value="open">未完成</option>
            <option value="doing">进行中</option>
            <option value="done">已完成</option>
            <option value="archived">归档</option>
          </select>
        </label>

        <label class="note-form-row">
          优先级
          <select v-model="draft.priority" class="form-field">
            <option value="low">低</option>
            <option value="normal">普通</option>
            <option value="high">高</option>
          </select>
        </label>

        <label class="note-form-row">
          绑定文档
          <select v-model="draft.documentId" class="form-field">
            <option :value="null">仅项目级</option>
            <option v-for="document in documents" :key="document.id" :value="document.id">
              {{ indentDocument(document) }}{{ document.title }}
            </option>
          </select>
        </label>
      </div>

      <label class="note-form-row">
        标题
        <input v-model="draft.title" class="form-field" placeholder="例如：回收银色怀表伏笔">
      </label>

      <label class="note-form-row">
        内容
        <textarea v-model="draft.body" class="form-field note-textarea"
                  placeholder="记录情节提示、待补内容或伏笔回收计划。"/>
      </label>

      <p v-if="errorMessage" class="editor-error">{{ errorMessage }}</p>
    </form>
  </section>
</template>

<script lang="ts" setup>
import type {CreativeNote} from '~/types/note'
import type {NovelDocument} from '~/types/document'

const props = defineProps<{
  note: CreativeNote | null
  documents: NovelDocument[]
}>()

const emit = defineEmits<{
  save: [payload: CreativeNote]
  new: []
  delete: [noteId: string]
  done: [noteId: string]
}>()

const draft = ref<CreativeNote | null>(null)
const errorMessage = ref<string | null>(null)

watch(() => props.note, note => {
  draft.value = note ? {...note} : null
  errorMessage.value = null
}, {immediate: true})

function save() {
  if (!draft.value) return
  if (!draft.value.title.trim()) {
    errorMessage.value = '事项标题不能为空'
    return
  }
  emit('save', {...draft.value, title: draft.value.title.trim(), body: draft.value.body.trim()})
}

function markDone() {
  if (!draft.value?.id) return
  emit('done', draft.value.id)
}

function shortParagraphId(paragraphId: string) {
  return paragraphId.replace(/^p_/, '').slice(0, 8)
}

function indentDocument(document: NovelDocument) {
  let depth = 0
  let parentId = document.parentId
  while (parentId) {
    const parent = props.documents.find(item => item.id === parentId)
    if (!parent) break
    depth += 1
    parentId = parent.parentId ?? null
  }
  return '　'.repeat(depth)
}
</script>
