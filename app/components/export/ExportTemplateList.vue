<template>
  <section aria-label="导出模板列表" class="publication-panel publication-template-list">
    <header class="publication-panel-header">
      <div>
        <p class="publication-kicker">Template</p>
        <h2>模板</h2>
      </div>
      <UButton icon="i-lucide-plus" label="新建" size="sm" @click="$emit('create')"/>
    </header>

    <div v-if="loading" aria-live="polite" class="publication-skeleton-stack">
      <USkeleton v-for="index in 5" :key="index" class="h-14 w-full"/>
    </div>

    <UEmpty
        v-else-if="!templates.length"
        class="publication-empty"
        description="创建项目模板，或等待内置模板加载完成。"
        icon="i-lucide-file-output"
        title="暂无导出模板"
    >
      <template #actions>
        <UButton icon="i-lucide-plus" label="新建模板" size="sm" @click="$emit('create')"/>
      </template>
    </UEmpty>

    <div v-else aria-label="模板" class="publication-template-items" role="listbox">
      <UButton
          v-for="template in templates"
          :key="template.id"
          :aria-selected="template.id === activeTemplateId"
          :color="template.id === activeTemplateId ? 'primary' : 'neutral'"
          :variant="template.id === activeTemplateId ? 'soft' : 'ghost'"
          block
          class="publication-template-button"
          role="option"
          size="sm"
          @click="$emit('select', template.id)"
      >
        <span class="publication-template-button-content">
          <span>
            <strong>{{ template.name }}</strong>
            <small>{{ formatLabel(template.format) }}</small>
          </span>
          <UBadge :color="template.isBuiltin ? 'neutral' : 'primary'" size="sm" variant="soft">
            {{ template.isBuiltin ? '内置' : '项目' }}
          </UBadge>
        </span>
      </UButton>
    </div>
  </section>
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
