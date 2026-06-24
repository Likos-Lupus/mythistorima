import {toAppErrorMessage} from '~/utils/appError'
import type {
    CreateExportTemplateInput,
    ExportTemplate,
    ExportTemplatePreview,
    ExportWithTemplateInput,
    ExportWithTemplateResult,
    ListExportTemplatesInput,
    UpdateExportTemplateInput
} from '~/types/exportTemplate'

function sortTemplates(items: ExportTemplate[]) {
    return [...items].sort((a, b) => {
        if (a.isBuiltin !== b.isBuiltin) return b.isBuiltin - a.isBuiltin
        if (a.format !== b.format) return a.format.localeCompare(b.format)
        return a.name.localeCompare(b.name, 'zh-CN')
    })
}

export const useExportTemplateStore = defineStore('exportTemplate', () => {
    const templates = ref<ExportTemplate[]>([])
    const activeTemplateId = ref<string | null>(null)
    const preview = ref<ExportTemplatePreview | null>(null)
    const loading = ref(false)
    const saving = ref(false)
    const exporting = ref(false)
    const previewing = ref(false)
    const error = ref<string | null>(null)
    const {call} = useTauriInvoke()

    const activeTemplate = computed(
        () => templates.value.find(template => template.id === activeTemplateId.value) ?? null
    )
    const projectTemplates = computed(() => templates.value.filter(template => template.projectId))
    const builtinTemplates = computed(() => templates.value.filter(template => !template.projectId))

    async function loadTemplates(input: ListExportTemplatesInput) {
        loading.value = true
        error.value = null
        try {
            templates.value = sortTemplates(
                await call<ExportTemplate[]>('list_export_templates', {input})
            )
            if (!activeTemplateId.value || !templates.value.some(item => item.id === activeTemplateId.value)) {
                activeTemplateId.value = projectTemplates.value[0]?.id ?? builtinTemplates.value[0]?.id ?? null
            }
            return templates.value
        } catch (err) {
            error.value = toAppErrorMessage(err, '加载导出模板失败')
            throw err
        } finally {
            loading.value = false
        }
    }

    async function createTemplate(input: CreateExportTemplateInput) {
        saving.value = true
        error.value = null
        try {
            const template = await call<ExportTemplate>('create_export_template', {input})
            templates.value = sortTemplates([template, ...templates.value.filter(item => item.id !== template.id)])
            activeTemplateId.value = template.id
            return template
        } catch (err) {
            error.value = toAppErrorMessage(err, '创建导出模板失败')
            throw err
        } finally {
            saving.value = false
        }
    }

    async function updateTemplate(input: UpdateExportTemplateInput) {
        saving.value = true
        error.value = null
        try {
            const template = await call<ExportTemplate>('update_export_template', {input})
            templates.value = sortTemplates(
                templates.value.map(item => item.id === template.id ? template : item)
            )
            activeTemplateId.value = template.id
            return template
        } catch (err) {
            error.value = toAppErrorMessage(err, '保存导出模板失败')
            throw err
        } finally {
            saving.value = false
        }
    }

    async function deleteTemplate(projectId: string, templateId: string) {
        saving.value = true
        error.value = null
        try {
            await call<boolean>('delete_export_template', {projectId, templateId})
            templates.value = templates.value.filter(template => template.id !== templateId)
            if (activeTemplateId.value === templateId) {
                activeTemplateId.value = projectTemplates.value[0]?.id ?? builtinTemplates.value[0]?.id ?? null
            }
        } catch (err) {
            error.value = toAppErrorMessage(err, '删除导出模板失败')
            throw err
        } finally {
            saving.value = false
        }
    }

    async function exportWithTemplate(input: ExportWithTemplateInput) {
        exporting.value = true
        error.value = null
        try {
            return await call<ExportWithTemplateResult>('export_with_template', {input})
        } catch (err) {
            error.value = toAppErrorMessage(err, '模板导出失败')
            throw err
        } finally {
            exporting.value = false
        }
    }

    async function previewWithTemplate(input: ExportWithTemplateInput) {
        previewing.value = true
        error.value = null
        try {
            preview.value = await call<ExportTemplatePreview>('preview_export_with_template', {input})
            return preview.value
        } catch (err) {
            error.value = toAppErrorMessage(err, '生成导出预览失败')
            throw err
        } finally {
            previewing.value = false
        }
    }

    function selectTemplate(templateId: string | null) {
        activeTemplateId.value = templateId
        preview.value = null
    }

    function clearPreview() {
        preview.value = null
    }

    return {
        templates,
        activeTemplateId,
        activeTemplate,
        projectTemplates,
        builtinTemplates,
        preview,
        loading,
        saving,
        exporting,
        previewing,
        error,
        loadTemplates,
        createTemplate,
        updateTemplate,
        deleteTemplate,
        exportWithTemplate,
        previewWithTemplate,
        selectTemplate,
        clearPreview
    }
})
