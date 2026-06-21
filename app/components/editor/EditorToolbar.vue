<template>
  <div class="editor-toolbar">
    <button
        v-for="button in buttons"
        :key="button.label"
        :class="{ 'is-active': button.active }"
        class="toolbar-button"
        type="button"
        @click="button.action"
    >
      {{ button.label }}
    </button>

    <span class="toolbar-divider"/>

    <button
        :class="{ 'is-active': focusMode }"
        class="toolbar-button"
        type="button"
        @click="$emit('toggleFocus')"
    >
      {{ focusMode ? '退出专注' : '专注模式' }}
    </button>
  </div>
</template>

<script lang="ts" setup>
import type {Editor} from '@tiptap/vue-3'

const props = withDefaults(defineProps<{
  editor: Editor | null
  focusMode?: boolean
}>(), {
  focusMode: false
})

defineEmits<{
  toggleFocus: []
}>()

const buttons = computed(() => {
  const editor = props.editor
  if (!editor) return []

  return [
    {
      label: '正文',
      active: editor.isActive('paragraph'),
      action: () => editor.chain().focus().setParagraph().run()
    },
    {
      label: '标题 1',
      active: editor.isActive('heading', {level: 1}),
      action: () => editor.chain().focus().toggleHeading({level: 1}).run()
    },
    {
      label: '标题 2',
      active: editor.isActive('heading', {level: 2}),
      action: () => editor.chain().focus().toggleHeading({level: 2}).run()
    },
    {
      label: '加粗',
      active: editor.isActive('bold'),
      action: () => editor.chain().focus().toggleBold().run()
    },
    {
      label: '斜体',
      active: editor.isActive('italic'),
      action: () => editor.chain().focus().toggleItalic().run()
    },
    {
      label: '引用',
      active: editor.isActive('blockquote'),
      action: () => editor.chain().focus().toggleBlockquote().run()
    },
    {
      label: '列表',
      active: editor.isActive('bulletList'),
      action: () => editor.chain().focus().toggleBulletList().run()
    },
    {
      label: '撤销',
      active: false,
      action: () => editor.chain().focus().undo().run()
    },
    {
      label: '重做',
      active: false,
      action: () => editor.chain().focus().redo().run()
    }
  ]
})
</script>
