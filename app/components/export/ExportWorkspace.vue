<template>
  <section aria-label="导出与发布工作区" class="publication-workspace" tabindex="0">
    <header class="publication-toolbar">
      <div class="publication-toolbar-title">
        <UIcon name="i-lucide-file-output"/>
        <div>
          <strong>{{ editorDraft.name || '导出模板' }}</strong>
          <span>{{ formatLabel }} · {{ rangeLabel }} · {{
              templateStore.preview ? '预览已生成' : '实时预览待生成'
            }}</span>
        </div>
      </div>

      <div class="publication-toolbar-controls">
        <div aria-label="预览比例" class="publication-zoom-control">
          <UIcon name="i-lucide-zoom-in"/>
          <USlider v-model="previewZoom" :max="140" :min="60" :step="5" class="publication-zoom-slider"/>
          <span>{{ previewZoom }}%</span>
        </div>
        <UTooltip text="重新生成预览（Ctrl+R）">
          <UButton
              :disabled="!canPreview"
              :loading="templateStore.previewing"
              aria-label="重新生成预览"
              color="neutral"
              icon="i-lucide-refresh-cw"
              label="重新生成"
              size="sm"
              variant="ghost"
              @click="previewTemplate()"
          />
        </UTooltip>
        <UTooltip text="导出（Ctrl+Enter）">
          <UButton
              :disabled="!canExport"
              :loading="templateStore.exporting"
              icon="i-lucide-upload"
              label="导出"
              size="sm"
              @click="exportTemplate"
          />
        </UTooltip>
      </div>
    </header>

    <div class="publication-layout">
      <aside aria-label="导出设置" class="publication-settings">
        <UTabs
            v-model="settingsTab"
            :content="false"
            :items="settingsTabs"
            class="publication-settings-tabs"
            color="neutral"
            size="sm"
            variant="link"
        />

        <div class="publication-settings-scroll">
          <template v-if="settingsTab === 'template'">
            <ExportTemplateList
                :active-template-id="templateStore.activeTemplateId"
                :loading="templateStore.loading"
                :templates="templateStore.templates"
                @create="startCreating"
                @select="selectTemplate"
            />
            <ExportTemplateEditor
                v-model="editorDraft"
                :creating="creating"
                :saving="templateStore.saving"
                @delete="deleteTemplate"
                @duplicate="duplicateTemplate"
                @save="saveTemplate"
            />
          </template>

          <ExportRangeAdvancedPicker
              v-else-if="settingsTab === 'range'"
              v-model="exportRange"
              v-model:selected-document-ids="selectedDocumentIds"
              :active-document-id="activeDocumentId"
              :documents="documents"
          />

          <ExportStyleEditor
              v-else-if="settingsTab === 'style'"
              v-model="editorDraft.config"
              :format="editorDraft.format"
          />

          <section v-else aria-label="发布操作" class="publication-panel">
            <header class="publication-panel-header">
              <div>
                <p class="publication-kicker">Publish</p>
                <h2>发布</h2>
                <p>输出路径可留空，留空时会导出到应用默认 exports 目录。</p>
              </div>
            </header>

            <UForm :state="publishState" class="publication-form">
              <UFormField label="输出路径" name="outputPath">
                <UInput
                    v-model="outputPath"
                    class="w-full"
                    placeholder="留空则导出到应用 exports 目录"
                    size="sm"
                />
              </UFormField>

              <div class="publication-actions">
                <UButton
                    :disabled="!canPreview"
                    :loading="templateStore.previewing"
                    color="neutral"
                    icon="i-lucide-file-search"
                    label="生成预览"
                    size="sm"
                    variant="soft"
                    @click="previewTemplate()"
                />
                <UButton
                    :disabled="!canExport"
                    :loading="templateStore.exporting"
                    icon="i-lucide-upload"
                    label="导出文件"
                    size="sm"
                    @click="exportTemplate"
                />
              </div>
            </UForm>

            <UAlert
                v-if="creating"
                color="warning"
                description="请先保存新模板，再生成预览或导出。"
                icon="i-lucide-circle-alert"
                title="模板尚未保存"
                variant="soft"
            />
            <UAlert
                v-if="exportRange === 'selected' && !selectedDocumentIds.length"
                color="warning"
                description="自选范围至少需要勾选一个文档。"
                icon="i-lucide-list-checks"
                title="未选择导出文档"
                variant="soft"
            />

            <section class="publication-utility-section">
              <header>
                <strong>项目包</strong>
                <span>导出项目、文档、设定卡和事项为单个 JSON 项目包。</span>
              </header>
              <UInput v-model="packagePath" placeholder="项目包输出路径，可留空" size="sm"/>
              <UButton
                  :loading="exportStore.busy"
                  block
                  color="neutral"
                  icon="i-lucide-package"
                  label="导出项目包"
                  size="sm"
                  variant="soft"
                  @click="exportPackage"
              />
            </section>

            <section class="publication-utility-section">
              <header>
                <strong>导入文本</strong>
                <span>导入 txt / md / html 为新章节。</span>
              </header>
              <UInput v-model="importPath" placeholder="例如 D:\\novel\\chapter1.md" size="sm"/>
              <div class="publication-form-grid">
                <USelect
                    v-model="importFormat"
                    :items="importFormatItems"
                    class="w-full"
                    label-key="label"
                    size="sm"
                    value-key="value"
                />
                <UInput v-model="importTitle" placeholder="章节标题，可留空" size="sm"/>
              </div>
              <UButton
                  :disabled="!importPath.trim()"
                  :loading="exportStore.busy"
                  block
                  color="neutral"
                  icon="i-lucide-file-plus-2"
                  label="导入为新章节"
                  size="sm"
                  variant="soft"
                  @click="importFile"
              />
            </section>

            <section class="publication-utility-section">
              <header>
                <strong>备份</strong>
                <span>创建当前项目的本地备份。</span>
              </header>
              <UButton
                  :loading="exportStore.busy"
                  block
                  icon="i-lucide-hard-drive-download"
                  label="创建备份"
                  size="sm"
                  @click="createBackup"
              />
              <div v-if="exportStore.busy && !exportStore.backups.length" class="publication-skeleton-stack">
                <USkeleton v-for="index in 3" :key="index" class="h-10 w-full"/>
              </div>
              <UEmpty
                  v-else-if="!exportStore.backups.length"
                  description="创建备份后会显示最近备份。"
                  icon="i-lucide-database-backup"
                  title="暂无备份"
              />
              <div v-else class="publication-backup-list">
                <article v-for="backup in exportStore.backups" :key="backup.id" class="publication-backup-item">
                  <strong>{{ formatDate(backup.createdAt) }}</strong>
                  <span>{{ formatSize(backup.sizeBytes) }}</span>
                </article>
              </div>
            </section>
          </section>
        </div>
      </aside>

      <ExportTemplatePreview
          :live="Boolean(templateStore.preview) && canPreview"
          :loading="templateStore.previewing"
          :preview="templateStore.preview"
          :zoom="previewZoom"
      />
    </div>

    <UAlert
        v-if="errorMessage"
        :description="errorMessage"
        class="publication-alert"
        close
        color="error"
        icon="i-lucide-circle-alert"
        title="导出操作失败"
        variant="soft"
        @close="clearErrors"
    />
  </section>
