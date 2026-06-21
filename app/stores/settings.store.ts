import type {AppLanguage, AppSetting, AppTheme, EditorFontFamily, SetAppSettingInput} from '~/types/settings'
import {
    DEFAULT_APP_LANGUAGE,
    DEFAULT_APP_THEME,
    DEFAULT_EDITOR_SETTINGS,
    FONT_FAMILY_OPTIONS,
    LANGUAGE_OPTIONS,
    THEME_OPTIONS
} from '~/types/settings'
import type {EditorSettings} from '~/types/editor'

function parseJsonSetting<T>(valueJson: string | undefined, fallback: T): T {
    if (!valueJson) return fallback
    try {
        return JSON.parse(valueJson) as T
    } catch {
        return fallback
    }
}

function parseNumberSetting(valueJson: string | undefined, fallback: number) {
    const parsed = parseJsonSetting<unknown>(valueJson, fallback)
    const numeric = typeof parsed === 'number' ? parsed : Number(parsed)
    return Number.isFinite(numeric) ? numeric : fallback
}

function isTheme(value: unknown): value is AppTheme {
    return typeof value === 'string' && THEME_OPTIONS.some(option => option.value === value)
}

function isLanguage(value: unknown): value is AppLanguage {
    return typeof value === 'string' && LANGUAGE_OPTIONS.some(option => option.value === value)
}

function isFontFamily(value: unknown): value is EditorFontFamily {
    return typeof value === 'string' && FONT_FAMILY_OPTIONS.some(option => option.value === value)
}

