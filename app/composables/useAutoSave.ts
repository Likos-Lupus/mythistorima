import type {ComputedRef, Ref} from 'vue'
import {ref, unref} from 'vue'

export type SaveState = 'idle' | 'dirty' | 'saving' | 'saved' | 'failed'

type MaybeDynamicDelay = number | Ref<number> | ComputedRef<number> | (() => number)

function readDelay(delay: MaybeDynamicDelay) {
    const raw = typeof delay === 'function' ? delay() : unref(delay)
    const numeric = Number(raw)
    return Number.isFinite(numeric) ? Math.min(10000, Math.max(250, Math.round(numeric))) : 1000
}

export function useAutoSave<TPayload>(handler: (payload: TPayload) => Promise<unknown>, delay: MaybeDynamicDelay = 1000) {
    const saveState = ref<SaveState>('idle')
    const lastSavedAt = ref<number | null>(null)
    const errorMessage = ref<string | null>(null)

    let timer: ReturnType<typeof setTimeout> | null = null
    let latestPayload: TPayload | null = null
    let activeSave: Promise<unknown> | null = null

    function clearTimer() {
        if (timer) {
            clearTimeout(timer)
            timer = null
        }
    }

    async function runSave(payload: TPayload) {
        saveState.value = 'saving'
        errorMessage.value = null

        try {
            activeSave = handler(payload)
            await activeSave
            lastSavedAt.value = Date.now()
            saveState.value = 'saved'
        } catch (error) {
            saveState.value = 'failed'
            errorMessage.value = error instanceof Error ? error.message : '保存失败'
            throw error
        } finally {
            activeSave = null
        }
    }

    function queueSave(payload: TPayload) {
        latestPayload = payload
        saveState.value = 'dirty'
        clearTimer()
        timer = setTimeout(() => {
            const snapshot = latestPayload
            if (!snapshot) return
            void runSave(snapshot)
        }, readDelay(delay))
    }

    async function flushSave() {
        clearTimer()
        if (latestPayload) {
            await runSave(latestPayload)
            latestPayload = null
        } else if (activeSave) {
            await activeSave
        }
    }

    return {
        saveState,
        lastSavedAt,
        errorMessage,
        queueSave,
        flushSave
    }
}
