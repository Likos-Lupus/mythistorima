import type {NovelDocument} from '~/types/document'

export interface OpenDocumentTab {
    documentId: string
    title: string
    pinned: boolean
    preview: boolean
    dirty: boolean
    updatedAt: number
}

const STORAGE_PREFIX = 'mythistorima:document-tabs:'

function tabFromDocument(document: NovelDocument, pinned = false, preview = true): OpenDocumentTab {
    return {
        documentId: document.id,
        title: document.title,
        pinned,
        preview,
        dirty: false,
        updatedAt: document.updatedAt
    }
}

function storageKey(projectId: string) {
    return `${STORAGE_PREFIX}${projectId}`
}

function normalizeTabs(parsed: unknown): OpenDocumentTab[] {
    if (!Array.isArray(parsed)) return []
    return parsed
        .filter(item => item && typeof item === 'object')
        .map(item => item as Partial<OpenDocumentTab>)
        .filter(item => typeof item.documentId === 'string' && typeof item.title === 'string')
        .map(item => ({
            documentId: item.documentId!,
            title: item.title!,
            pinned: Boolean(item.pinned),
            preview: Boolean(item.preview),
            dirty: Boolean(item.dirty),
            updatedAt: typeof item.updatedAt === 'number' ? item.updatedAt : 0
        }))
}

function safeParseSession(raw: string | null): {
    tabs: OpenDocumentTab[],
    activeTabId: string | null,
    recentTabIds: string[]
} {
    if (!raw) return {tabs: [], activeTabId: null, recentTabIds: []}
    try {
        const parsed = JSON.parse(raw) as unknown
        if (Array.isArray(parsed)) {
            return {tabs: normalizeTabs(parsed), activeTabId: null, recentTabIds: []}
        }
        if (!parsed || typeof parsed !== 'object') return {tabs: [], activeTabId: null, recentTabIds: []}
        const session = parsed as { tabs?: unknown, activeTabId?: unknown, recentTabIds?: unknown }
        return {
            tabs: normalizeTabs(session.tabs),
            activeTabId: typeof session.activeTabId === 'string' ? session.activeTabId : null,
            recentTabIds: Array.isArray(session.recentTabIds) ? session.recentTabIds.filter((id): id is string => typeof id === 'string') : []
        }
    } catch {
        return {tabs: [], activeTabId: null, recentTabIds: []}
    }
}

