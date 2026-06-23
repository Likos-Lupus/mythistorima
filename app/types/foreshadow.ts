export type ForeshadowStatus = 'all' | 'open' | 'planned' | 'paid_off' | 'abandoned' | 'archived'
export type ForeshadowPriority = 'all' | 'low' | 'normal' | 'high'
export type ConcreteForeshadowStatus = Exclude<ForeshadowStatus, 'all'>
export type ConcreteForeshadowPriority = Exclude<ForeshadowPriority, 'all'>

export interface ForeshadowThread {
    id: string
    projectId: string
    title: string
    description: string
    status: ConcreteForeshadowStatus | string
    setupNoteId?: string | null
    setupNoteTitle?: string | null
    payoffNoteId?: string | null
    payoffNoteTitle?: string | null
    setupDocumentId?: string | null
    setupDocumentTitle?: string | null
    payoffDocumentId?: string | null
    payoffDocumentTitle?: string | null
    priority: ConcreteForeshadowPriority | string
    createdAt: number
    updatedAt: number
}

export interface CreateForeshadowThreadInput {
    projectId: string
    title: string
    description?: string | null
    status?: ConcreteForeshadowStatus | string | null
    setupNoteId?: string | null
    payoffNoteId?: string | null
    setupDocumentId?: string | null
    payoffDocumentId?: string | null
    priority?: ConcreteForeshadowPriority | string | null
}

export interface UpdateForeshadowThreadInput {
    threadId: string
    title?: string
    description?: string
    status?: ConcreteForeshadowStatus | string
    setupNoteId?: string | null
    payoffNoteId?: string | null
    setupDocumentId?: string | null
    payoffDocumentId?: string | null
    priority?: ConcreteForeshadowPriority | string
}

export interface CreateForeshadowThreadFromNoteInput {
    noteId: string
    title?: string | null
    description?: string | null
    payoffDocumentId?: string | null
    priority?: ConcreteForeshadowPriority | string | null
}

export interface ListForeshadowThreadsInput {
    projectId: string
    status?: ForeshadowStatus | string | null
    priority?: ForeshadowPriority | string | null
    onlyUnpaid?: boolean | null
}

export const foreshadowStatusOptions: Array<{ value: ForeshadowStatus, label: string }> = [
    {value: 'all', label: '全部'},
    {value: 'open', label: '未回收'},
    {value: 'planned', label: '计划回收'},
    {value: 'paid_off', label: '已回收'},
    {value: 'abandoned', label: '放弃'},
    {value: 'archived', label: '归档'}
]

export const foreshadowPriorityOptions: Array<{ value: ForeshadowPriority, label: string }> = [
    {value: 'all', label: '全部'},
    {value: 'high', label: '高'},
    {value: 'normal', label: '普通'},
    {value: 'low', label: '低'}
]

export function foreshadowStatusLabel(status: string) {
    switch (status) {
        case 'open':
            return '未回收'
        case 'planned':
            return '计划回收'
        case 'paid_off':
            return '已回收'
        case 'abandoned':
            return '放弃'
        case 'archived':
            return '归档'
        default:
            return status
    }
}

export function foreshadowPriorityLabel(priority: string) {
    switch (priority) {
        case 'high':
            return '高'
        case 'low':
            return '低'
        default:
            return '普通'
    }
}
