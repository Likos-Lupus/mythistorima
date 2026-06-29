<template>
  <section aria-label="实时导出预览" class="publication-preview">
    <header class="publication-preview-header">
      <div>
        <p class="publication-kicker">Preview</p>
        <h2>实时页面预览</h2>
        <p v-if="preview">
          {{ preview.documentCount }} 个文档 · {{ preview.characterCount }} 字
          <span v-if="preview.truncated"> · 预览已截断</span>
        </p>
        <p v-else>修改模板、范围或样式后会自动刷新。</p>
      </div>
      <UBadge :color="live ? 'success' : 'neutral'" size="sm" variant="soft">
        {{ live ? '实时' : '待生成' }}
      </UBadge>
    </header>

    <div v-if="loading" aria-live="polite" class="publication-preview-loading">
      <USkeleton class="h-10 w-1/2"/>
      <USkeleton class="h-4 w-full"/>
      <USkeleton class="h-4 w-5/6"/>
      <USkeleton class="h-4 w-4/6"/>
      <USkeleton class="h-80 w-full"/>
    </div>

    <UEmpty
        v-else-if="!preview"
        class="publication-preview-empty"
        description="选择模板和导出范围后，预览会使用同一份导出配置自动渲染。"
        icon="i-lucide-file-search"
        title="尚未生成预览"
    />

    <div v-else class="publication-preview-scroll">
      <div :style="{'--publication-preview-zoom': zoom / 100}" class="publication-preview-page">
        <iframe
            v-if="['html', 'docx', 'epub'].includes(preview.format)"
            :srcdoc="preview.content"
            class="publication-preview-frame"
            sandbox=""
            title="导出 HTML 预览"
        />
        <pre v-else class="publication-preview-text">{{ preview.content }}</pre>
      </div>
    </div>
  </section>
</template>

<script lang="ts" setup>
import type {ExportTemplatePreview} from '~/types/exportTemplate'

defineProps<{
  preview: ExportTemplatePreview | null
  loading?: boolean
  live?: boolean
  zoom: number
}>()
</script>
