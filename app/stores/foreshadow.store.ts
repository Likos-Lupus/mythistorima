import type {
    CreateForeshadowThreadFromNoteInput,
    CreateForeshadowThreadInput,
    ForeshadowPriority,
    ForeshadowStatus,
    ForeshadowThread,
    ListForeshadowThreadsInput,
    UpdateForeshadowThreadInput
} from '~/types/foreshadow'

function sortThreads(threads: ForeshadowThread[]) {
    const statusOrder = new Map([
        ['open', 0],
        ['planned', 1],
        ['paid_off', 2],
        ['abandoned', 3],
        ['archived', 4]
    ])
    const priorityOrder = new Map([
        ['high', 0],
        ['normal', 1],
        ['low', 2]
    ])
    return [...threads].sort((a, b) => {
        const statusA = statusOrder.get(a.status) ?? 9
        const statusB = statusOrder.get(b.status) ?? 9
        if (statusA !== statusB) return statusA - statusB
        const priorityA = priorityOrder.get(a.priority) ?? 9
        const priorityB = priorityOrder.get(b.priority) ?? 9
        if (priorityA !== priorityB) return priorityA - priorityB
        return b.updatedAt - a.updatedAt
    })
}

export const useForeshadowStore = defineStore('foreshadow', () => {
    const threads = ref<ForeshadowThread[]>([])
    const activeThreadId = ref<string | null>(null)
    const statusFilter = ref<ForeshadowStatus>('all')
    const priorityFilter = ref<ForeshadowPriority>('all')
    const onlyUnpaid = ref(false)
    const loading = ref(false)
    const saving = ref(false)
    const {call} = useTauriInvoke()

    const activeThread = computed(() => threads.value.find(thread => thread.id === activeThreadId.value) ?? null)
    const unresolvedThreads = computed(() => threads.value.filter(thread => thread.status !== 'paid_off' && thread.status !== 'archived'))
    const counts = computed(() => {
        const result: Record<string, number> = {
            all: threads.value.length,
            unresolved: unresolvedThreads.value.length,
            open: 0,
            planned: 0,
            paid_off: 0,
            abandoned: 0,
            archived: 0,
            high: 0,
            normal: 0,
            low: 0
        }
        for (const thread of threads.value) {
            result[thread.status] = (result[thread.status] ?? 0) + 1
            result[thread.priority] = (result[thread.priority] ?? 0) + 1
        }
        return result
    })

    async function loadThreads(projectId: string) {
        loading.value = true
        try {
            const input: ListForeshadowThreadsInput = {
                projectId,
                status: statusFilter.value,
                priority: priorityFilter.value,
                onlyUnpaid: onlyUnpaid.value
            }
            threads.value = sortThreads(await call<ForeshadowThread[]>('list_foreshadow_threads', {input}))
            if (activeThreadId.value && !threads.value.some(thread => thread.id === activeThreadId.value)) {
                activeThreadId.value = threads.value[0]?.id ?? null
            }
            return threads.value
        } finally {
            loading.value = false
        }
    }

    function selectThread(threadId: string | null) {
        activeThreadId.value = threadId
    }

    function setFilters(status: ForeshadowStatus, priority: ForeshadowPriority, unpaid: boolean) {
        statusFilter.value = status
        priorityFilter.value = priority
        onlyUnpaid.value = unpaid
    }

    async function createThread(input: CreateForeshadowThreadInput) {
        saving.value = true
        try {
            const thread = await call<ForeshadowThread>('create_foreshadow_thread', {input})
            threads.value = sortThreads([thread, ...threads.value.filter(item => item.id !== thread.id)])
            activeThreadId.value = thread.id
            return thread
        } finally {
            saving.value = false
        }
    }

    async function createThreadFromNote(input: CreateForeshadowThreadFromNoteInput) {
        saving.value = true
        try {
            const thread = await call<ForeshadowThread>('create_foreshadow_thread_from_note', {input})
            threads.value = sortThreads([thread, ...threads.value.filter(item => item.id !== thread.id)])
            activeThreadId.value = thread.id
            return thread
        } finally {
            saving.value = false
        }
    }

    async function updateThread(input: UpdateForeshadowThreadInput) {
        saving.value = true
        try {
            const thread = await call<ForeshadowThread>('update_foreshadow_thread', {input})
            const index = threads.value.findIndex(item => item.id === thread.id)
            if (index >= 0) threads.value[index] = thread
            else threads.value.push(thread)
            threads.value = sortThreads(threads.value)
            activeThreadId.value = thread.id
            return thread
        } finally {
            saving.value = false
        }
    }

    async function markPaidOff(threadId: string, payoffDocumentId?: string | null) {
        saving.value = true
        try {
            const thread = await call<ForeshadowThread>('mark_foreshadow_paid_off', {
                threadId,
                payoffDocumentId: payoffDocumentId ?? null
            })
            const index = threads.value.findIndex(item => item.id === thread.id)
            if (index >= 0) threads.value[index] = thread
            else threads.value.push(thread)
            threads.value = sortThreads(threads.value)
            activeThreadId.value = thread.id
            return thread
        } finally {
            saving.value = false
        }
    }

    async function deleteThread(threadId: string) {
        saving.value = true
        try {
            await call<boolean>('delete_foreshadow_thread', {threadId})
            threads.value = threads.value.filter(thread => thread.id !== threadId)
            if (activeThreadId.value === threadId) activeThreadId.value = threads.value[0]?.id ?? null
        } finally {
            saving.value = false
        }
    }

    return {
        threads,
        activeThreadId,
        activeThread,
        unresolvedThreads,
        statusFilter,
        priorityFilter,
        onlyUnpaid,
        counts,
        loading,
        saving,
        loadThreads,
        selectThread,
        setFilters,
        createThread,
        createThreadFromNote,
        updateThread,
        markPaidOff,
        deleteThread
    }
})
