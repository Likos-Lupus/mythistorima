<template>
  <Transition name="editor-find-float">
    <UCard
        v-if="open"
        ref="panelRef"
        :ui="{ body: 'p-2' }"
        aria-label="当前章节查找替换"
        class="editor-find-replace"
        role="search"
        @keydown.esc.stop.prevent="close"
    >
      <div class="find-row">
        <UInput
            ref="queryInputRef"
            v-model="query"
            class="find-input"
            icon="i-lucide-search"
            placeholder="查找当前章节…"
            size="sm"
            type="search"
            @keydown.enter.prevent="goNext"
        />
        <UInput
            v-if="replaceMode"
            v-model="replacement"
            class="find-input"
            icon="i-lucide-repeat-2"
            placeholder="替换为…"
            size="sm"
            type="text"
            @keydown.enter.prevent="replaceCurrent"
        />
      </div>
      <div class="find-actions">
        <span class="find-count">{{ matchSummary }}</span>
        <UTooltip text="上一个">
          <UButton :disabled="!canSearch" color="neutral" icon="i-lucide-chevron-up" size="xs" variant="ghost"
                   @click="goPrevious"/>
        </UTooltip>
        <UTooltip text="下一个">
          <UButton :disabled="!canSearch" color="neutral" icon="i-lucide-chevron-down" size="xs" variant="ghost"
                   @click="goNext"/>
        </UTooltip>
        <UButton
            v-if="!replaceMode"
            color="neutral"
            icon="i-lucide-repeat-2"
            label="替换"
            size="xs"
            variant="ghost"
            @click="$emit('update:replaceMode', true)"
        />
        <template v-else>
          <UButton :disabled="!activeMatch" color="neutral" label="替换" size="xs" variant="ghost"
                   @click="replaceCurrent"/>
          <UButton :disabled="!canSearch" color="neutral" label="全部" size="xs" variant="ghost" @click="replaceAll"/>
        </template>
        <UButton aria-label="关闭查找" color="neutral" icon="i-lucide-x" size="xs" variant="ghost" @click="close"/>
      </div>
    </UCard>
  </Transition>
</template>

<script lang="ts" setup>
import type {Editor} from '@tiptap/core'
import {TextSelection} from '@tiptap/pm/state'
import type {EditorFindMatch} from '~/types/editor'

const props = withDefaults(defineProps<{
  editor: Editor | null
  open: boolean
  replaceMode?: boolean
}>(), {
  replaceMode: false
})

const emit = defineEmits<{
  replaced: []
  'update:open': [value: boolean]
  'update:replaceMode': [value: boolean]
}>()

const query = ref('')
const replacement = ref('')
const matches = ref<EditorFindMatch[]>([])
const activeIndex = ref(-1)
const queryInputRef = ref<{ inputRef?: HTMLInputElement } | null>(null)

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

watch(() => props.open, async value => {
  if (!value) return
  await nextTick()
  const input = queryInputRef.value?.inputRef
  input?.focus()
  input?.select()
})

function close() {
  emit('update:open', false)
  emit('update:replaceMode', false)
  props.editor?.commands.focus()
}

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
  if (!editor || !match || !props.open) return

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
