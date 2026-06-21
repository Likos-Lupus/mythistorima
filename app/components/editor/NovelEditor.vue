<template>
  <section :style="editorStyle" class="novel-editor-shell paper-card">
    <EditorToolbar
        :editor="editor"
        :focus-mode="focusMode"
        @toggle-focus="$emit('toggleFocusMode')"
    />

    <EditorFindReplace :editor="editor" @replaced="handleManualContentChange"/>

    <div v-if="editor" class="paragraph-note-toolbar">
      <span>段落事项</span>
      <button class="toolbar-button" type="button" @click="requestParagraphNote('memo')">备忘</button>
      <button class="toolbar-button" type="button" @click="requestParagraphNote('todo')">待办</button>
      <button class="toolbar-button" type="button" @click="requestParagraphNote('foreshadow')">伏笔</button>
    </div>

    <SettingMentionList
        v-if="mentionState.open"
        :active-index="mentionState.activeIndex"
        :cards="mentionCards"
        :left="mentionState.left"
        :query="mentionState.query"
        :top="mentionState.top"
        @hover="mentionState.activeIndex = $event"
        @select="insertSettingReference"
    />

    <SettingReferencePreview
        v-if="hoverPreview.card"
        :card="hoverPreview.card"
        :left="hoverPreview.left"
        :top="hoverPreview.top"
    />

    <div class="editor-scroll-area">
      <div v-if="loading" class="editor-loading">
        正在加载章节…
      </div>

      <div
          v-else-if="editor"
          aria-label="小说正文编辑器"
          class="editor-paper"
          role="textbox"
          @click="focusEditor"
          @mouseleave="closeReferencePreview"
          @mousemove="handleReferenceMouseMove"
      >
        <div v-if="isEmpty" aria-hidden="true" class="editor-empty-hint">
          <p><strong>从这里开始写作…</strong></p>
          <p>点击纸张区域即可输入正文，内容会在停止输入后自动保存。</p>
        </div>
        <EditorContent :editor="editor"/>
      </div>

      <div v-else class="editor-fallback">
        编辑器尚未准备好。
      </div>
    </div>

    <EditorStatusBar
        :character-count="characterCount"
        :error-message="errorMessage"
        :last-saved-at="lastSavedAt"
        :save-state="saveState"
        :session-delta="sessionDelta"
        :session-elapsed-ms="sessionElapsedMs"
        :target-character-count="targetCharacterCount"
    />
  </section>
</template>

<script lang="ts" setup>
import {Editor, EditorContent} from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import EditorToolbar from '~/components/editor/EditorToolbar.vue'
import EditorStatusBar from '~/components/editor/EditorStatusBar.vue'
import EditorFindReplace from '~/components/editor/EditorFindReplace.vue'
import SettingMentionList from '~/components/editor/SettingMentionList.vue'
import SettingReferencePreview from '~/components/editor/SettingReferencePreview.vue'
import {ParagraphId} from '~/extensions/ParagraphId'
import {SettingReference} from '~/extensions/SettingReference'
import type {SettingCard} from '~/types/card'
import {parseTiptapDocument} from '~/utils/tiptapDocument'
import type {SaveState} from '~/composables/useAutoSave'
import type {UpdateDocumentContentInput} from '~/types/document'
import type {EditorSessionSnapshot, EditorSettings} from '~/types/editor'
import type {EditorParagraphNoteRequest} from '~/types/note'

const props = withDefaults(defineProps<{
  documentId: string
  projectId: string
  focusMode?: boolean
  settings: EditorSettings
  targetCharacterCount?: number | null
}>(), {
  focusMode: false,
  targetCharacterCount: null
})

const emit = defineEmits<{
  saved: [payload: { characterCount: number, updatedAt: number }]
  status: [payload: {
    saveState: SaveState,
    characterCount: number,
    lastSavedAt: number | null,
    errorMessage: string | null,
    sessionElapsedMs: number,
    sessionDelta: number
  }]
  session: [payload: EditorSessionSnapshot]
  paragraphNote: [payload: EditorParagraphNoteRequest]
  toggleFocusMode: []
}>()

const documentStore = useDocumentStore()
const editorStore = useEditorStore()
const timerStore = useTimerStore()
const cardStore = useCardStore()
const {countCharacters} = useCharacterCount()
const {call} = useTauriInvoke()

