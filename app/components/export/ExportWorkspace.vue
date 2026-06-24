<template>
  <section class="export-workspace">
    <header class="workspace-section-header glass-panel export-workspace-header">
      <div>
        <p class="eyebrow">Advanced export templates</p>
        <h2>高级导出模板</h2>
        <p>保存可复用的 TXT、Markdown、HTML、DOCX、EPUB 与 Pixiv 版式，并按全项目、当前树或自选文档导出。</p>
      </div>
    </header>

    <div class="export-template-workspace">
      <ExportTemplateList
          :active-template-id="templateStore.activeTemplateId"
          :loading="templateStore.loading"
          :templates="templateStore.templates"
          @create="startCreating"
          @select="selectTemplate"
      />

      <ExportTemplateEditor
          :creating="creating"
          :saving="templateStore.saving"
          :template="templateStore.activeTemplate"
          @delete="deleteTemplate"
          @duplicate="duplicateTemplate"
          @save="saveTemplate"
          @draft-change="editorDraft = $event"
      />

      <div class="export-template-output-stack">
        <ExportRangeAdvancedPicker
            v-model="exportRange"
            v-model:selected-document-ids="selectedDocumentIds"
            :active-document-id="activeDocumentId"
            :documents="documents"
        />

        <section class="export-output-card glass-panel">
          <label>
            输出路径，可留空
            <input v-model="outputPath" class="text-field" placeholder="留空则导出到应用 exports 目录">
          </label>
          <div class="export-output-actions">
            <button
                :disabled="!canUseTemplate || templateStore.previewing"
                class="secondary-button"
                type="button"
                @click="previewTemplate"
            >
              {{ templateStore.previewing ? '生成中…' : '生成预览' }}
            </button>
            <button
                :disabled="!canExport || templateStore.exporting"
                class="primary-button"
                type="button"
                @click="exportTemplate"
            >
              {{ templateStore.exporting ? '导出中…' : '使用模板导出' }}
            </button>
          </div>
          <p v-if="creating" class="subtle-message">请先保存新模板，再生成预览或导出。</p>
          <p v-if="exportRange === 'selected' && !selectedDocumentIds.length" class="editor-error">
            自选范围至少需要勾选一个文档。
          </p>
        </section>
      </div>
    </div>

    <ExportTemplatePreview
        :loading="templateStore.previewing"
        :preview="templateStore.preview"
    />

    <details class="export-legacy-tools glass-panel">
      <summary>项目包、文本导入与本地备份</summary>
      <div class="export-grid">
        <article class="export-card">
          <h3>项目包</h3>
          <p>导出项目、文档、设定卡和事项为单个 JSON 项目包。</p>
          <label>输出路径，可留空
            <input v-model="packagePath" class="text-field" placeholder="留空则导出到应用 exports 目录">
          </label>
          <button :disabled="exportStore.busy" class="secondary-button full-width-button" type="button"
                  @click="exportPackage">
            导出项目包
          </button>
        </article>

        <article class="export-card">
          <h3>导入文本</h3>
          <p>导入 txt / md / html 为新章节。当前阶段使用文件路径输入。</p>
          <label>文件路径
            <input v-model="importPath" class="text-field" placeholder="例如 D:\\novel\\chapter1.md">
          </label>
          <label>格式
            <select v-model="importFormat" class="text-field">
              <option value="txt">TXT</option>
              <option value="markdown">Markdown</option>
              <option value="html">HTML</option>
            </select>
          </label>
          <label>章节标题，可留空
            <input v-model="importTitle" class="text-field" placeholder="留空则使用文件名或 Markdown 标题">
          </label>
          <button :disabled="exportStore.busy || !importPath.trim()" class="secondary-button full-width-button"
                  type="button" @click="importFile">
            导入为新章节
          </button>
        </article>

        <article class="export-card">
          <h3>备份</h3>
          <p>创建当前项目的本地备份。应用启动时也会自动创建一次备份。</p>
          <button :disabled="exportStore.busy" class="primary-button full-width-button" type="button"
                  @click="createBackup">
            创建备份
          </button>
          <div class="backup-list">
            <article v-for="backup in exportStore.backups" :key="backup.id" class="backup-item">
              <strong>{{ formatDate(backup.createdAt) }}</strong>
              <span>{{ formatSize(backup.sizeBytes) }}</span>
              <small>本地备份</small>
            </article>
          </div>
        </article>
      </div>
    </details>

    <p v-if="templateStore.error || exportStore.error" class="editor-error">
      {{ templateStore.error || exportStore.error }}
    </p>
    <p v-if="message" class="subtle-message">{{ message }}</p>
  </section>
</template>

<script lang="ts" setup>
import ExportRangeAdvancedPicker from '~/components/export/ExportRangeAdvancedPicker.vue'
import ExportTemplateEditor from '~/components/export/ExportTemplateEditor.vue'
import ExportTemplateList from '~/components/export/ExportTemplateList.vue'
import ExportTemplatePreview from '~/components/export/ExportTemplatePreview.vue'
import {useExportStore} from '~/stores/export.store'
import {useExportTemplateStore} from '~/stores/exportTemplate.store'
import type {ExportRange} from '~/types/export'
import type {NovelDocument} from '~/types/document'
import type {ExportTemplateDraft} from '~/types/exportTemplate'
import {serializeExportTemplateConfig} from '~/utils/exportTemplate'

