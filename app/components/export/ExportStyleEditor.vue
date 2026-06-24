<template>
  <section class="export-style-editor">
    <header class="export-template-panel-header">
      <div>
        <h4>版式配置</h4>
        <p>控制标题、段落和 HTML 样式。未使用的样式字段会安全保留。</p>
      </div>
    </header>

    <div class="export-style-grid">
      <label class="export-check-row">
        <input :checked="modelValue.includeTitle" type="checkbox" @change="patchBoolean('includeTitle', $event)">
        包含作品标题
      </label>
      <label class="export-check-row">
        <input :checked="modelValue.includeAuthor" type="checkbox" @change="patchBoolean('includeAuthor', $event)">
        包含作者
      </label>
      <label class="export-check-row">
        <input
            :checked="modelValue.includeChapterTitle"
            type="checkbox"
            @change="patchBoolean('includeChapterTitle', $event)"
        >
        包含章节标题
      </label>
      <label class="export-check-row">
        <input
            :checked="modelValue.firstLineIndent"
            type="checkbox"
            @change="patchBoolean('firstLineIndent', $event)"
        >
        正文首行缩进
      </label>

      <label class="export-style-wide">
        章节标题格式
        <input
            :value="modelValue.chapterTitleFormat"
            class="text-field"
            placeholder="第 {index} 章 {title}"
            @input="patchText('chapterTitleFormat', $event)"
        >
        <small>支持 <code>{index}</code>、<code>{title}</code>、<code>{type}</code>、<code>{depth}</code>。</small>
      </label>

      <label>
        段落间距
        <select
            :value="modelValue.paragraphSeparator"
            class="text-field"
            @change="patchText('paragraphSeparator', $event)"
        >
          <option :value="'\n'">单换行</option>
          <option :value="'\n\n'">空一行</option>
          <option :value="'\n\n\n'">空两行</option>
        </select>
      </label>

      <label>
        HTML 字体
        <select :value="modelValue.fontFamily" class="text-field" @change="patchText('fontFamily', $event)">
          <option value="serif">衬线</option>
          <option value="sans-serif">无衬线</option>
          <option value="monospace">等宽</option>
          <option value="system">系统字体</option>
        </select>
      </label>

      <label>
        HTML 字号（pt）
        <input
            :value="modelValue.fontSize"
            class="text-field"
            max="72"
            min="8"
            step="0.5"
            type="number"
            @input="patchNumber('fontSize', $event)"
        >
      </label>

      <label>
        HTML 行距
        <input
            :value="modelValue.lineHeight"
            class="text-field"
            max="3"
            min="1"
            step="0.05"
            type="number"
            @input="patchNumber('lineHeight', $event)"
        >
      </label>

      <label v-if="format === 'pixiv'" class="export-check-row export-style-wide">
        <input
            :checked="modelValue.pixivPageBreak"
            type="checkbox"
            @change="patchBoolean('pixivPageBreak', $event)"
        >
        文档之间插入 <code>[newpage]</code>
      </label>
    </div>
  </section>
</template>

<script lang="ts" setup>
import type {ExportFontFamily, ExportTemplateConfig, ExportTemplateFormat} from '~/types/exportTemplate'

const props = defineProps<{
  modelValue: ExportTemplateConfig
  format: ExportTemplateFormat
}>()

const emit = defineEmits<{
  'update:modelValue': [value: ExportTemplateConfig]
}>()

function update(patch: Partial<ExportTemplateConfig>) {
  emit('update:modelValue', {...props.modelValue, ...patch})
}

function patchBoolean(key: keyof ExportTemplateConfig, event: Event) {
  update({[key]: (event.target as HTMLInputElement).checked} as Partial<ExportTemplateConfig>)
}

function patchText(key: 'chapterTitleFormat' | 'paragraphSeparator' | 'fontFamily', event: Event) {
  const value = (event.target as HTMLInputElement | HTMLSelectElement).value
  if (key === 'fontFamily') {
    update({fontFamily: value as ExportFontFamily})
  } else {
    update({[key]: value} as Partial<ExportTemplateConfig>)
  }
}

function patchNumber(key: 'fontSize' | 'lineHeight', event: Event) {
  const value = Number((event.target as HTMLInputElement).value)
  if (Number.isFinite(value)) update({[key]: value})
}
</script>
