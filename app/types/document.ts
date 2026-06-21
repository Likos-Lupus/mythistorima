export type DocumentType = 'chapter' | 'volume' | 'scene' | 'note' | string
export type DocumentStatus = 'draft' | 'writing' | 'revised' | 'done' | 'archived' | string

export interface NovelDocument {
    id: string
    projectId: string
    parentId?: string | null
    type: DocumentType
    title: string
    sortOrder: number
    status: DocumentStatus
    summary?: string | null
    metadataJson: string
    characterCount: number
    createdAt: number
    updatedAt: number
}

export interface DocumentContent {
    documentId: string
    schemaVersion: number
    contentJson: unknown
    contentText: string
    contentHtml: string
    characterCount: number
    updatedAt: number
}

export interface CreateDocumentInput {
    projectId: string
    parentId?: string | null
    type?: DocumentType
    title: string
    sortOrder?: number | null
}

export interface UpdateDocumentContentInput {
    documentId: string
    contentJson: unknown
    contentText: string
    contentHtml: string
    characterCount: number
}