</template>

<script lang="ts" setup>
import ExportRangeAdvancedPicker from '~/components/export/ExportRangeAdvancedPicker.vue'
import ExportStyleEditor from '~/components/export/ExportStyleEditor.vue'
import ExportTemplateEditor from '~/components/export/ExportTemplateEditor.vue'
import ExportTemplateList from '~/components/export/ExportTemplateList.vue'
import ExportTemplatePreview from '~/components/export/ExportTemplatePreview.vue'
import {useExportStore} from '~/stores/export.store'
import {useExportTemplateStore} from '~/stores/exportTemplate.store'
import type {ExportRange} from '~/types/export'
import type {NovelDocument} from '~/types/document'
import type {ExportTemplateDraft, ExportTemplateFormat} from '~/types/exportTemplate'
import {
  defaultExportTemplateConfig,
  exportTemplateFormatLabel,
  parseExportTemplateConfig,
  serializeExportTemplateConfig
} from '~/utils/exportTemplate'

const props = defineProps<{
  projectId: string
  activeDocumentId: string | null
  documents: NovelDocument[]
}>()
const emit = defineEmits<{ imported: [] }>()

const exportStore = useExportStore()
const templateStore = useExportTemplateStore()
const toast = useToast()
const creating = ref(false)
const editorDraft = ref<ExportTemplateDraft>(emptyDraft())
const exportRange = ref<ExportRange>('all')
const selectedDocumentIds = ref<string[]>([])
const outputPath = ref('')
const packagePath = ref('')
const importPath = ref('')
const importFormat = ref<'txt' | 'markdown' | 'html'>('markdown')
const importTitle = ref('')
const previewZoom = ref(100)
const settingsTab = ref<'template' | 'range' | 'style' | 'publish'>('template')
const publishState = computed(() => ({outputPath: outputPath.value}))

