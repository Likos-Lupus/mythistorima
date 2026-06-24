<template>
  <article class="export-template-editor glass-panel">
    <header class="export-template-panel-header">
      <div>
        <p class="eyebrow">{{
            draft.isBuiltin ? 'Built-in template' : creating ? 'New template' : 'Project template'
          }}</p>
        <h3>{{ draft.isBuiltin ? '内置模板' : creating ? '创建项目模板' : '编辑项目模板' }}</h3>
        <p v-if="draft.isBuiltin">内置模板只读，可复制后调整。</p>
      </div>
      <span class="export-template-format-pill">{{ formatLabel }}</span>
    </header>

    <div class="export-template-form">
      <label>
        模板名称
        <input v-model="draft.name" :disabled="draft.isBuiltin" class="text-field" maxlength="120">
      </label>
      <label>
        格式
        <select v-model="draft.format" :disabled="draft.isBuiltin" class="text-field" @change="onFormatChange">
          <option value="txt">TXT</option>
          <option value="markdown">Markdown</option>
          <option value="html">HTML</option>
          <option value="docx">DOCX</option>
          <option value="epub">EPUB</option>
          <option value="pixiv">Pixiv</option>
        </select>
      </label>
    </div>

    <ExportStyleEditor
        v-model="draft.config"
        :class="{ 'is-readonly': draft.isBuiltin }"
        :format="draft.format"
    />

    <footer class="export-template-editor-actions">
      <button
          v-if="draft.isBuiltin"
          :disabled="saving"
          class="primary-button"
          type="button"
          @click="$emit('duplicate', draft)"
      >
        复制为项目模板
      </button>
      <template v-else>
        <button :disabled="saving || !draft.name.trim()" class="primary-button" type="button" @click="save">
          {{ saving ? '保存中…' : creating ? '创建模板' : '保存模板' }}
        </button>
        <button v-if="!creating" class="danger-button" type="button" @click="remove">
          删除
        </button>
      </template>
    </footer>
  </article>
</template>

<script lang="ts" setup>
import ExportStyleEditor from '~/components/export/ExportStyleEditor.vue'
import type {ExportTemplate, ExportTemplateDraft, ExportTemplateFormat} from '~/types/exportTemplate'
import {defaultExportTemplateConfig, exportTemplateFormatLabel, parseExportTemplateConfig} from '~/utils/exportTemplate'

const props = defineProps<{
  template: ExportTemplate | null
  creating?: boolean
  saving?: boolean
}>()

const emit = defineEmits<{
  save: [draft: ExportTemplateDraft]
  delete: [templateId: string]
  duplicate: [draft: ExportTemplateDraft]
  draftChange: [draft: ExportTemplateDraft]
}>()

const draft = reactive<ExportTemplateDraft>(emptyDraft())

const formatLabel = computed(() => exportTemplateFormatLabel(draft.format))

watch(
    () => [props.template?.id, props.creating] as const,
    () => resetDraft(),
    {immediate: true}
)

watch(
    draft,
    () => emit('draftChange', cloneDraft(draft)),
    {deep: true, immediate: true}
)

function emptyDraft(): ExportTemplateDraft {
  return {
    templateId: null,
    name: '新的导出模板',
    format: 'txt',
    config: defaultExportTemplateConfig('txt'),
    isBuiltin: false
  }
}

function resetDraft() {
  if (!props.template || props.creating) {
    Object.assign(draft, emptyDraft())
    return
  }
  Object.assign(draft, {
    templateId: props.template.id,
    name: props.template.name,
    format: props.template.format,
    config: parseExportTemplateConfig(props.template.configJson, props.template.format),
    isBuiltin: props.template.isBuiltin !== 0
  })
}

function onFormatChange() {
  const format = draft.format as ExportTemplateFormat
  const defaults = defaultExportTemplateConfig(format)
  draft.config = {
    ...draft.config,
    documentPageBreak: defaults.documentPageBreak,
    epubIncludeToc: defaults.epubIncludeToc,
    epubIncludeAssets: defaults.epubIncludeAssets,
    pixivPageBreak: defaults.pixivPageBreak
  }
}

function save() {
  emit('save', cloneDraft(draft))
}

function remove() {
  if (!draft.templateId) return
  if (!window.confirm(`确定删除导出模板“${draft.name}”吗？`)) return
  emit('delete', draft.templateId)
}

function cloneDraft(value: ExportTemplateDraft): ExportTemplateDraft {
  return {
    templateId: value.templateId ?? null,
    name: value.name,
    format: value.format,
    config: {...value.config},
    isBuiltin: value.isBuiltin
  }
}
</script>
