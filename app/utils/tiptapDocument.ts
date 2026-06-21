export interface TiptapNodeLike {
    type?: string
    attrs?: Record<string, unknown> | null
    text?: string
    content?: TiptapNodeLike[]
    marks?: unknown[]
}

export interface TiptapDocLike extends TiptapNodeLike {
    type: 'doc'
    content: TiptapNodeLike[]
}

export const EMPTY_TIPTAP_DOCUMENT: TiptapDocLike = {
    type: 'doc',
    content: [
        {
            type: 'paragraph'
        }
    ]
}

export function createParagraphId() {
    const random = typeof crypto !== 'undefined' && 'randomUUID' in crypto
        ? crypto.randomUUID()
        : `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`

    return `p_${random.replace(/-/g, '').slice(0, 20)}`
}

export function isTiptapDoc(value: unknown): value is TiptapDocLike {
    return Boolean(
        value
        && typeof value === 'object'
        && (value as { type?: unknown }).type === 'doc'
        && Array.isArray((value as { content?: unknown }).content)
    )
}

export function parseTiptapDocument(contentJson: unknown): TiptapDocLike {
    if (isTiptapDoc(contentJson)) return normalizeTiptapDocument(contentJson)

    if (typeof contentJson === 'string') {
        try {
            const parsed = JSON.parse(contentJson)
            if (isTiptapDoc(parsed)) return normalizeTiptapDocument(parsed)
        } catch {
            return cloneEmptyDocument()
        }
    }

    return cloneEmptyDocument()
}

export function normalizeTiptapDocument(document: TiptapDocLike): TiptapDocLike {
    return normalizeNode(document) as TiptapDocLike
}

function normalizeNode(node: TiptapNodeLike): TiptapNodeLike {
    const next: TiptapNodeLike = {
        ...node,
        attrs: node.attrs ? {...node.attrs} : undefined
    }

    if (next.type === 'paragraph') {
        next.attrs = {
            ...(next.attrs ?? {}),
            pid: typeof next.attrs?.pid === 'string' && next.attrs.pid.length > 0
                ? next.attrs.pid
                : createParagraphId()
        }
    }

    if (Array.isArray(node.content)) {
        next.content = node.content.map(child => normalizeNode(child))
    }

    return next
}

function cloneEmptyDocument(): TiptapDocLike {
    return normalizeTiptapDocument(JSON.parse(JSON.stringify(EMPTY_TIPTAP_DOCUMENT)))
}