const settingsTabs = [
  {label: '模板', value: 'template', icon: 'i-lucide-layout-template'},
  {label: '范围', value: 'range', icon: 'i-lucide-list-tree'},
  {label: '样式', value: 'style', icon: 'i-lucide-palette'},
  {label: '发布', value: 'publish', icon: 'i-lucide-upload'}
]
const importFormatItems = [
  {label: 'TXT', value: 'txt'},
  {label: 'Markdown', value: 'markdown'},
  {label: 'HTML', value: 'html'}
]
const formatLabel = computed(() => exportTemplateFormatLabel(editorDraft.value.format))
const rangeLabel = computed(() => {
  if (exportRange.value === 'current') return '当前文档'
  if (exportRange.value === 'selected') return `自选 ${selectedDocumentIds.value.length} 项`
  return '全项目'
})
const canUseTemplate = computed(
    () => !creating.value && Boolean(templateStore.activeTemplateId && editorDraft.value)
)
const canPreview = computed(
    () => canUseTemplate.value
        && (exportRange.value !== 'current' || Boolean(props.activeDocumentId))
        && (exportRange.value !== 'selected' || selectedDocumentIds.value.length > 0)
)
const canExport = computed(() => canPreview.value)
const errorMessage = computed(() => templateStore.error || exportStore.error)

let previewTimer: ReturnType<typeof setTimeout> | null = null

onMounted(async () => {
  window.addEventListener('keydown', handleKeydown)
  try {
    await Promise.all([
      templateStore.loadTemplates({projectId: props.projectId, includeBuiltin: true}),
      exportStore.listBackups(props.projectId)
    ])
    resetDraftFromTemplate()
    schedulePreview()
  } catch {
    notify('导出数据加载失败', errorMessage.value ?? '请稍后重试。', 'error')
  }
})

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleKeydown)
  if (previewTimer) clearTimeout(previewTimer)
})

watch(
    () => [templateStore.activeTemplateId, creating.value] as const,
    () => {
      resetDraftFromTemplate()
      schedulePreview()
    }
)

watch(
    () => [
      exportRange.value,
      selectedDocumentIds.value.join('|'),
      props.activeDocumentId,
      editorDraft.value.format,
      JSON.stringify(editorDraft.value.config)
    ],
    () => schedulePreview()
)

watch(
    () => props.documents.map(document => document.id).join('|'),
    () => {
      selectedDocumentIds.value = selectedDocumentIds.value.filter(
          id => props.documents.some(document => document.id === id)
      )
      schedulePreview()
    }
)

function emptyDraft(format: ExportTemplateFormat = 'txt'): ExportTemplateDraft {
  return {
    templateId: null,
    name: '新的导出模板',
    format,
    config: defaultExportTemplateConfig(format),
    isBuiltin: false
  }
}

function resetDraftFromTemplate() {
  if (!templateStore.activeTemplate || creating.value) {
    editorDraft.value = emptyDraft()
    return
  }
  editorDraft.value = {
    templateId: templateStore.activeTemplate.id,
    name: templateStore.activeTemplate.name,
    format: templateStore.activeTemplate.format,
    config: parseExportTemplateConfig(templateStore.activeTemplate.configJson, templateStore.activeTemplate.format),
    isBuiltin: templateStore.activeTemplate.isBuiltin !== 0
  }
}

function selectTemplate(templateId: string) {
  creating.value = false
  templateStore.selectTemplate(templateId)
}

function startCreating() {
  creating.value = true
  templateStore.selectTemplate(null)
  templateStore.clearPreview()
  settingsTab.value = 'template'
  editorDraft.value = emptyDraft()
}

async function saveTemplate(draft: ExportTemplateDraft) {
  try {
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
      notify('模板已保存', `“${template.name}”已用于实时预览。`)
      schedulePreview()
      return
    }

    const template = await templateStore.createTemplate({
      projectId: props.projectId,
      name: draft.name.trim(),
      format: draft.format,
      configJson
    })
    creating.value = false
    notify('模板已创建', `“${template.name}”已加入项目模板。`)
    schedulePreview()
  } catch {
    notify('保存模板失败', errorMessage.value ?? '请检查模板设置。', 'error')
  }
}

