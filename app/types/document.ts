export type DocumentType = 'volume' | 'chapter' | 'scene' | 'note' | string
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

export interface DocumentTreeNode extends NovelDocument {
    children: DocumentTreeNode[]
    depth: number
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

export interface MoveDocumentInput {
    documentId: string
    parentId?: string | null
    sortOrder: number
}

export interface UpdateDocumentContentInput {
    documentId: string
    contentJson: unknown
    contentText: string
    contentHtml: string
    characterCount: number
}

export interface UpdateDocumentStatusInput {
    documentId: string
    status: DocumentStatus
}

export interface DocumentCreatePayload {
    parentId?: string | null
    type: DocumentType
    title: string
    sortOrder?: number | null
}
