<template>
  <section aria-label="样式与内容选项" class="publication-panel">
    <header class="publication-panel-header">
      <div>
        <p class="publication-kicker">Style</p>
        <h2>内容与样式</h2>
        <p>这些设置会用于实时预览和最终导出。</p>
      </div>
    </header>

    <UForm :state="modelValue" class="publication-form publication-style-form">
      <section aria-label="内容选项" class="publication-check-grid">
        <UCheckbox v-model="includeTitle" label="包含作品标题"/>
        <UCheckbox v-model="includeAuthor" label="包含作者"/>
        <UCheckbox v-model="includeChapterTitle" label="包含章节标题"/>
        <UCheckbox v-model="firstLineIndent" label="正文首行缩进"/>
      </section>

      <UFormField label="章节标题格式" name="chapterTitleFormat">
        <UInput
            v-model="chapterTitleFormat"
            class="w-full"
            placeholder="第 {index} 章 {title}"
            size="sm"
        />
        <template #help>
          支持 {index}、{title}、{type}、{depth}。
        </template>
      </UFormField>

      <div class="publication-form-grid">
        <UFormField label="段落间距" name="paragraphSeparator">
          <USelect
              v-model="paragraphSeparator"
              :items="separatorItems"
              class="w-full"
              label-key="label"
              size="sm"
              value-key="value"
          />
        </UFormField>
        <UFormField label="正文字体" name="fontFamily">
          <USelect
              v-model="fontFamily"
              :items="fontItems"
              class="w-full"
              label-key="label"
              size="sm"
              value-key="value"
          />
        </UFormField>
      </div>

      <div class="publication-form-grid">
        <UFormField label="字号" name="fontSize">
          <UInputNumber v-model="fontSize" :max="72" :min="8" :step="0.5" class="w-full" size="sm"/>
        </UFormField>
        <UFormField label="行距" name="lineHeight">
          <div class="publication-slider-field">
            <USlider v-model="lineHeight" :max="3" :min="1" :step="0.05"/>
            <span>{{ lineHeight.toFixed(2) }}</span>
          </div>
        </UFormField>
      </div>

      <UCheckbox
          v-if="format === 'docx' || format === 'epub'"
          v-model="documentPageBreak"
          label="文档之间分页"
      />

      <template v-if="format === 'epub'">
        <section aria-label="EPUB 选项" class="publication-check-grid">
          <UCheckbox v-model="epubIncludeToc" label="生成 EPUB 目录"/>
          <UCheckbox v-model="epubIncludeAssets" label="打包项目图片资源"/>
        </section>
        <UFormField label="EPUB 出版者" name="epubPublisher">
          <UInput v-model="epubPublisher" class="w-full" maxlength="120" placeholder="Mythistorima" size="sm"/>
          <template #help>标题、作者、语言与项目封面会自动从项目资料读取。</template>
        </UFormField>
      </template>

      <template v-if="format === 'pixiv'">
        <section aria-label="Pixiv 选项" class="publication-check-grid">
          <UCheckbox v-model="pixivPageBreak" label="文档之间插入 [newpage]"/>
          <UCheckbox v-model="pixivConvertRuby" label="转换 ｜文字《注音》"/>
          <UCheckbox v-model="pixivConvertEmphasis" label="转换强调标记"/>
        </section>
        <UFormField label="Pixiv 标签" name="pixivTags">
          <UInputTags v-model="pixivTags" class="w-full" placeholder="输入标签后回车" size="sm"/>
          <template #help>最多 10 个，导出文本顶部会生成 #标签 行。</template>
        </UFormField>
      </template>
    </UForm>
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

const separatorItems = [
  {label: '单换行', value: '\n'},
  {label: '空一行', value: '\n\n'},
  {label: '空两行', value: '\n\n\n'}
]
const fontItems: { label: string, value: ExportFontFamily }[] = [
  {label: '衬线', value: 'serif'},
  {label: '无衬线', value: 'sans-serif'},
  {label: '等宽', value: 'monospace'},
  {label: '系统字体', value: 'system'}
]

function update(patch: Partial<ExportTemplateConfig>) {
  emit('update:modelValue', {...props.modelValue, ...patch})
}

function booleanModel(key: keyof Pick<ExportTemplateConfig,
    'includeTitle' | 'includeAuthor' | 'includeChapterTitle' | 'firstLineIndent' | 'documentPageBreak' |
    'epubIncludeToc' | 'epubIncludeAssets' | 'pixivPageBreak' | 'pixivConvertRuby' | 'pixivConvertEmphasis'>) {
  return computed({
    get: () => Boolean(props.modelValue[key]),
    set: value => update({[key]: Boolean(value)} as Partial<ExportTemplateConfig>)
  })
}

const includeTitle = booleanModel('includeTitle')
const includeAuthor = booleanModel('includeAuthor')
const includeChapterTitle = booleanModel('includeChapterTitle')
const firstLineIndent = booleanModel('firstLineIndent')
const documentPageBreak = booleanModel('documentPageBreak')
const epubIncludeToc = booleanModel('epubIncludeToc')
const epubIncludeAssets = booleanModel('epubIncludeAssets')
const pixivPageBreak = booleanModel('pixivPageBreak')
const pixivConvertRuby = booleanModel('pixivConvertRuby')
const pixivConvertEmphasis = booleanModel('pixivConvertEmphasis')

const chapterTitleFormat = computed({
  get: () => props.modelValue.chapterTitleFormat,
  set: value => update({chapterTitleFormat: value})
})
const paragraphSeparator = computed({
  get: () => props.modelValue.paragraphSeparator,
  set: value => update({paragraphSeparator: value})
})
const fontFamily = computed({
  get: () => props.modelValue.fontFamily,
  set: value => update({fontFamily: value as ExportFontFamily})
})
const fontSize = computed({
  get: () => props.modelValue.fontSize,
  set: value => update({fontSize: Number(value)})
})
const lineHeight = computed({
  get: () => props.modelValue.lineHeight,
  set: value => update({lineHeight: Number(value)})
})
const epubPublisher = computed({
  get: () => props.modelValue.epubPublisher,
  set: value => update({epubPublisher: value})
})
const pixivTags = computed({
  get: () => props.modelValue.pixivTags,
  set: value => update({pixivTags: value.map(tag => tag.trim().replace(/^#/, '')).filter(Boolean).slice(0, 10)})
})
</script>