const props = defineProps<{
  projectId: string
  activeDocumentId: string | null
  documents: NovelDocument[]
}>()
const emit = defineEmits<{ imported: [] }>()

const exportStore = useExportStore()
const templateStore = useExportTemplateStore()
const creating = ref(false)
const editorDraft = ref<ExportTemplateDraft | null>(null)
const exportRange = ref<ExportRange>('all')
const selectedDocumentIds = ref<string[]>([])
const outputPath = ref('')
const packagePath = ref('')
const importPath = ref('')
const importFormat = ref<'txt' | 'markdown' | 'html'>('markdown')
const importTitle = ref('')
const message = ref<string | null>(null)

const canUseTemplate = computed(
    () => !creating.value && Boolean(templateStore.activeTemplateId && editorDraft.value)
)
const canExport = computed(
    () => canUseTemplate.value
        && (exportRange.value !== 'current' || Boolean(props.activeDocumentId))
        && (exportRange.value !== 'selected' || selectedDocumentIds.value.length > 0)
)

onMounted(async () => {
  await Promise.all([
    templateStore.loadTemplates({projectId: props.projectId, includeBuiltin: true}),
    exportStore.listBackups(props.projectId)
  ])
})

watch(
    () => props.documents.map(document => document.id).join('|'),
    () => {
      selectedDocumentIds.value = selectedDocumentIds.value.filter(
          id => props.documents.some(document => document.id === id)
      )
    }
)

function selectTemplate(templateId: string) {
  creating.value = false
  templateStore.selectTemplate(templateId)
}

function startCreating() {
  creating.value = true
  templateStore.selectTemplate(null)
  templateStore.clearPreview()
}

async function saveTemplate(draft: ExportTemplateDraft) {
  message.value = null
  const configJson = serializeExportTemplateConfig(draft.config)
  if (draft.templateId && !creating.value) {
    const template = await templateStore.updateTemplate({
      projectId: props.projectId,
      templateId: draft.templateId,
      name: draft.name.trim(),
      format: draft.format,
      configJson
    })
    creating.value = false
    message.value = `模板“${template.name}”已保存。`
    return
  }

  const template = await templateStore.createTemplate({
    projectId: props.projectId,
    name: draft.name.trim(),
    format: draft.format,
    configJson
  })
  creating.value = false
  message.value = `模板“${template.name}”已创建。`
}

async function duplicateTemplate(draft: ExportTemplateDraft) {
  const template = await templateStore.createTemplate({
    projectId: props.projectId,
    name: `${draft.name} 副本`,
    format: draft.format,
    configJson: serializeExportTemplateConfig(draft.config)
  })
  creating.value = false
  message.value = `已复制为项目模板“${template.name}”。`
}

async function deleteTemplate(templateId: string) {
  await templateStore.deleteTemplate(props.projectId, templateId)
  creating.value = false
  message.value = '项目模板已删除。'
}

function templateInput(configOverride = true) {
  if (!templateStore.activeTemplateId) return null
  return {
    projectId: props.projectId,
    templateId: templateStore.activeTemplateId,
    range: exportRange.value,
    documentId: exportRange.value === 'current' ? props.activeDocumentId : null,
    documentIds: exportRange.value === 'selected' ? selectedDocumentIds.value : null,
    outputPath: outputPath.value.trim() || null,
    configOverrideJson: configOverride && editorDraft.value
        ? serializeExportTemplateConfig(editorDraft.value.config)
        : null
  }
}

async function previewTemplate() {
  message.value = null
  const input = templateInput()
  if (!input) return
  await templateStore.previewWithTemplate({...input, outputPath: null})
}

async function exportTemplate() {
  message.value = null
  const input = templateInput()
  if (!input) return
  const result = await templateStore.exportWithTemplate(input)
  message.value = `已使用模板导出 ${result.documentCount} 个文档到：${result.path}`
}

async function exportPackage() {
  message.value = null
  const result = await exportStore.exportProject({
    projectId: props.projectId,
    format: 'project_package',
    outputPath: packagePath.value.trim() || null
  })
  message.value = `项目包已导出到：${result.path}`
}

async function importFile() {
  message.value = null
  const result = await exportStore.importTextFile({
    projectId: props.projectId,
    path: importPath.value.trim(),
    format: importFormat.value,
    title: importTitle.value.trim() || null
  })
  message.value = `已导入“${result.title}”，${result.characterCount} 字。`
  emit('imported')
}

async function createBackup() {
  message.value = null
  const backup = await exportStore.createBackup(props.projectId)
  await exportStore.listBackups(props.projectId)
  message.value = `备份已创建：${backup.path}`
}

function formatDate(timestamp: number) {
  return new Intl.DateTimeFormat('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  }).format(new Date(timestamp))
}

function formatSize(size: number) {
  if (size > 1024 * 1024) return `${(size / 1024 / 1024).toFixed(1)} MB`
  if (size > 1024) return `${(size / 1024).toFixed(1)} KB`
  return `${size} B`
}
</script>
