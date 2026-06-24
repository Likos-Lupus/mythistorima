export type AppCommandId =
    | 'commandPalette.open'
    | 'document.createChapter'
    | 'card.createCharacter'
    | 'editor.toggleFocus'
    | 'proofreading.runCurrent'
    | 'export.currentDocument'
    | 'theme.cycle'
    | 'navigation.settings'

export type CommandPaletteItemKind = 'command' | 'document' | 'card' | 'note'

export interface AppCommandDefinition {
    id: AppCommandId
    title: string
    description: string
    group: string
    keywords: string[]
    defaultShortcut: string
    allowInInput?: boolean
}

export type CommandPaletteAction =
    | { type: 'command', commandId: AppCommandId }
    | { type: 'openDocument', targetId: string }
    | { type: 'openCard', targetId: string }
    | { type: 'openNote', targetId: string }

export interface CommandPaletteItem {
    id: string
    kind: CommandPaletteItemKind
    title: string
    subtitle: string
    group: string
    keywords: string[]
    shortcut?: string | null
    disabled?: boolean
    action: CommandPaletteAction
}

export interface ShortcutConflict {
    shortcut: string
    commandIds: AppCommandId[]
}

export type ShortcutMap = Record<AppCommandId, string>