export const useDocumentTabsStore = defineStore('document-tabs', () => {
    const projectId = ref<string | null>(null)
    const tabs = ref<OpenDocumentTab[]>([])
    const activeTabId = ref<string | null>(null)
    const recentTabIds = ref<string[]>([])

    const activeTab = computed(() => tabs.value.find(tab => tab.documentId === activeTabId.value) ?? null)
    const hasTabs = computed(() => tabs.value.length > 0)

    function hydrate(nextProjectId: string, documents: NovelDocument[], activeDocumentId?: string | null) {
        if (projectId.value !== nextProjectId) {
            projectId.value = nextProjectId
            const saved = process.client
                ? safeParseSession(window.localStorage.getItem(storageKey(nextProjectId)))
                : {tabs: [], activeTabId: null, recentTabIds: []}
            tabs.value = saved.tabs
            activeTabId.value = saved.activeTabId
            recentTabIds.value = saved.recentTabIds
        }

        syncDocuments(documents)

        const savedActiveDocument = documents.find(document => document.id === activeTabId.value)
        const activeDocument = documents.find(document => document.id === activeDocumentId)

        if (savedActiveDocument && tabs.value.some(tab => tab.documentId === savedActiveDocument.id)) {
            activate(savedActiveDocument.id)
            return
        }

        if (activeDocument) {
            openPreview(activeDocument)
            return
        }

        if (!activeTabId.value || !tabs.value.some(tab => tab.documentId === activeTabId.value)) {
            activeTabId.value = tabs.value[0]?.documentId ?? null
        }
    }

    function syncDocuments(documents: NovelDocument[]) {
        const documentById = new Map(documents.map(document => [document.id, document]))
        tabs.value = tabs.value
            .filter(tab => documentById.has(tab.documentId))
            .map(tab => {
                const document = documentById.get(tab.documentId)!
                return {
                    ...tab,
                    title: document.title,
                    updatedAt: document.updatedAt
                }
            })
        persist()
    }

    function openPreview(document: NovelDocument) {
        openDocument(document, {pinned: false, preview: true})
    }

    function openPinned(document: NovelDocument) {
        openDocument(document, {pinned: true, preview: false})
    }

    function openDocument(document: NovelDocument, options: { pinned: boolean, preview: boolean }) {
        const existing = tabs.value.find(tab => tab.documentId === document.id)
        if (existing) {
            existing.title = document.title
            existing.updatedAt = document.updatedAt
            if (options.pinned) {
                existing.pinned = true
                existing.preview = false
            }
            activate(document.id)
            persist()
            return
        }

        if (options.preview) {
            tabs.value = tabs.value.filter(tab => tab.pinned || !tab.preview)
        }

        tabs.value.push(tabFromDocument(document, options.pinned, options.preview))
        activate(document.id)
        persist()
    }

    function activate(documentId: string) {
        if (!tabs.value.some(tab => tab.documentId === documentId)) return
        activeTabId.value = documentId
        recentTabIds.value = [documentId, ...recentTabIds.value.filter(id => id !== documentId)].slice(0, 12)
        persist()
    }

    function pin(documentId: string) {
        const tab = tabs.value.find(item => item.documentId === documentId)
        if (!tab) return
        tab.pinned = true
        tab.preview = false
        persist()
    }

    function markDirty(documentId: string, dirty: boolean) {
        const tab = tabs.value.find(item => item.documentId === documentId)
        if (!tab) return
        tab.dirty = dirty
        persist()
    }

    function close(documentId: string) {
        const index = tabs.value.findIndex(tab => tab.documentId === documentId)
        if (index < 0) return null

        tabs.value.splice(index, 1)
        recentTabIds.value = recentTabIds.value.filter(id => id !== documentId)

        if (activeTabId.value !== documentId) {
            persist()
            return activeTabId.value
        }

        const recent = recentTabIds.value.find(id => tabs.value.some(tab => tab.documentId === id))
        const fallback = recent ?? tabs.value[Math.max(0, index - 1)]?.documentId ?? tabs.value[index]?.documentId ?? null
        activeTabId.value = fallback
        if (fallback) activate(fallback)
        persist()
        return fallback
    }

    function closeOthers(documentId: string) {
        tabs.value = tabs.value.filter(tab => tab.documentId === documentId)
        const tab = tabs.value[0]
        if (tab) {
            tab.pinned = true
            tab.preview = false
            activeTabId.value = tab.documentId
            recentTabIds.value = [tab.documentId]
        } else {
            activeTabId.value = null
            recentTabIds.value = []
        }
        persist()
    }

    function closeAll() {
        tabs.value = []
        activeTabId.value = null
        recentTabIds.value = []
        persist()
    }

    function activateRecent(offset = 1) {
        if (tabs.value.length <= 1) return activeTabId.value
        const ordered = recentTabIds.value.filter(id => tabs.value.some(tab => tab.documentId === id))
        const currentIndex = Math.max(0, ordered.indexOf(activeTabId.value ?? ''))
        const next = ordered[(currentIndex + offset) % ordered.length] ?? tabs.value[0]?.documentId ?? null
        if (next) activate(next)
        return next
    }

    function persist() {
        if (!process.client || !projectId.value) return
        window.localStorage.setItem(storageKey(projectId.value), JSON.stringify({
            tabs: tabs.value,
            activeTabId: activeTabId.value,
            recentTabIds: recentTabIds.value
        }))
    }

    function reset() {
        projectId.value = null
        tabs.value = []
        activeTabId.value = null
        recentTabIds.value = []
    }

    return {
        projectId,
        tabs,
        activeTabId,
        recentTabIds,
        activeTab,
        hasTabs,
        hydrate,
        syncDocuments,
        openPreview,
        openPinned,
        activate,
        pin,
        markDirty,
        close,
        closeOthers,
        closeAll,
        activateRecent,
        reset
    }
})
