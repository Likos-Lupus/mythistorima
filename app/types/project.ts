export type ProjectStatus = 'active' | 'archived'

export interface Project {
    id: string
    title: string
    author?: string | null
    description?: string | null
    status: ProjectStatus | string
    createdAt: number
    updatedAt: number
}

export interface CreateProjectInput {
    title: string
    author?: string | null
    description?: string | null
}
