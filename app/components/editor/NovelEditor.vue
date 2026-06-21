<template>
  <section class="novel-editor-shell paper-card">
    <EditorToolbar :editor="editor"/>

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
    />
  </section>
</template>

<script lang="ts" setup>
import {Editor, EditorContent} from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import EditorToolbar from '~/components/editor/EditorToolbar.vue'
import EditorStatusBar from '~/components/editor/EditorStatusBar.vue'
import type {SaveState} from '~/composables/useAutoSave'
import type {UpdateDocumentContentInput} from '~/types/document'

const props = defineProps<{
  documentId: string
}>()

const emit = defineEmits<{
  saved: [payload: { characterCount: number, updatedAt: number }]
  status: [payload: {
    saveState: SaveState,
    characterCount: number,
    lastSavedAt: number | null,
    errorMessage: string | null
  }]
}>()

const documentStore = useDocumentStore()
const editorStore = useEditorStore()
const {countCharacters} = useCharacterCount()

const editor = shallowRef<Editor | null>(null)
const loading = ref(true)
const characterCount = ref(0)

const emptyDocument = {
  type: 'doc',
  content: [
    {type: 'paragraph'}
  ]
}

const isEmpty = computed(() => characterCount.value === 0 && !loading.value && Boolean(editor.value))

async function persist(payload: UpdateDocumentContentInput) {
  const content = await documentStore.updateDocumentContent(payload)
  emit('saved', {
    characterCount: content.characterCount,
    updatedAt: content.updatedAt
  })
}

const {
  saveState,
  lastSavedAt,
  errorMessage,
  queueSave,
  flushSave
} = useAutoSave<UpdateDocumentContentInput>(persist, 1000)

watch([saveState, lastSavedAt, errorMessage, characterCount], () => {
  editorStore.setSaveState(saveState.value)
  editorStore.setLastSavedAt(lastSavedAt.value)
  editorStore.setCurrentCharacterCount(characterCount.value)
  editorStore.setErrorMessage(errorMessage.value)
  emit('status', {
    saveState: saveState.value,
    characterCount: characterCount.value,
    lastSavedAt: lastSavedAt.value,
    errorMessage: errorMessage.value
  })
}, {immediate: true})

onMounted(async () => {
  loading.value = true
  try {
    const content = await documentStore.getDocumentContent(props.documentId)
    characterCount.value = content.characterCount
    lastSavedAt.value = content.updatedAt

    editor.value = new Editor({
      extensions: [StarterKit],
      content: normalizeContent(content.contentJson),
      editorProps: {
        attributes: {
          class: 'novel-prose'
        }
      },
      onUpdate: ({editor: instance}) => {
        const text = instance.getText()
        const nextCharacterCount = countCharacters(text)
        characterCount.value = nextCharacterCount

        queueSave({
          documentId: props.documentId,
          contentJson: instance.getJSON(),
          contentText: text,
          contentHtml: instance.getHTML(),
          characterCount: nextCharacterCount
        })
      }
    })
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : '无法加载章节内容'
    saveState.value = 'failed'
  } finally {
    loading.value = false
  }
})

onBeforeUnmount(() => {
  void flushSave()
  editor.value?.destroy()
})

function focusEditor() {
  editor.value?.commands.focus('end')
}

function normalizeContent(contentJson: unknown): typeof emptyDocument {
  if (isValidTiptapDoc(contentJson)) return contentJson

  if (typeof contentJson === 'string') {
    try {
      const parsed = JSON.parse(contentJson)
      if (isValidTiptapDoc(parsed)) return parsed
    } catch {
      return emptyDocument
    }
  }

  return emptyDocument
}

function isValidTiptapDoc(value: unknown): value is typeof emptyDocument {
  return Boolean(
      value
      && typeof value === 'object'
      && 'type' in value
      && (value as { type?: unknown }).type === 'doc'
  )
}
</script>
