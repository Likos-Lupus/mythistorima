import type {AppSetting, SetAppSettingInput} from '~/types/settings'
import {DEFAULT_EDITOR_SETTINGS} from '~/types/settings'
import type {EditorSettings} from '~/types/editor'

function parseNumberSetting(valueJson: string | undefined, fallback: number) {
    if (!valueJson) return fallback
    try {
        const parsed = JSON.parse(valueJson)
        const numeric = typeof parsed === 'number' ? parsed : Number(parsed)
        return Number.isFinite(numeric) ? numeric : fallback
    } catch {
        const numeric = Number(valueJson)
        return Number.isFinite(numeric) ? numeric : fallback
    }
}

export const useSettingsStore = defineStore('settings', () => {
    const {call} = useTauriInvoke()
    const loading = ref(false)
    const settings = ref<Record<string, AppSetting>>({})
    const editorSettings = reactive<EditorSettings>({...DEFAULT_EDITOR_SETTINGS})

    async function loadAppSettings() {
        loading.value = true
        try {
            const rows = await call<AppSetting[]>('list_app_settings')
            settings.value = Object.fromEntries(rows.map(row => [row.key, row]))
            applyEditorSettingsFromRows()
            return rows
        } finally {
            loading.value = false
        }
    }

    async function setAppSetting(input: SetAppSettingInput) {
        const row = await call<AppSetting>('set_app_setting', {input})
        settings.value[row.key] = row
        applyEditorSettingsFromRows()
        return row
    }

    async function updateEditorSetting<K extends keyof EditorSettings>(key: K, value: EditorSettings[K]) {
        const normalized = normalizeEditorValue(key, Number(value))
        editorSettings[key] = normalized as EditorSettings[K]
        await setAppSetting({
            key: `editor.${key}`,
            valueJson: JSON.stringify(normalized)
        })
    }

    function applyEditorSettingsFromRows() {
        editorSettings.fontSize = normalizeEditorValue(
            'fontSize',
            parseNumberSetting(settings.value['editor.fontSize']?.valueJson, DEFAULT_EDITOR_SETTINGS.fontSize)
        )
        editorSettings.lineHeight = normalizeEditorValue(
            'lineHeight',
            parseNumberSetting(settings.value['editor.lineHeight']?.valueJson, DEFAULT_EDITOR_SETTINGS.lineHeight)
        )
        editorSettings.pageWidth = normalizeEditorValue(
            'pageWidth',
            parseNumberSetting(settings.value['editor.pageWidth']?.valueJson, DEFAULT_EDITOR_SETTINGS.pageWidth)
        )
    }

    function normalizeEditorValue(key: keyof EditorSettings, value: number) {
        if (key === 'fontSize') return Math.min(28, Math.max(12, Math.round(value)))
        if (key === 'lineHeight') return Math.min(2.8, Math.max(1.3, Number(value.toFixed(2))))
        if (key === 'pageWidth') return Math.min(1100, Math.max(560, Math.round(value)))
        return value
    }

    return {
        loading,
        settings,
        editorSettings,
        loadAppSettings,
        setAppSetting,
        updateEditorSetting
    }
})
