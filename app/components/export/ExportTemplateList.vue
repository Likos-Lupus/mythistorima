<template>
  <aside class="export-template-list glass-panel">
    <header class="export-template-panel-header">
      <div>
        <p class="eyebrow">Reusable templates</p>
        <h3>导出模板</h3>
      </div>
      <button class="primary-button compact-button" type="button" @click="$emit('create')">
        新建
      </button>
    </header>

    <div v-if="loading" class="export-template-empty">正在加载模板…</div>
    <div v-else-if="!templates.length" class="export-template-empty">暂无导出模板。</div>

    <div v-else class="export-template-items">
      <button
          v-for="template in templates"
          :key="template.id"
          :class="{ 'is-active': template.id === activeTemplateId }"
          class="export-template-item"
          type="button"
          @click="$emit('select', template.id)"
      >
        <span class="export-template-item-main">
          <strong>{{ template.name }}</strong>
          <small>{{ formatLabel(template.format) }}</small>
        </span>
        <span v-if="template.isBuiltin" class="export-template-badge">内置</span>
        <span v-else class="export-template-badge is-project">项目</span>
      </button>
    </div>
  </aside>
</template>

<script lang="ts" setup>
import type {ExportTemplate} from '~/types/exportTemplate'
import {exportTemplateFormatLabel} from '~/utils/exportTemplate'

defineProps<{
  templates: ExportTemplate[]
  activeTemplateId: string | null
  loading?: boolean
}>()

defineEmits<{
  select: [templateId: string]
  create: []
}>()

function formatLabel(format: ExportTemplate['format']) {
  return exportTemplateFormatLabel(format)
}
</script>