export const useSettingsStore = defineStore('settings', () => {
    const {call} = useTauriInvoke()
    const loading = ref(false)
    const settings = ref<Record<string, AppSetting>>({})
    const theme = ref<AppTheme>(DEFAULT_APP_THEME)
    const language = ref<AppLanguage>(DEFAULT_APP_LANGUAGE)
    const editorSettings = reactive<EditorSettings>({...DEFAULT_EDITOR_SETTINGS})

    const themeOptions = THEME_OPTIONS
    const languageOptions = LANGUAGE_OPTIONS
    const fontFamilyOptions = FONT_FAMILY_OPTIONS

    async function loadAppSettings() {
        loading.value = true
        try {
            const rows = await call<AppSetting[]>('list_app_settings')
            settings.value = Object.fromEntries(rows.map(row => [row.key, row]))
            applySettingsFromRows()
            return rows
        } finally {
            loading.value = false
        }
    }

    async function setAppSetting(input: SetAppSettingInput) {
        const row = await call<AppSetting>('set_app_setting', {input})
        settings.value[row.key] = row
        applySettingsFromRows()
        return row
    }

    async function setTheme(nextTheme: AppTheme) {
        theme.value = nextTheme
        applyTheme(nextTheme)
        await setAppSetting({
            key: 'app.theme',
            valueJson: JSON.stringify(nextTheme)
        })
    }

    async function setLanguage(nextLanguage: AppLanguage) {
        language.value = nextLanguage
        applyLanguage(nextLanguage)
        await setAppSetting({
            key: 'app.language',
            valueJson: JSON.stringify(nextLanguage)
        })
    }

    async function updateEditorSetting<K extends keyof EditorSettings>(key: K, value: EditorSettings[K]) {
        const normalized = normalizeEditorValue(key, value)
        editorSettings[key] = normalized as EditorSettings[K]
        applyEditorSettingsToDocument()
        await setAppSetting({
            key: `editor.${key}`,
            valueJson: JSON.stringify(normalized)
        })
    }

    function applySettingsFromRows() {
        const storedTheme = parseJsonSetting<unknown>(settings.value['app.theme']?.valueJson, DEFAULT_APP_THEME)
        theme.value = isTheme(storedTheme) ? storedTheme : DEFAULT_APP_THEME

        const storedLanguage = parseJsonSetting<unknown>(settings.value['app.language']?.valueJson, DEFAULT_APP_LANGUAGE)
        language.value = isLanguage(storedLanguage) ? storedLanguage : DEFAULT_APP_LANGUAGE

        editorSettings.fontSize = normalizeEditorValue(
            'fontSize',
            parseNumberSetting(settings.value['editor.fontSize']?.valueJson, DEFAULT_EDITOR_SETTINGS.fontSize)
        ) as number
        editorSettings.lineHeight = normalizeEditorValue(
            'lineHeight',
            parseNumberSetting(settings.value['editor.lineHeight']?.valueJson, DEFAULT_EDITOR_SETTINGS.lineHeight)
        ) as number
        editorSettings.pageWidth = normalizeEditorValue(
            'pageWidth',
            parseNumberSetting(settings.value['editor.pageWidth']?.valueJson, DEFAULT_EDITOR_SETTINGS.pageWidth)
        ) as number
        editorSettings.fontFamily = normalizeEditorValue(
            'fontFamily',
            parseJsonSetting<unknown>(settings.value['editor.fontFamily']?.valueJson, DEFAULT_EDITOR_SETTINGS.fontFamily)
        ) as EditorSettings['fontFamily']
        editorSettings.autosaveIntervalMs = normalizeEditorValue(
            'autosaveIntervalMs',
            parseNumberSetting(settings.value['editor.autosaveIntervalMs']?.valueJson, DEFAULT_EDITOR_SETTINGS.autosaveIntervalMs)
        ) as number

        applyTheme(theme.value)
        applyLanguage(language.value)
        applyEditorSettingsToDocument()
    }

    function normalizeEditorValue(key: keyof EditorSettings, value: unknown): EditorSettings[keyof EditorSettings] {
        if (key === 'fontSize') {
            const numeric = Number(value)
            return Math.min(28, Math.max(12, Math.round(Number.isFinite(numeric) ? numeric : DEFAULT_EDITOR_SETTINGS.fontSize)))
        }
        if (key === 'lineHeight') {
            const numeric = Number(value)
            return Math.min(2.8, Math.max(1.3, Number((Number.isFinite(numeric) ? numeric : DEFAULT_EDITOR_SETTINGS.lineHeight).toFixed(2))))
        }
        if (key === 'pageWidth') {
            const numeric = Number(value)
            return Math.min(1100, Math.max(560, Math.round(Number.isFinite(numeric) ? numeric : DEFAULT_EDITOR_SETTINGS.pageWidth)))
        }
        if (key === 'autosaveIntervalMs') {
            const numeric = Number(value)
            return Math.min(10000, Math.max(500, Math.round(Number.isFinite(numeric) ? numeric : DEFAULT_EDITOR_SETTINGS.autosaveIntervalMs)))
        }
        if (key === 'fontFamily') {
            return isFontFamily(value) ? value : DEFAULT_EDITOR_SETTINGS.fontFamily
        }
        return value as EditorSettings[keyof EditorSettings]
    }

    function applyTheme(nextTheme = theme.value) {
        if (typeof document === 'undefined') return
        document.documentElement.dataset.theme = nextTheme
        document.body.dataset.theme = nextTheme
    }

    function applyLanguage(nextLanguage = language.value) {
        if (typeof document === 'undefined') return
        document.documentElement.lang = nextLanguage
    }

    function applyEditorSettingsToDocument() {
        if (typeof document === 'undefined') return
        document.documentElement.style.setProperty('--app-editor-font-size', `${editorSettings.fontSize}px`)
        document.documentElement.style.setProperty('--app-editor-line-height', String(editorSettings.lineHeight))
        document.documentElement.style.setProperty('--app-editor-page-width', `${editorSettings.pageWidth}px`)
        document.documentElement.dataset.editorFont = editorSettings.fontFamily
    }

    return {
        loading,
        settings,
        theme,
        language,
        editorSettings,
        themeOptions,
        languageOptions,
        fontFamilyOptions,
        loadAppSettings,
        setAppSetting,
        setTheme,
        setLanguage,
        updateEditorSetting,
        applyTheme,
        applyLanguage,
        applyEditorSettingsToDocument
    }
})
