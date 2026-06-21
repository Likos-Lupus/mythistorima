export function toAppErrorMessage(error: unknown, fallback = '操作失败') {
    if (typeof error === 'string' && error.trim()) return error
    if (error instanceof Error && error.message.trim()) return error.message
    if (typeof error === 'object' && error) {
        const maybe = error as { message?: unknown, code?: unknown, detail?: unknown }
        if (typeof maybe.message === 'string' && maybe.message.trim()) return maybe.message
        if (typeof maybe.detail === 'string' && maybe.detail.trim()) return maybe.detail
        if (typeof maybe.code === 'string' && maybe.code.trim()) return `${fallback}：${maybe.code}`
    }
    return fallback
}
