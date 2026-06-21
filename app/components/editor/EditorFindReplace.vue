<template>
  <section aria-label="当前章节查找替换" class="editor-find-replace">
    <div class="find-row">
      <input
          v-model="query"
          class="find-input"
          placeholder="查找当前章节…"
          type="search"
          @keydown.enter.prevent="goNext"
      >
      <input
          v-model="replacement"
          class="find-input"
          placeholder="替换为…"
          type="text"
          @keydown.enter.prevent="replaceCurrent"
      >
    </div>
    <div class="find-actions">
      <span class="find-count">{{ matchSummary }}</span>
      <button :disabled="!canSearch" class="tree-action-button" type="button" @click="goPrevious">上一个</button>
      <button :disabled="!canSearch" class="tree-action-button" type="button" @click="goNext">下一个</button>
      <button :disabled="!activeMatch" class="tree-action-button" type="button" @click="replaceCurrent">替换</button>
      <button :disabled="!canSearch" class="tree-action-button" type="button" @click="replaceAll">全部替换</button>
    </div>
  </section>
</template>

<script lang="ts" setup>
import type {Editor} from '@tiptap/vue-3'
import {TextSelection} from '@tiptap/pm/state'
import type {EditorFindMatch} from '~/types/editor'

const props = defineProps<{
  editor: Editor | null
}>()

const emit = defineEmits<{
  replaced: []
}>()

const query = ref('')
const replacement = ref('')
const matches = ref<EditorFindMatch[]>([])
const activeIndex = ref(-1)

const canSearch = computed(() => Boolean(props.editor && query.value.trim().length > 0 && matches.value.length > 0))
const activeMatch = computed(() => activeIndex.value >= 0 ? matches.value[activeIndex.value] : null)
const matchSummary = computed(() => {
  if (!query.value.trim()) return '输入关键词以查找'
  if (!matches.value.length) return '无匹配'
  return `${activeIndex.value + 1} / ${matches.value.length}`
})

watch([query, () => props.editor], () => {
  refreshMatches()
}, {immediate: true})

function refreshMatches() {
  const editor = props.editor
  const term = query.value
  matches.value = editor && term ? findMatches(editor, term) : []
  activeIndex.value = matches.value.length > 0 ? 0 : -1
  revealActiveMatch()
}

function goNext() {
  if (!canSearch.value) return
  activeIndex.value = (activeIndex.value + 1) % matches.value.length
  revealActiveMatch()
}

function goPrevious() {
  if (!canSearch.value) return
  activeIndex.value = activeIndex.value <= 0 ? matches.value.length - 1 : activeIndex.value - 1
  revealActiveMatch()
}

function replaceCurrent() {
  const editor = props.editor
  const match = activeMatch.value
  if (!editor || !match) return

  editor.chain().focus().insertContentAt({from: match.from, to: match.to}, replacement.value).run()
  emit('replaced')
  nextTick(refreshMatches)
}

function replaceAll() {
  const editor = props.editor
  if (!editor || !canSearch.value) return

  const allMatches = [...matches.value].sort((a, b) => b.from - a.from)
  for (const match of allMatches) {
    editor.commands.insertContentAt({from: match.from, to: match.to}, replacement.value, {
      updateSelection: false
    })
  }

  editor.commands.focus()
  emit('replaced')
  nextTick(refreshMatches)
}

function revealActiveMatch() {
  const editor = props.editor
  const match = activeMatch.value
  if (!editor || !match) return

  const selection = TextSelection.create(editor.state.doc, match.from, match.to)
  editor.view.dispatch(editor.state.tr.setSelection(selection).scrollIntoView())
  editor.view.focus()
}

function findMatches(editor: Editor, term: string): EditorFindMatch[] {
  const normalized = term.trim()
  if (!normalized) return []

  const result: EditorFindMatch[] = []
  const lowerTerm = normalized.toLocaleLowerCase()

  editor.state.doc.descendants((node, position) => {
    if (!node.isText || typeof node.text !== 'string') return true

    const text = node.text
    const lowerText = text.toLocaleLowerCase()
    let start = lowerText.indexOf(lowerTerm)

    while (start >= 0) {
      result.push({
        from: position + start,
        to: position + start + normalized.length,
        index: result.length
      })
      start = lowerText.indexOf(lowerTerm, start + Math.max(1, lowerTerm.length))
    }

    return true
  })

  return result
}
</script>