const editor = shallowRef<Editor | null>(null)
const loading = ref(true)
const characterCount = ref(0)
const sessionStartedAt = ref(Date.now())
const sessionId = ref(createSessionId())
const charactersBefore = ref(0)
const sessionElapsedMs = ref(0)
const sessionDelta = computed(() => Math.max(0, characterCount.value - charactersBefore.value))
const mentionState = reactive({
  open: false,
  query: '',
  activeIndex: 0,
  from: 0,
  to: 0,
  top: 0,
  left: 0
})
const hoverPreview = reactive<{ card: SettingCard | null, top: number, left: number }>({
  card: null,
  top: 0,
  left: 0
})

const mentionCards = computed(() => {
  const query = mentionState.query.trim().toLowerCase()
  return cardStore.cards
      .filter(card => matchesMentionQuery(card, query))
      .slice(0, 8)
})

let sessionTimer: ReturnType<typeof setInterval> | null = null

const isEmpty = computed(() => characterCount.value === 0 && !loading.value && Boolean(editor.value))
const editorStyle = computed(() => ({
  '--editor-font-size': `${props.settings.fontSize}px`,
  '--editor-line-height': String(props.settings.lineHeight),
  '--editor-page-width': `${props.settings.pageWidth}px`,
  '--editor-font-family': editorFontFamily(props.settings.fontFamily)
}))

async function persist(payload: UpdateDocumentContentInput) {
  const normalizedPayload = {
    ...payload,
    contentJson: parseTiptapDocument(payload.contentJson)
  }
  const content = await documentStore.updateDocumentContent(normalizedPayload)

  await recordWritingSession(content.characterCount)

  emit('saved', {
    characterCount: content.characterCount,
    updatedAt: content.updatedAt
  })

  if (cardStore.activeCardId) {
    void cardStore.loadReferences(cardStore.activeCardId)
  }

  return content
}

const {
  saveState,
  lastSavedAt,
  errorMessage,
  queueSave,
  flushSave
} = useAutoSave<UpdateDocumentContentInput>(persist, () => props.settings.autosaveIntervalMs)

watch([saveState, lastSavedAt, errorMessage, characterCount, sessionElapsedMs, sessionDelta], () => {
  editorStore.setSaveState(saveState.value)
  editorStore.setLastSavedAt(lastSavedAt.value)
  editorStore.setCurrentCharacterCount(characterCount.value)
  editorStore.setErrorMessage(errorMessage.value)
  timerStore.recordActivity(sessionDelta.value)
  emit('status', {
    saveState: saveState.value,
    characterCount: characterCount.value,
    lastSavedAt: lastSavedAt.value,
    errorMessage: errorMessage.value,
    sessionElapsedMs: sessionElapsedMs.value,
    sessionDelta: sessionDelta.value
  })
}, {immediate: true})

watch(() => props.documentId, async () => {
  await resetEditorForDocument()
})

watch(() => props.projectId, async projectId => {
  if (!projectId) return
  await ensureCardsLoaded(projectId)
}, {immediate: true})

watch(mentionCards, cards => {
  if (mentionState.activeIndex >= cards.length) mentionState.activeIndex = Math.max(0, cards.length - 1)
})

onMounted(async () => {
  await resetEditorForDocument()
})

onBeforeUnmount(() => {
  void flushSave().then(() => recordWritingSession(characterCount.value))
  stopSessionTimer()
  timerStore.finishSession()
  editor.value?.destroy()
})

async function resetEditorForDocument() {
  loading.value = true
  stopSessionTimer()
  editor.value?.destroy()
  editor.value = null

  try {
    const content = await documentStore.getDocumentContent(props.documentId)
    const normalized = parseTiptapDocument(content.contentJson)
    const normalizedText = extractTextFromDocument(normalized)
    const initialCharacterCount = countCharacters(normalizedText || content.contentText)

    characterCount.value = initialCharacterCount
    charactersBefore.value = initialCharacterCount
    lastSavedAt.value = content.updatedAt
    sessionStartedAt.value = Date.now()
    sessionId.value = createSessionId()
    sessionElapsedMs.value = 0
    timerStore.startSession(props.documentId, sessionStartedAt.value, sessionId.value)
    startSessionTimer()

    editor.value = new Editor({
      extensions: [
        StarterKit.configure({
          paragraph: false
        }),
        ParagraphId,
        SettingReference
      ],
      content: normalized,
      editorProps: {
        attributes: {
          class: 'novel-prose'
        },
        handleKeyDown: (_view, event) => handleMentionKeyDown(event)
      },
      onUpdate: ({editor: instance}) => {
        queueEditorSave(instance)
        updateMentionState(instance)
      },
      onSelectionUpdate: ({editor: instance}) => {
        updateMentionState(instance)
      }
    })

    // The ParagraphId plugin can append IDs immediately after creation. Persist once
    // when it normalizes a legacy Phase 0 document without paragraph ids.
    nextTick(() => {
      if (!editor.value) return
      queueEditorSave(editor.value)
    })
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : '无法加载章节内容'
    saveState.value = 'failed'
  } finally {
    loading.value = false
  }
}

