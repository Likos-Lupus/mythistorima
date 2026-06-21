export interface EditorSettings {
    fontSize: number
    lineHeight: number
    pageWidth: number
    fontFamily: 'serif' | 'sans' | 'system' | 'mono'
    autosaveIntervalMs: number
}

export interface EditorSessionSnapshot {
    sessionId: string
    startedAt: number
    elapsedMs: number
    charactersBefore: number
    charactersAfter: number
    sessionDelta: number
}

export interface EditorFindMatch {
    from: number
    to: number
    index: number
}
