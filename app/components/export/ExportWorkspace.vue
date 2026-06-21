<template>
  <section class="export-workspace glass-panel">
    <header class="workspace-section-header">
      <div>
        <p class="eyebrow">Export / Import / Backup</p>
        <h2>导入导出与备份</h2>
        <p>支持 txt、Markdown、HTML、项目包导出，以及文本文件导入和本地 SQLite 备份。</p>
      </div>
    </header>

    <div class="export-grid">
      <article class="export-card">
        <h3>导出文档</h3>
        <label>格式
          <select v-model="exportFormat" class="text-field">
            <option value="txt">TXT</option>
            <option value="markdown">Markdown</option>
            <option value="html">HTML</option>
          </select>
        </label>
        <label>范围
          <select v-model="exportRange" class="text-field">
            <option value="all">全项目</option>
            <option value="current">当前文档及子文档</option>
          </select>
        </label>
        <label>输出路径，可留空
          <input v-model="outputPath" class="text-field" placeholder="留空则导出到应用 exports 目录">
        </label>
        <button :disabled="exportStore.busy" class="primary-button full-width-button" type="button" @click="exportDocs">
          {{ exportStore.busy ? '处理中…' : '导出' }}
        </button>
      </article>

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
        <p>创建当前数据库的项目备份快照。应用启动时也会自动创建一次备份。</p>
        <button :disabled="exportStore.busy" class="primary-button full-width-button" type="button"
                @click="createBackup">
          创建备份
        </button>
        <div class="backup-list">
          <article v-for="backup in exportStore.backups" :key="backup.id" class="backup-item">
            <strong>{{ formatDate(backup.createdAt) }}</strong>
            <span>{{ formatSize(backup.sizeBytes) }}</span>
            <small>{{ backup.path }}</small>
          </article>
        </div>
      </article>
    </div>

    <p v-if="exportStore.error" class="editor-error">{{ exportStore.error }}</p>
    <p v-if="message" class="subtle-message">{{ message }}</p>
  </section>
</template>

<script lang="ts" setup>
import type {ExportFormat, ExportRange} from '~/types/export'

const props = defineProps<{ projectId: string; activeDocumentId: string | null }>()
const emit = defineEmits<{ (event: 'imported'): void }>()

const exportStore = useExportStore()
const exportFormat = ref<Exclude<ExportFormat, 'project_package'>>('markdown')
const exportRange = ref<ExportRange>('all')
const outputPath = ref('')
const packagePath = ref('')
const importPath = ref('')
const importFormat = ref<'txt' | 'markdown' | 'html'>('markdown')
const importTitle = ref('')
const message = ref<string | null>(null)

onMounted(async () => {
  await exportStore.listBackups(props.projectId)
})

async function exportDocs() {
  message.value = null
  const result = await exportStore.exportDocuments({
    projectId: props.projectId,
    format: exportFormat.value,
    range: exportRange.value,
    documentId: exportRange.value === 'current' ? props.activeDocumentId : null,
    outputPath: outputPath.value.trim() || null
  })
  message.value = `已导出 ${result.documentCount} 个文档到：${result.path}`
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
