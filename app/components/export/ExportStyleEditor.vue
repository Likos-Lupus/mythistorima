<template>
  <section class="export-style-editor">
    <header class="export-template-panel-header">
      <div>
        <h4>版式与发布配置</h4>
        <p>通用字段会用于 HTML、DOCX 与 EPUB；Pixiv 使用平台专用标记。</p>
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
        正文字体
        <select :value="modelValue.fontFamily" class="text-field" @change="patchText('fontFamily', $event)">
          <option value="serif">衬线</option>
          <option value="sans-serif">无衬线</option>
          <option value="monospace">等宽</option>
          <option value="system">系统字体</option>
        </select>
      </label>

      <label>
        字号（pt）
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
        行距
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

      <label v-if="format === 'docx' || format === 'epub'" class="export-check-row export-style-wide">
        <input
            :checked="modelValue.documentPageBreak"
            type="checkbox"
            @change="patchBoolean('documentPageBreak', $event)"
        >
        文档之间分页
      </label>

      <template v-if="format === 'epub'">
        <label class="export-check-row">
          <input
              :checked="modelValue.epubIncludeToc"
              type="checkbox"
              @change="patchBoolean('epubIncludeToc', $event)"
          >
          生成 EPUB 目录
        </label>
        <label class="export-check-row">
          <input
              :checked="modelValue.epubIncludeAssets"
              type="checkbox"
              @change="patchBoolean('epubIncludeAssets', $event)"
          >
          打包项目图片资源
        </label>
        <label class="export-style-wide">
          EPUB 出版者
          <input
              :value="modelValue.epubPublisher"
              class="text-field"
              maxlength="120"
              placeholder="Mythistorima"
              @input="patchText('epubPublisher', $event)"
          >
          <small>标题、作者、语言与项目封面会自动从项目资料读取。</small>
        </label>
      </template>

      <template v-if="format === 'pixiv'">
        <label class="export-check-row">
          <input
              :checked="modelValue.pixivPageBreak"
              type="checkbox"
              @change="patchBoolean('pixivPageBreak', $event)"
          >
          文档之间插入 <code>[newpage]</code>
        </label>
        <label class="export-check-row">
          <input
              :checked="modelValue.pixivConvertRuby"
              type="checkbox"
              @change="patchBoolean('pixivConvertRuby', $event)"
          >
          转换 <code>｜文字《注音》</code>
        </label>
        <label class="export-check-row">
          <input
              :checked="modelValue.pixivConvertEmphasis"
              type="checkbox"
              @change="patchBoolean('pixivConvertEmphasis', $event)"
          >
          转换强调标记
        </label>
        <label class="export-style-wide">
          Pixiv 标签
          <input
              :value="modelValue.pixivTags.join(', ')"
              class="text-field"
              placeholder="原创, 奇幻, 长篇"
              @input="patchPixivTags"
          >
          <small>最多 10 个，以逗号分隔；导出文本顶部会生成 <code>#标签</code> 行。</small>
        </label>
      </template>
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

function patchText(
    key: 'chapterTitleFormat' | 'paragraphSeparator' | 'fontFamily' | 'epubPublisher',
    event: Event
) {
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

function patchPixivTags(event: Event) {
  const tags = (event.target as HTMLInputElement).value
      .split(/[,，]/)
      .map(tag => tag.trim().replace(/^#/, ''))
      .filter(Boolean)
      .slice(0, 10)
  update({pixivTags: tags})
}
</script>
