<template>
  <div class="editor-toolbar">
    <UTooltip
        v-for="button in buttons"
        :key="button.label"
        :text="button.label"
    >
      <UButton
          :aria-label="button.label"
          :color="button.active ? 'primary' : 'neutral'"
          :icon="button.icon"
          :variant="button.active ? 'soft' : 'ghost'"
          size="sm"
          @click="button.action"
      />
    </UTooltip>

    <span class="toolbar-divider"/>

    <UTooltip text="查找 Ctrl+F">
      <UButton
          aria-label="查找"
          color="neutral"
          icon="i-lucide-search"
          size="sm"
          variant="ghost"
          @click="$emit('find')"
      />
    </UTooltip>

    <UTooltip text="替换 Ctrl+H">
      <UButton
          aria-label="替换"
          color="neutral"
          icon="i-lucide-repeat-2"
          size="sm"
          variant="ghost"
          @click="$emit('replace')"
      />
    </UTooltip>

    <span class="toolbar-divider"/>

    <UTooltip :text="focusMode ? '退出专注' : '专注模式'">
      <UButton
          :color="focusMode ? 'primary' : 'neutral'"
          :icon="focusMode ? 'i-lucide-minimize-2' : 'i-lucide-focus'"
          :variant="focusMode ? 'soft' : 'ghost'"
          aria-label="切换专注模式"
          size="sm"
          @click="$emit('toggleFocus')"
      />
    </UTooltip>
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
  find: []
  replace: []
}>()

const buttons = computed(() => {
  const editor = props.editor
  if (!editor) return []

  return [
    {
      label: '正文',
      icon: 'i-lucide-pilcrow',
      active: editor.isActive('paragraph'),
      action: () => editor.chain().focus().setParagraph().run()
    },
    {
      label: '标题 1',
      icon: 'i-lucide-heading-1',
      active: editor.isActive('heading', {level: 1}),
      action: () => editor.chain().focus().toggleHeading({level: 1}).run()
    },
    {
      label: '标题 2',
      icon: 'i-lucide-heading-2',
      active: editor.isActive('heading', {level: 2}),
      action: () => editor.chain().focus().toggleHeading({level: 2}).run()
    },
    {
      label: '加粗',
      icon: 'i-lucide-bold',
      active: editor.isActive('bold'),
      action: () => editor.chain().focus().toggleBold().run()
    },
    {
      label: '斜体',
      icon: 'i-lucide-italic',
      active: editor.isActive('italic'),
      action: () => editor.chain().focus().toggleItalic().run()
    },
    {
      label: '引用',
      icon: 'i-lucide-quote',
      active: editor.isActive('blockquote'),
      action: () => editor.chain().focus().toggleBlockquote().run()
    },
    {
      label: '列表',
      icon: 'i-lucide-list',
      active: editor.isActive('bulletList'),
      action: () => editor.chain().focus().toggleBulletList().run()
    },
    {
      label: '撤销',
      icon: 'i-lucide-undo-2',
      active: false,
      action: () => editor.chain().focus().undo().run()
    },
    {
      label: '重做',
      icon: 'i-lucide-redo-2',
      active: false,
      action: () => editor.chain().focus().redo().run()
    }
  ]
})
</script>
