import type {ExportFontFamily, ExportTemplateConfig, ExportTemplateFormat} from '~/types/exportTemplate'

export function defaultExportTemplateConfig(format: ExportTemplateFormat = 'txt'): ExportTemplateConfig {
    return {
        includeTitle: true,
        includeAuthor: format !== 'pixiv',
        includeChapterTitle: true,
        chapterTitleFormat: format === 'markdown' || format === 'pixiv'
            ? '{title}'
            : '第 {index} 章 {title}',
        paragraphSeparator: '\n\n',
        firstLineIndent: format === 'txt' || format === 'html',
        fontFamily: 'serif',
        fontSize: 12,
        lineHeight: format === 'html' ? 1.8 : 1.6,
        pixivPageBreak: format === 'pixiv'
    }
}

export function parseExportTemplateConfig(
    configJson: string | null | undefined,
    format: ExportTemplateFormat = 'txt'
): ExportTemplateConfig {
    const fallback = defaultExportTemplateConfig(format)
    if (!configJson) return fallback
    try {
        const value = JSON.parse(configJson) as Partial<ExportTemplateConfig>
        return normalizeExportTemplateConfig({...fallback, ...value})
    } catch {
        return fallback
    }
}

export function normalizeExportTemplateConfig(config: ExportTemplateConfig): ExportTemplateConfig {
    const allowedFonts: ExportFontFamily[] = ['serif', 'sans-serif', 'monospace', 'system']
    return {
        includeTitle: Boolean(config.includeTitle),
        includeAuthor: Boolean(config.includeAuthor),
        includeChapterTitle: Boolean(config.includeChapterTitle),
        chapterTitleFormat: String(config.chapterTitleFormat || '{title}').slice(0, 160),
        paragraphSeparator: String(config.paragraphSeparator || '\n\n').slice(0, 12),
        firstLineIndent: Boolean(config.firstLineIndent),
        fontFamily: allowedFonts.includes(config.fontFamily) ? config.fontFamily : 'serif',
        fontSize: clampNumber(config.fontSize, 8, 72, 12),
        lineHeight: clampNumber(config.lineHeight, 1, 3, 1.6),
        pixivPageBreak: Boolean(config.pixivPageBreak)
    }
}

export function serializeExportTemplateConfig(config: ExportTemplateConfig) {
    return JSON.stringify(normalizeExportTemplateConfig(config))
}

export function exportTemplateFormatLabel(format: ExportTemplateFormat) {
    switch (format) {
        case 'markdown':
            return 'Markdown'
        case 'html':
            return 'HTML'
        case 'pixiv':
            return 'Pixiv'
        default:
            return 'TXT'
    }
}

function clampNumber(value: unknown, min: number, max: number, fallback: number) {
    const number = Number(value)
    if (!Number.isFinite(number)) return fallback
    return Math.min(max, Math.max(min, number))
}
