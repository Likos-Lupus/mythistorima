import type {CreateNoteInput, CreativeNote, ListNotesInput, NoteStatus, NoteType, UpdateNoteInput} from '~/types/note'

function sortNotes(notes: CreativeNote[]) {
    const statusOrder = new Map([
        ['open', 0],
        ['doing', 1],
        ['done', 2],
        ['archived', 3]
    ])
    const priorityOrder = new Map([
        ['high', 0],
        ['normal', 1],
        ['low', 2]
    ])
    return [...notes].sort((a, b) => {
        const statusA = statusOrder.get(a.status) ?? 9
        const statusB = statusOrder.get(b.status) ?? 9
        if (statusA !== statusB) return statusA - statusB
        const priorityA = priorityOrder.get(a.priority) ?? 9
        const priorityB = priorityOrder.get(b.priority) ?? 9
        if (priorityA !== priorityB) return priorityA - priorityB
        return b.updatedAt - a.updatedAt
    })
}

export const useNoteStore = defineStore('note', () => {
    const notes = ref<CreativeNote[]>([])
    const activeNoteId = ref<string | null>(null)
    const typeFilter = ref<NoteType>('all')
    const statusFilter = ref<NoteStatus>('all')
    const loading = ref(false)
    const saving = ref(false)
    const {call} = useTauriInvoke()

    const activeNote = computed(() => notes.value.find(note => note.id === activeNoteId.value) ?? null)
    const filteredNotes = computed(() => notes.value.filter(note => {
        if (typeFilter.value !== 'all' && note.type !== typeFilter.value) return false
        if (statusFilter.value !== 'all' && note.status !== statusFilter.value) return false
        return true
    }))
    const openNotes = computed(() => notes.value.filter(note => note.status === 'open' || note.status === 'doing'))
    const counts = computed(() => {
        const result: Record<string, number> = {
            all: notes.value.length,
            memo: 0,
            todo: 0,
            foreshadow: 0,
            open: 0,
            done: 0
        }
        for (const note of notes.value) {
            result[note.type] = (result[note.type] ?? 0) + 1
            result[note.status] = (result[note.status] ?? 0) + 1
        }
        return result
    })

    async function loadNotes(input: ListNotesInput) {
        loading.value = true
        try {
            notes.value = sortNotes(await call<CreativeNote[]>('list_notes', {input}))
            if (!activeNoteId.value || !notes.value.some(note => note.id === activeNoteId.value)) {
                activeNoteId.value = notes.value[0]?.id ?? null
            }
            return notes.value
        } finally {
            loading.value = false
        }
    }

    async function loadProjectNotes(projectId: string) {
        return loadNotes({
            projectId,
            type: typeFilter.value,
            status: statusFilter.value
        })
    }

    async function loadDocumentNotes(projectId: string, documentId: string, status: NoteStatus | string | null = null) {
        return call<CreativeNote[]>('list_notes', {
            input: {
                projectId,
                documentId,
                status
            }
        })
    }

    function selectNote(noteId: string | null) {
        activeNoteId.value = noteId
    }

    function setTypeFilter(type: NoteType) {
        typeFilter.value = type
    }

    function setStatusFilter(status: NoteStatus) {
        statusFilter.value = status
    }

    async function createNote(input: CreateNoteInput) {
        saving.value = true
        try {
            const note = await call<CreativeNote>('create_note', {input})
            notes.value = sortNotes([note, ...notes.value.filter(item => item.id !== note.id)])
            activeNoteId.value = note.id
            return note
        } finally {
            saving.value = false
        }
    }

    async function updateNote(input: UpdateNoteInput) {
        saving.value = true
        try {
            const note = await call<CreativeNote>('update_note', {input})
            const index = notes.value.findIndex(item => item.id === note.id)
            if (index >= 0) notes.value[index] = note
            else notes.value.push(note)
            notes.value = sortNotes(notes.value)
            activeNoteId.value = note.id
            return note
        } finally {
            saving.value = false
        }
    }

    async function updateNoteStatus(noteId: string, status: Exclude<NoteStatus, 'all'> | string) {
        const note = await call<CreativeNote>('update_note_status', {noteId, status})
        const index = notes.value.findIndex(item => item.id === note.id)
        if (index >= 0) {
            notes.value[index] = note
            notes.value = sortNotes(notes.value)
        }
        return note
    }

    async function deleteNote(noteId: string) {
        await call<boolean>('delete_note', {noteId})
        notes.value = notes.value.filter(note => note.id !== noteId)
        if (activeNoteId.value === noteId) {
            activeNoteId.value = notes.value[0]?.id ?? null
        }
    }

    return {
        notes,
        activeNoteId,
        activeNote,
        filteredNotes,
        openNotes,
        typeFilter,
        statusFilter,
        counts,
        loading,
        saving,
        loadNotes,
        loadProjectNotes,
        loadDocumentNotes,
        selectNote,
        setTypeFilter,
        setStatusFilter,
        createNote,
        updateNote,
        updateNoteStatus,
        deleteNote
    }
})
