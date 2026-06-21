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
