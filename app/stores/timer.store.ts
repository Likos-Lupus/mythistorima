export const useTimerStore = defineStore('timer', () => {
    const activeSessionId = ref<string | null>(null)
    const activeDocumentId = ref<string | null>(null)
    const sessionStartedAt = ref<number | null>(null)
    const elapsedMs = ref(0)
    const currentSessionDelta = ref(0)
    const todayCharacterCount = ref(0)
    const todayElapsedMs = ref(0)
    const isWriting = ref(false)

    let intervalId: ReturnType<typeof setInterval> | null = null

    function startSession(documentId: string, startedAt: number, sessionId: string) {
        stopTicker()
        activeDocumentId.value = documentId
        activeSessionId.value = sessionId
        sessionStartedAt.value = startedAt
        elapsedMs.value = 0
        currentSessionDelta.value = 0
        isWriting.value = true
        intervalId = setInterval(() => {
            if (sessionStartedAt.value) elapsedMs.value = Date.now() - sessionStartedAt.value
        }, 1000)
    }

    function recordActivity(sessionDelta: number) {
        currentSessionDelta.value = Math.max(0, sessionDelta)
        isWriting.value = true
    }

    function setTodayStats(payload: { characterCount: number, elapsedMs: number }) {
        todayCharacterCount.value = payload.characterCount
        todayElapsedMs.value = payload.elapsedMs
    }

    function finishSession() {
        stopTicker()
        isWriting.value = false
    }

    function stopTicker() {
        if (intervalId) {
            clearInterval(intervalId)
            intervalId = null
        }
    }

    return {
        activeSessionId,
        activeDocumentId,
        sessionStartedAt,
        elapsedMs,
        currentSessionDelta,
        todayCharacterCount,
        todayElapsedMs,
        isWriting,
        startSession,
        recordActivity,
        setTodayStats,
        finishSession
    }
})
