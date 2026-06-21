import type {EditorSettings} from '~/types/editor'

export type AppTheme = 'paper' | 'light' | 'dark'
export type AppLanguage = 'zh-CN' | 'en'

export interface AppSetting {
    key: string
    valueJson: string
    updatedAt: number
}

export interface SetAppSettingInput {
    key: string
    valueJson: string
}

export interface ThemeOption {
    value: AppTheme
    label: string
    description: string
}

export interface LanguageOption {
    value: AppLanguage
    label: string
    description: string
}

export const DEFAULT_APP_THEME: AppTheme = 'paper'
export const DEFAULT_APP_LANGUAGE: AppLanguage = 'zh-CN'

export const DEFAULT_EDITOR_SETTINGS: EditorSettings = {
    fontSize: 17,
    lineHeight: 2,
    pageWidth: 820,
    fontFamily: 'serif',
    autosaveIntervalMs: 1000
}

export const THEME_OPTIONS: ThemeOption[] = [
    {
        value: 'paper',
        label: '纸张护眼',
        description: '温暖纸张背景，适合长时间写作。'
    },
    {
        value: 'light',
        label: '明亮',
        description: '高亮度、清爽界面，适合白天。'
    },
    {
        value: 'dark',
        label: '夜间',
        description: '低亮度深色界面，适合夜间。'
    }
]

export const LANGUAGE_OPTIONS: LanguageOption[] = [
    {
        value: 'zh-CN',
        label: '简体中文',
        description: '默认中文界面。'
    },
    {
        value: 'en',
        label: 'English',
        description: 'English UI structure for future localization.'
    }
]

export const FONT_FAMILY_OPTIONS = [
    {value: 'serif', label: '宋体 / Serif'},
    {value: 'sans', label: '黑体 / Sans'},
    {value: 'system', label: '系统字体'},
    {value: 'mono', label: '等宽字体'}
] as const

export type EditorFontFamily = typeof FONT_FAMILY_OPTIONS[number]['value']
