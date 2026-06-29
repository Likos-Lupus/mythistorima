<template>
  <section aria-label="模板设置" class="publication-panel">
    <header class="publication-panel-header">
      <div>
        <p class="publication-kicker">Format</p>
        <h2>{{ draft.isBuiltin ? '内置模板' : creating ? '创建模板' : '模板设置' }}</h2>
        <p>{{ draft.isBuiltin ? '内置模板只读，可复制后调整。' : '设置模板名称和目标发布格式。' }}</p>
      </div>
      <UBadge color="neutral" size="sm" variant="soft">{{ formatLabel }}</UBadge>
    </header>

    <UForm :state="draft" class="publication-form" @submit="save">
      <UFormField label="模板名称" name="name" required>
        <UInput
            v-model="draft.name"
            :disabled="draft.isBuiltin"
            class="w-full"
            maxlength="120"
            size="sm"
        />
      </UFormField>

      <UFormField label="格式" name="format">
        <USelect
            v-model="draft.format"
            :disabled="draft.isBuiltin"
            :items="formatItems"
            class="w-full"
            label-key="label"
            size="sm"
            value-key="value"
        />
      </UFormField>

      <footer class="publication-actions">
        <UButton
            v-if="draft.isBuiltin"
            :loading="saving"
            icon="i-lucide-copy-plus"
            label="复制为项目模板"
            size="sm"
            @click="$emit('duplicate', cloneDraft(draft))"
        />
        <template v-else>
          <UButton
              :disabled="!draft.name.trim()"
              :label="creating ? '创建模板' : '保存模板'"
              :loading="saving"
              icon="i-lucide-save"
              size="sm"
              type="submit"
          />
          <UButton
              v-if="!creating"
              color="error"
              icon="i-lucide-trash-2"
              label="删除"
              size="sm"
              variant="ghost"
              @click="confirmDeleteOpen = true"
          />
        </template>
      </footer>
    </UForm>

    <UModal
        v-model:open="confirmDeleteOpen"
        description="删除后无法从模板列表恢复。内置模板不会被删除。"
        title="删除导出模板？"
    >
      <template #footer>
        <div class="publication-modal-actions">
          <UButton color="neutral" label="取消" size="sm" variant="ghost" @click="confirmDeleteOpen = false"/>
          <UButton color="error" icon="i-lucide-trash-2" label="删除" size="sm" @click="remove"/>
        </div>
      </template>
    </UModal>
  </section>
</template>

<script lang="ts" setup>
import type {ExportTemplateDraft, ExportTemplateFormat} from '~/types/exportTemplate'
import {defaultExportTemplateConfig, exportTemplateFormatLabel} from '~/utils/exportTemplate'

const props = defineProps<{
  creating?: boolean
  saving?: boolean
}>()

const draft = defineModel<ExportTemplateDraft>({required: true})

const emit = defineEmits<{
  save: [draft: ExportTemplateDraft]
  delete: [templateId: string]
  duplicate: [draft: ExportTemplateDraft]
}>()

const confirmDeleteOpen = ref(false)
const formatItems: { label: string, value: ExportTemplateFormat }[] = [
  {label: 'TXT', value: 'txt'},
  {label: 'Markdown', value: 'markdown'},
  {label: 'HTML', value: 'html'},
  {label: 'DOCX', value: 'docx'},
  {label: 'EPUB', value: 'epub'},
  {label: 'Pixiv', value: 'pixiv'}
]
const formatLabel = computed(() => exportTemplateFormatLabel(draft.value.format))

watch(
    () => draft.value.format,
    (format, previous) => {
      if (!previous || draft.value.isBuiltin) return
      const defaults = defaultExportTemplateConfig(format)
      draft.value.config = {
        ...draft.value.config,
        documentPageBreak: defaults.documentPageBreak,
        epubIncludeToc: defaults.epubIncludeToc,
        epubIncludeAssets: defaults.epubIncludeAssets,
        pixivPageBreak: defaults.pixivPageBreak,
        lineHeight: defaults.lineHeight,
        chapterTitleFormat: defaults.chapterTitleFormat
      }
    }
)

function save() {
  emit('save', cloneDraft(draft.value))
}

function remove() {
  if (!draft.value.templateId) return
  confirmDeleteOpen.value = false
  emit('delete', draft.value.templateId)
}

function cloneDraft(value: ExportTemplateDraft): ExportTemplateDraft {
  return {
    templateId: value.templateId ?? null,
    name: value.name,
    format: value.format,
    config: {...value.config, pixivTags: [...value.config.pixivTags]},
    isBuiltin: value.isBuiltin
  }
}
</script>
