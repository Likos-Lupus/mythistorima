export interface CardAppearance {
    id: string
    projectId: string
    documentId: string
    documentTitle: string
    documentType: string
    cardId: string
    cardName: string
    cardType: string
    mentionCount: number
    firstSeenPosition?: number | null
    updatedAt: number
}

export interface CardAppearanceSummary {
    cardId: string
    cardName: string
    cardType: string
    documentCount: number
    totalMentions: number
    firstDocumentId?: string | null
    firstDocumentTitle?: string | null
}

export interface DocumentAppearanceDocument {
    documentId: string
    documentTitle: string
    documentType: string
    sortOrder: number
    parentId?: string | null
}

export interface DocumentAppearanceCell {
    documentId: string
    cardId: string
    mentionCount: number
    firstSeenPosition?: number | null
}

export interface ProjectAppearanceSummary {
    projectId: string
    totalCards: number
    appearedCards: number
    totalDocuments: number
    documentsWithAppearances: number
    totalMentions: number
    cards: CardAppearanceSummary[]
    documents: DocumentAppearanceDocument[]
    appearances: DocumentAppearanceCell[]
}
