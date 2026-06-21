export type Phase1FeatureStatus = 'planned' | 'foundation' | 'enabled'

export interface Phase1FeatureFlag {
    key: string
    label: string
    week: number
    status: Phase1FeatureStatus
    description: string
}

export interface SchemaMigration {
    version: number
    name: string
    appliedAt: number
}

export interface Card {
    id: string
    projectId: string
    type: 'character' | 'location' | 'concept' | string
    name: string
    aliasesJson: string
    description: string
    fieldsJson: string
    avatarAssetId?: string | null
    createdAt: number
    updatedAt: number
}

export interface CardReference {
    id: string
    projectId: string
    documentId: string
    cardId: string
    displayText: string
    fromPos?: number | null
    toPos?: number | null
    paragraphId?: string | null
    createdAt: number
    updatedAt: number
}

export interface Note {
    id: string
    projectId: string
    documentId?: string | null
    paragraphId?: string | null
    type: 'memo' | 'todo' | 'foreshadow' | 'issue' | 'idea' | string
    title: string
    body: string
    status: 'open' | 'doing' | 'done' | 'archived' | string
    priority: 'low' | 'normal' | 'high' | string
    dueAt?: number | null
    createdAt: number
    updatedAt: number
}

export interface SearchResult {
    targetType: string
    targetId: string
    projectId: string
    title: string
    snippet: string
}
