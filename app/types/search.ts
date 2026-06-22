export type SearchScope =
    'all'
    | 'documents'
    | 'cards'
    | 'notes'
    | 'outline'
    | 'timeline'
    | 'foreshadow'
    | 'proofreading'
    | 'exportTemplates'

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