function queueEditorSave(instance: Editor) {
  const normalizedJson = parseTiptapDocument(instance.getJSON())
  const text = instance.getText()
  const nextCharacterCount = countCharacters(text)
  characterCount.value = nextCharacterCount

  queueSave({
    documentId: props.documentId,
    contentJson: normalizedJson,
    contentText: text,
    contentHtml: instance.getHTML(),
    characterCount: nextCharacterCount
  })

  emit('session', createSessionSnapshot())
}

async function ensureCardsLoaded(projectId: string) {
  try {
    await cardStore.loadCards(projectId)
  } catch (error) {
    console.warn('[setting-reference] failed to load cards', error)
  }
}

function matchesMentionQuery(card: SettingCard, query: string) {
  if (!query) return true
  const values = [card.name, card.description]
  try {
    const aliases = JSON.parse(card.aliasesJson)
    if (Array.isArray(aliases)) values.push(...aliases.map(String))
  } catch {
    // Ignore malformed aliases; the Rust service validates new writes.
  }
  return values.some(value => value.toLowerCase().includes(query))
}

function updateMentionState(instance: Editor) {
  const match = findMentionMatch(instance)
  if (!match) {
    mentionState.open = false
    mentionState.query = ''
    mentionState.activeIndex = 0
    return
  }

  const coords = instance.view.coordsAtPos(match.from)
  mentionState.open = true
  mentionState.query = match.query
  mentionState.from = match.from
  mentionState.to = match.to
  mentionState.left = Math.min(Math.max(16, coords.left), Math.max(16, window.innerWidth - 340))
  mentionState.top = Math.min(coords.bottom + 8, Math.max(16, window.innerHeight - 320))
}

