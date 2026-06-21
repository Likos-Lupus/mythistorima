<template>
  <section :style="editorStyle" class="novel-editor-shell paper-card">
    <EditorToolbar
        :editor="editor"
        :focus-mode="focusMode"
        @toggle-focus="$emit('toggleFocusMode')"
    />

    <EditorFindReplace :editor="editor" @replaced="handleManualContentChange"/>

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
import {ParagraphId} from '~/extensions/ParagraphId'
import {parseTiptapDocument} from '~/utils/tiptapDocument'
import type {SaveState} from '~/composables/useAutoSave'
import type {UpdateDocumentContentInput} from '~/types/document'
import type {EditorSessionSnapshot, EditorSettings} from '~/types/editor'

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
  toggleFocusMode: []
}>()

const documentStore = useDocumentStore()
const editorStore = useEditorStore()
const timerStore = useTimerStore()
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

let sessionTimer: ReturnType<typeof setInterval> | null = null

const isEmpty = computed(() => characterCount.value === 0 && !loading.value && Boolean(editor.value))
const editorStyle = computed(() => ({
  '--editor-font-size': `${props.settings.fontSize}px`,
  '--editor-line-height': String(props.settings.lineHeight),
  '--editor-page-width': `${props.settings.pageWidth}px`
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

  return content
}

const {
  saveState,
  lastSavedAt,
  errorMessage,
  queueSave,
  flushSave
} = useAutoSave<UpdateDocumentContentInput>(persist, 1000)

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
        ParagraphId
      ],
      content: normalized,
      editorProps: {
        attributes: {
          class: 'novel-prose'
        }
      },
      onUpdate: ({editor: instance}) => {
        queueEditorSave(instance)
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

function handleManualContentChange() {
  if (!editor.value) return
  queueEditorSave(editor.value)
}

function focusEditor() {
  editor.value?.commands.focus('end')
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
