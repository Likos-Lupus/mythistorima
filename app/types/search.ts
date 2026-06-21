export type SearchScope = 'all' | 'documents' | 'cards' | 'notes'

export interface SearchResult {
    targetType: string
    targetId: string
    projectId: string
    title: string
    snippet: string
}

export interface SearchProjectInput {
    projectId: string
    query: string
    scopes: SearchScope[]
    limit?: number | null
}
