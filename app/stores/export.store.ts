import type {
    BackupItem,
    ExportDocumentsInput,
    ExportProjectInput,
    ExportResult,
    ImportResult,
    ImportTextFileInput
} from '~/types/export'

export const useExportStore = defineStore('export', () => {
    const lastExport = ref<ExportResult | null>(null)
    const lastImport = ref<ImportResult | null>(null)
    const backups = ref<BackupItem[]>([])
    const busy = ref(false)
    const error = ref<string | null>(null)
    const {call} = useTauriInvoke()

    async function exportDocuments(input: ExportDocumentsInput) {
        busy.value = true
        error.value = null
        try {
            lastExport.value = await call<ExportResult>('export_documents', {input})
            return lastExport.value
        } catch (err) {
            error.value = errorMessage(err, '导出失败')
            throw err
        } finally {
            busy.value = false
        }
    }

    async function exportProject(input: ExportProjectInput) {
        busy.value = true
        error.value = null
        try {
            lastExport.value = await call<ExportResult>('export_project', {input})
            return lastExport.value
        } catch (err) {
            error.value = errorMessage(err, '导出项目包失败')
            throw err
        } finally {
            busy.value = false
        }
    }

    async function importTextFile(input: ImportTextFileInput) {
        busy.value = true
        error.value = null
        try {
            lastImport.value = await call<ImportResult>('import_text_file', {input})
            return lastImport.value
        } catch (err) {
            error.value = errorMessage(err, '导入失败')
            throw err
        } finally {
            busy.value = false
        }
    }

    async function createBackup(projectId: string) {
        busy.value = true
        error.value = null
        try {
            const backup = await call<BackupItem>('create_backup', {projectId})
            backups.value = [backup, ...backups.value.filter(item => item.id !== backup.id)]
            return backup
        } catch (err) {
            error.value = errorMessage(err, '创建备份失败')
            throw err
        } finally {
            busy.value = false
        }
    }

    async function listBackups(projectId: string) {
        backups.value = await call<BackupItem[]>('list_backups', {projectId})
        return backups.value
    }

    return {
        lastExport,
        lastImport,
        backups,
        busy,
        error,
        exportDocuments,
        exportProject,
        importTextFile,
        createBackup,
        listBackups
    }
})

function errorMessage(error: unknown, fallback: string) {
    return typeof error === 'object' && error && 'message' in error
        ? String((error as { message?: string }).message)
        : fallback
}