function findMentionMatch(instance: Editor) {
  const {selection} = instance.state
  if (!selection.empty) return null

  const $from = selection.$from
  const beforeCursor = $from.parent.textBetween(0, $from.parentOffset, undefined, '\ufffc')
  const atIndex = beforeCursor.lastIndexOf('@')
  if (atIndex < 0) return null

  const prefix = atIndex > 0 ? beforeCursor.charAt(atIndex - 1) : ''
  if (prefix && !/[\s([{'"“‘「『《【]/u.test(prefix)) return null

  const query = beforeCursor.slice(atIndex + 1)
  if (/\s/.test(query) || query.length > 40) return null

  return {
    query,
    from: $from.pos - query.length - 1,
    to: $from.pos
  }
}

function handleMentionKeyDown(event: KeyboardEvent) {
  if (!mentionState.open) return false

  if (event.key === 'ArrowDown') {
    event.preventDefault()
    mentionState.activeIndex = mentionCards.value.length > 0
        ? (mentionState.activeIndex + 1) % mentionCards.value.length
        : 0
    return true
  }

  if (event.key === 'ArrowUp') {
    event.preventDefault()
    mentionState.activeIndex = mentionCards.value.length > 0
        ? (mentionState.activeIndex - 1 + mentionCards.value.length) % mentionCards.value.length
        : 0
    return true
  }

  if (event.key === 'Enter' || event.key === 'Tab') {
    const card = mentionCards.value[mentionState.activeIndex]
    if (!card) return false
    event.preventDefault()
    insertSettingReference(card)
    return true
  }

  if (event.key === 'Escape') {
    event.preventDefault()
    mentionState.open = false
    return true
  }

  return false
}

function insertSettingReference(card: SettingCard) {
  if (!editor.value) return

  const from = mentionState.from
  const to = mentionState.to
  mentionState.open = false

  editor.value
      .chain()
      .focus()
      .deleteRange({from, to})
      .insertContent([
        {
          type: 'text',
          text: card.name,
          marks: [
            {
              type: 'settingReference',
              attrs: {
                cardId: card.id,
                cardType: card.type,
                displayName: card.name
              }
            }
          ]
        },
        {
          type: 'text',
          text: ' '
        }
      ])
      .run()

  nextTick(() => {
    if (editor.value) queueEditorSave(editor.value)
  })
}

function handleReferenceMouseMove(event: MouseEvent) {
  const target = event.target instanceof HTMLElement
      ? event.target.closest<HTMLElement>('.setting-reference[data-card-id]')
      : null

  if (!target) {
    closeReferencePreview()
    return
  }

  const cardId = target.dataset.cardId
  const card = cardStore.cards.find(item => item.id === cardId) ?? null
  if (!card) {
    closeReferencePreview()
    return
  }

  hoverPreview.card = card
  hoverPreview.left = Math.min(event.clientX + 16, Math.max(16, window.innerWidth - 340))
  hoverPreview.top = Math.min(event.clientY + 18, Math.max(16, window.innerHeight - 280))
}

function closeReferencePreview() {
  hoverPreview.card = null
}

function handleManualContentChange() {
  if (!editor.value) return
  queueEditorSave(editor.value)
}

function requestParagraphNote(type: EditorParagraphNoteRequest['type']) {
  if (!editor.value) return
  const paragraphId = getSelectedParagraphId(editor.value)
  const selectedText = getSelectedText(editor.value)
  emit('paragraphNote', {
    type,
    paragraphId,
    selectedText
  })
}

function getSelectedParagraphId(instance: Editor) {
  const {selection} = instance.state
  const $from = selection.$from
  for (let depth = $from.depth; depth >= 0; depth -= 1) {
    const node = $from.node(depth)
    if (node.type.name === 'paragraph' || node.type.name === 'heading') {
      const attrs = node.attrs as { pid?: unknown }
      return typeof attrs.pid === 'string' && attrs.pid.length > 0 ? attrs.pid : null
    }
  }
  return null
}

function getSelectedText(instance: Editor) {
  const {from, to, empty} = instance.state.selection
  if (empty) return ''
  return instance.state.doc.textBetween(from, to, '\n').trim()
}

function focusEditor() {
  editor.value?.commands.focus('end')
}

function editorFontFamily(family: EditorSettings['fontFamily']) {
  switch (family) {
    case 'sans':
      return 'Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif'
    case 'system':
      return 'system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif'
    case 'mono':
      return 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace'
    default:
      return 'ui-serif, "Songti SC", "Noto Serif CJK SC", "Source Han Serif SC", Georgia, serif'
  }
}

function startSessionTimer() {
  stopSessionTimer()
  sessionTimer = setInterval(() => {
    sessionElapsedMs.value = Math.max(0, Date.now() - sessionStartedAt.value)
    emit('session', createSessionSnapshot())
  }, 1000)
}

function stopSessionTimer() {
  if (sessionTimer) {
    clearInterval(sessionTimer)
    sessionTimer = null
  }
}

async function recordWritingSession(charactersAfter: number) {
  try {
    const snapshot = createSessionSnapshot(charactersAfter)
    await call('record_writing_session', {
      input: {
        sessionId: snapshot.sessionId,
        projectId: props.projectId,
        documentId: props.documentId,
        startedAt: snapshot.startedAt,
        endedAt: Date.now(),
        charactersBefore: snapshot.charactersBefore,
        charactersAfter: snapshot.charactersAfter
      }
    })
    emit('session', snapshot)
  } catch (error) {
    console.warn('[writing-session] failed to record session', error)
  }
}

function createSessionSnapshot(charactersAfter = characterCount.value): EditorSessionSnapshot {
  return {
    sessionId: sessionId.value,
    startedAt: sessionStartedAt.value,
    elapsedMs: sessionElapsedMs.value,
    charactersBefore: charactersBefore.value,
    charactersAfter,
    sessionDelta: Math.max(0, charactersAfter - charactersBefore.value)
  }
}

function createSessionId() {
  const random = typeof crypto !== 'undefined' && 'randomUUID' in crypto
      ? crypto.randomUUID()
      : `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`
  return `session_${random.replace(/-/g, '')}`
}

function extractTextFromDocument(document: { content?: unknown[] }): string {
  const chunks: string[] = []
  const walk = (node: unknown) => {
    if (!node || typeof node !== 'object') return
    const current = node as { text?: unknown, content?: unknown[], type?: unknown }
    if (typeof current.text === 'string') chunks.push(current.text)
    if (Array.isArray(current.content)) {
      for (const child of current.content) walk(child)
      if (current.type === 'paragraph' || current.type === 'heading') chunks.push('\n')
    }
  }
  walk(document)
  return chunks.join('')
}
</script>
