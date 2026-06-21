export type ProjectStatus = 'active' | 'archived'

export interface Project {
    id: string
    title: string
    author?: string | null
    description?: string | null
    status: ProjectStatus | string
    language: string
    coverAssetId?: string | null
    targetCharacterCount?: number | null
    dailyTargetCount?: number | null
    metadataJson: string
    createdAt: number
    updatedAt: number
}

export interface CreateProjectInput {
    title: string
    author?: string | null
    description?: string | null
    language?: string | null
}
