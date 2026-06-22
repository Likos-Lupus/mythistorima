export type Phase2Status = 'foundation' | 'planned' | 'active'

export interface OutlineNode {
    id: string
    projectId: string
    parentId: string | null
    linkedDocumentId: string | null
    title: string
    type: 'plot' | 'conflict' | 'twist' | 'event' | 'arc' | 'theme' | 'note' | string
    summary: string
    status: 'planned' | 'drafting' | 'done' | 'archived' | string
    sortOrder: number
    metadataJson: string
    createdAt: number
    updatedAt: number
}

export interface CardRelation {
    id: string
    projectId: string
    sourceCardId: string
    targetCardId: string
    relationType: string
    description: string
    direction: 'directed' | 'undirected' | string
    metadataJson: string
    createdAt: number
    updatedAt: number
}

export interface TimelineEvent {
    id: string
    projectId: string
    linkedCardId: string | null
    linkedDocumentId: string | null
    title: string
    description: string
    startsAtLabel: string | null
    endsAtLabel: string | null
    sortKey: number
    locationCardId: string | null
    metadataJson: string
    createdAt: number
    updatedAt: number
}

export interface ForeshadowThread {
    id: string
    projectId: string
    title: string
    description: string
    status: 'open' | 'planned' | 'paid_off' | 'abandoned' | string
    setupNoteId: string | null
    payoffNoteId: string | null
    setupDocumentId: string | null
    payoffDocumentId: string | null
    priority: 'low' | 'normal' | 'high' | string
    createdAt: number
    updatedAt: number
}

export interface AppearanceStat {
    id: string
    projectId: string
    documentId: string
    cardId: string
    mentionCount: number
    firstSeenPosition: number | null
    updatedAt: number
}

export interface ExportTemplate {
    id: string
    projectId: string | null
    name: string
    format: 'txt' | 'markdown' | 'html' | 'docx' | 'epub' | 'pixiv' | string
    configJson: string
    isBuiltin: number
    createdAt: number
    updatedAt: number
}

export interface ProofreadingRule {
    id: string
    projectId: string | null
    name: string
    type: string
    pattern: string | null
    configJson: string
    severity: 'info' | 'warning' | 'error' | string
    enabled: number
    createdAt: number
    updatedAt: number
}
