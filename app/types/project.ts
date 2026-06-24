export interface ProjectMetadata {
    genre: string
    premise: string
    audience: string
    pointOfView: string
    tense: string
    tags: string[]
    notes: string
}

export const emptyProjectMetadata = (): ProjectMetadata => ({
    genre: '',
    premise: '',
    audience: '',
    pointOfView: '',
    tense: '',
    tags: [],
    notes: ''
})

export function parseProjectMetadata(value?: string | null): ProjectMetadata {
    if (!value) return emptyProjectMetadata()
    try {
        const parsed = JSON.parse(value) as Partial<ProjectMetadata>
        return {
            genre: typeof parsed.genre === 'string' ? parsed.genre : '',
            premise: typeof parsed.premise === 'string' ? parsed.premise : '',
            audience: typeof parsed.audience === 'string' ? parsed.audience : '',
            pointOfView: typeof parsed.pointOfView === 'string' ? parsed.pointOfView : '',
            tense: typeof parsed.tense === 'string' ? parsed.tense : '',
            tags: Array.isArray(parsed.tags)
                ? parsed.tags.filter((item): item is string => typeof item === 'string')
                : [],
            notes: typeof parsed.notes === 'string' ? parsed.notes : ''
        }
    } catch {
        return emptyProjectMetadata()
    }
}

export function serializeProjectMetadata(value: ProjectMetadata) {
    return JSON.stringify({
        genre: value.genre.trim(),
        premise: value.premise.trim(),
        audience: value.audience.trim(),
        pointOfView: value.pointOfView.trim(),
        tense: value.tense.trim(),
        tags: value.tags.map(item => item.trim()).filter(Boolean),
        notes: value.notes.trim()
    })
}

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

export interface UpdateProjectInput {
    projectId: string
    title: string
    author?: string | null
    description?: string | null
    language?: string | null
    status?: ProjectStatus | string | null
    targetCharacterCount?: number | null
    dailyTargetCount?: number | null
    metadataJson?: string | null
}
