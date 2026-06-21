import {invoke} from '@tauri-apps/api/core'

export interface TauriCallError {
    code?: string
    message: string
    detail?: string
}

function normalizeError(error: unknown): TauriCallError {
    if (typeof error === 'string') {
        return {message: error}
    }

    if (error && typeof error === 'object') {
        const value = error as Record<string, unknown>
        return {
            code: typeof value.code === 'string' ? value.code : undefined,
            message: typeof value.message === 'string' ? value.message : 'Tauri command failed',
            detail: typeof value.detail === 'string' ? value.detail : undefined
        }
    }

    return {message: 'Tauri command failed'}
}

export function useTauriInvoke() {
    async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
        try {
            return await invoke<T>(command, args)
        } catch (error) {
            const normalized = normalizeError(error)
            console.error(`[tauri:${command}]`, normalized)
            throw normalized
        }
    }

    return {call}
}
