export type ExportFormat = 'txt' | 'markdown' | 'html' | 'project_package'
export type ExportRange = 'all' | 'current' | 'selected'

export interface ExportDocumentsInput {
    projectId: string
    format: ExportFormat
    range?: ExportRange | null
    documentId?: string | null
    documentIds?: string[] | null
    outputPath?: string | null
}

export interface ExportProjectInput {
    projectId: string
    format?: ExportFormat | null
    outputPath?: string | null
}

export interface ExportResult {
    path: string
    format: ExportFormat
    documentCount: number
    characterCount: number
    createdAt: number
}

export interface ImportTextFileInput {
    projectId: string
    path: string
    format?: 'txt' | 'markdown' | 'html' | null
    parentId?: string | null
    title?: string | null
}

export interface ImportResult {
    documentId: string
    title: string
    characterCount: number
    importedAt: number
}

export interface BackupItem {
    id: string
    projectId: string
    path: string
    sizeBytes: number
    createdAt: number
}
