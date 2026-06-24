import type {ExportRange, ExportResult} from '~/types/export'

export type ExportTemplateFormat = 'txt' | 'markdown' | 'html' | 'pixiv'
export type ExportFontFamily = 'serif' | 'sans-serif' | 'monospace' | 'system'

export interface ExportTemplateConfig {
    includeTitle: boolean
    includeAuthor: boolean
    includeChapterTitle: boolean
    chapterTitleFormat: string
    paragraphSeparator: string
    firstLineIndent: boolean
    fontFamily: ExportFontFamily
    fontSize: number
    lineHeight: number
    pixivPageBreak: boolean
}

export interface ExportTemplate {
    id: string
    projectId?: string | null
    name: string
    format: ExportTemplateFormat
    configJson: string
    isBuiltin: number
    createdAt: number
    updatedAt: number
}

export interface ExportTemplateDraft {
    templateId?: string | null
    name: string
    format: ExportTemplateFormat
    config: ExportTemplateConfig
    isBuiltin: boolean
}

export interface CreateExportTemplateInput {
    projectId: string
    name: string
    format: ExportTemplateFormat
    configJson?: string | null
}

export interface UpdateExportTemplateInput {
    projectId: string
    templateId: string
    name?: string | null
    format?: ExportTemplateFormat | null
    configJson?: string | null
}

export interface ListExportTemplatesInput {
    projectId: string
    includeBuiltin?: boolean | null
    format?: ExportTemplateFormat | null
}

export interface ExportWithTemplateInput {
    projectId: string
    templateId: string
    range?: ExportRange | null
    documentId?: string | null
    documentIds?: string[] | null
    outputPath?: string | null
    configOverrideJson?: string | null
}

export interface ExportTemplatePreview {
    format: ExportTemplateFormat
    content: string
    documentCount: number
    characterCount: number
    truncated: boolean
}

export type ExportWithTemplateResult = ExportResult
