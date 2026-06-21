import type {EditorSettings} from '~/types/editor'

export interface AppSetting {
    key: string
    valueJson: string
    updatedAt: number
}

export interface SetAppSettingInput {
    key: string
    valueJson: string
}

export const DEFAULT_EDITOR_SETTINGS: EditorSettings = {
    fontSize: 17,
    lineHeight: 2,
    pageWidth: 820
}
