export type NoteType = 'all' | 'memo' | 'todo' | 'foreshadow' | 'issue' | 'idea'
export type NoteStatus = 'all' | 'open' | 'doing' | 'done' | 'archived'
export type NotePriority = 'low' | 'normal' | 'high'

export interface CreativeNote {
    id: string
    projectId: string
    documentId?: string | null
    documentTitle?: string | null
    paragraphId?: string | null
    type: Exclude<NoteType, 'all'> | string
    title: string
    body: string
    status: Exclude<NoteStatus, 'all'> | string
    priority: NotePriority | string
    dueAt?: number | null
    createdAt: number
    updatedAt: number
}

export interface CreateNoteInput {
    projectId: string
    documentId?: string | null
    paragraphId?: string | null
    type: Exclude<NoteType, 'all'> | string
    title: string
    body?: string | null
    priority?: NotePriority | string | null
    dueAt?: number | null
}

export interface UpdateNoteInput {
    noteId: string
    documentId?: string | null
    paragraphId?: string | null
    type?: Exclude<NoteType, 'all'> | string
    title?: string
    body?: string
    status?: Exclude<NoteStatus, 'all'> | string
    priority?: NotePriority | string
    dueAt?: number | null
}

export interface ListNotesInput {
    projectId: string
    documentId?: string | null
    paragraphId?: string | null
    type?: NoteType | string | null
    status?: NoteStatus | string | null
}

export interface EditorParagraphNoteRequest {
    type: Exclude<NoteType, 'all'>
    paragraphId: string | null
    selectedText: string
}
