<template>
  <section class="export-template-preview glass-panel">
    <header class="export-template-panel-header">
      <div>
        <h3>导出预览</h3>
        <p v-if="preview">
          {{ preview.documentCount }} 个文档 · {{ preview.characterCount }} 字
          <span v-if="preview.truncated"> · 预览已截断</span>
        </p>
        <p v-else>选择模板和范围后生成预览。</p>
      </div>
    </header>

    <div v-if="loading" class="export-preview-empty">正在生成预览…</div>
    <div v-else-if="!preview" class="export-preview-empty">尚未生成预览。</div>
    <iframe
        v-else-if="['html', 'docx', 'epub'].includes(preview.format)"
        :srcdoc="preview.content"
        class="export-preview-frame"
        sandbox=""
        title="HTML 导出预览"
    />
    <pre v-else class="export-preview-text">{{ preview.content }}</pre>
  </section>
</template>

<script lang="ts" setup>
import type {ExportTemplatePreview} from '~/types/exportTemplate'

defineProps<{
  preview: ExportTemplatePreview | null
  loading?: boolean
}>()
</script>
