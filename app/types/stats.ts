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
