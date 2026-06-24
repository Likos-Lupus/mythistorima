export interface ProjectStats {
    projectId: string
    documentCount: number
    characterCount: number
    updatedAt?: number | null
}

export interface DocumentStats {
    documentId: string
    characterCount: number
    updatedAt: number
}

export interface AppInfo {
    name: string
    version: string
    databasePath: string
}

export interface SchemaMigration {
    version: number
    name: string
    appliedAt: number
}

export interface TodayWritingStats {
    projectId: string
    characterCount: number
    elapsedMs: number
}

export interface WritingSession {
    id: string
    projectId: string
    documentId: string
    startedAt: number
    endedAt?: number | null
    charactersBefore: number
    charactersAfter: number
    insertedCount: number
    deletedCount: number
}


export interface OverviewDocument {
    id: string
    title: string
    documentType: string
    status: string
    characterCount: number
    updatedAt: number
}

export interface OverviewNote {
    id: string
    title: string
    noteType: string
    status: string
    priority: string
    documentId?: string | null
    documentTitle?: string | null
    updatedAt: number
}

export interface OverviewForeshadow {
    id: string
    title: string
    status: string
    priority: string
    setupDocumentId?: string | null
    setupDocumentTitle?: string | null
    updatedAt: number
}

export interface OverviewCharacter {
    cardId: string
    cardName: string
    documentCount: number
    mentionCount: number
}

export interface WritingTrendPoint {
    dayStart: number
    characterCount: number
    elapsedMs: number
}

export interface ProjectOverview {
    projectId: string
    documentCount: number
    completedDocumentCount: number
    characterCount: number
    todayCharacterCount: number
    todayElapsedMs: number
    writingStreakDays: number
    recentDocuments: OverviewDocument[]
    openNotes: OverviewNote[]
    unresolvedForeshadows: OverviewForeshadow[]
    topCharacters: OverviewCharacter[]
    writingTrend: WritingTrendPoint[]
}

export interface ProjectOverviewInput {
    projectId: string
    dayStart: number
    dayEnd: number
    trendStart: number
}