async function duplicateTemplate(draft: ExportTemplateDraft) {
  try {
    const template = await templateStore.createTemplate({
      projectId: props.projectId,
      name: `${draft.name} 副本`,
      format: draft.format,
      configJson: serializeExportTemplateConfig(draft.config)
    })
    creating.value = false
    notify('已复制模板', `项目模板“${template.name}”已创建。`)
    schedulePreview()
  } catch {
    notify('复制模板失败', errorMessage.value ?? '请稍后重试。', 'error')
  }
}

async function deleteTemplate(templateId: string) {
  try {
    await templateStore.deleteTemplate(props.projectId, templateId)
    creating.value = false
    notify('模板已删除', '已切换到可用模板。')
    schedulePreview()
  } catch {
    notify('删除模板失败', errorMessage.value ?? '请稍后重试。', 'error')
  }
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
    configOverrideJson: configOverride
        ? serializeExportTemplateConfig(editorDraft.value.config)
        : null
  }
}

function schedulePreview() {
  if (previewTimer) clearTimeout(previewTimer)
  if (!canPreview.value) {
    templateStore.clearPreview()
    return
  }
  previewTimer = setTimeout(() => previewTemplate({silent: true}), 300)
}

async function previewTemplate(options: { silent?: boolean } = {}) {
  const input = templateInput()
  if (!input || !canPreview.value) return
  try {
    await templateStore.previewWithTemplate({...input, outputPath: null})
    if (!options.silent) notify('预览已更新', '预览与最终导出使用同一份配置。')
  } catch {
    if (!options.silent) notify('生成预览失败', errorMessage.value ?? '请检查导出范围。', 'error')
  }
}

async function exportTemplate() {
  const input = templateInput()
  if (!input || !canExport.value) return
  try {
    const result = await templateStore.exportWithTemplate(input)
    notify('导出完成', `${result.documentCount} 个文档已导出到：${result.path}`)
  } catch {
    notify('导出失败', errorMessage.value ?? '请检查输出路径和模板设置。', 'error')
  }
}

async function exportPackage() {
  try {
    const result = await exportStore.exportProject({
      projectId: props.projectId,
      format: 'project_package',
      outputPath: packagePath.value.trim() || null
    })
    notify('项目包已导出', result.path)
  } catch {
    notify('项目包导出失败', errorMessage.value ?? '请稍后重试。', 'error')
  }
}

async function importFile() {
  try {
    const result = await exportStore.importTextFile({
      projectId: props.projectId,
      path: importPath.value.trim(),
      format: importFormat.value,
      title: importTitle.value.trim() || null
    })
    notify('导入完成', `“${result.title}”，${result.characterCount} 字。`)
    emit('imported')
  } catch {
    notify('导入失败', errorMessage.value ?? '请检查文件路径。', 'error')
  }
}

async function createBackup() {
  try {
    const backup = await exportStore.createBackup(props.projectId)
    await exportStore.listBackups(props.projectId)
    notify('备份已创建', backup.path)
  } catch {
    notify('创建备份失败', errorMessage.value ?? '请稍后重试。', 'error')
  }
}

function handleKeydown(event: KeyboardEvent) {
  const target = event.target as HTMLElement | null
  const isFormField = target?.closest('input, textarea, [contenteditable="true"]')
  const mod = event.metaKey || event.ctrlKey
  if (event.altKey && ['1', '2', '3', '4'].includes(event.key)) {
    event.preventDefault()
    settingsTab.value = settingsTabs[Number(event.key) - 1]!.value as 'template' | 'range' | 'style' | 'publish'
    return
  }
  if (!mod) return
  const key = event.key.toLowerCase()
  if (key === 'enter') {
    event.preventDefault()
    exportTemplate()
  } else if (key === 'r') {
    event.preventDefault()
    previewTemplate()
  } else if (key === 's' && !isFormField) {
    event.preventDefault()
    if (!editorDraft.value.isBuiltin) saveTemplate(editorDraft.value)
  }
}

function clearErrors() {
  templateStore.error = null
  exportStore.error = null
}

function notify(title: string, description: string, color: 'success' | 'error' | 'warning' = 'success') {
  toast.add({title, description, color})
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
