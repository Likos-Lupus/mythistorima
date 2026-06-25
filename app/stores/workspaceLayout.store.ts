export type ToolWindowId =
    'project'
    | 'inspector'
    | 'notes'
    | 'foreshadow'
    | 'proofreading'
    | 'references'
    | 'stats'
    | 'search'
export type ToolWindowSide = 'left' | 'right'
export type ToolWindowPosition = 'top' | 'center' | 'bottom'

export interface ToolWindowState {
    id: ToolWindowId
    visible: boolean
    side: ToolWindowSide
    position: ToolWindowPosition
    size: number
    order: number
}

const STORAGE_PREFIX = 'mythistorima:workspace-layout:v7:'

const DEFAULT_WINDOWS: ToolWindowState[] = [
    {id: 'project', visible: true, side: 'left', position: 'top', size: 312, order: 10},
    {id: 'notes', visible: false, side: 'left', position: 'center', size: 320, order: 20},
    {id: 'foreshadow', visible: false, side: 'left', position: 'center', size: 320, order: 30},
    {id: 'search', visible: false, side: 'left', position: 'bottom', size: 360, order: 40},
    {id: 'inspector', visible: true, side: 'right', position: 'top', size: 312, order: 50},
    {id: 'references', visible: false, side: 'right', position: 'center', size: 320, order: 60},
    {id: 'proofreading', visible: false, side: 'right', position: 'bottom', size: 360, order: 70},
    {id: 'stats', visible: false, side: 'right', position: 'bottom', size: 360, order: 80}
]

function cloneDefaults() {
    return DEFAULT_WINDOWS.map(window => ({...window}))
}

function normalizeSide(value: unknown, fallback: ToolWindowSide): ToolWindowSide {
    return value === 'left' || value === 'right' ? value : fallback
}

function normalizePosition(value: unknown, fallback: ToolWindowPosition, legacySide: unknown): ToolWindowPosition {
    if (value === 'top' || value === 'center' || value === 'bottom') return value
    if (legacySide === 'bottom') return 'bottom'
    return fallback
}

function clampWindowSize(position: ToolWindowPosition, size: number) {
    const min = position === 'bottom' ? 240 : 276
    const max = position === 'bottom' ? 560 : 440
    return Math.min(max, Math.max(min, Math.round(size)))
}

function normalizeWindow(input: Partial<ToolWindowState> & {
    pinned?: unknown
}, fallback: ToolWindowState): ToolWindowState {
    const side = normalizeSide(input.side, fallback.side)
    const position = normalizePosition(input.position, fallback.position, input.side)
    const size = typeof input.size === 'number' && Number.isFinite(input.size)
        ? clampWindowSize(position, input.size)
        : fallback.size

    return {
        id: fallback.id,
        visible: typeof input.visible === 'boolean' ? input.visible : fallback.visible,
        side,
        position,
        size,
        order: typeof input.order === 'number' ? input.order : fallback.order
    }
}

export const useWorkspaceLayoutStore = defineStore('workspace-layout', () => {
    const projectId = ref<string | null>(null)
    const windows = ref<ToolWindowState[]>(cloneDefaults())

    const visibleWindows = computed(() => windows.value.filter(window => window.visible).sort((a, b) => a.order - b.order))
    const hiddenWindows = computed(() => windows.value.filter(window => !window.visible).sort((a, b) => a.order - b.order))
    const leftWindows = computed(() => windows.value.filter(window => window.side === 'left').sort((a, b) => a.order - b.order))
    const rightWindows = computed(() => windows.value.filter(window => window.side === 'right').sort((a, b) => a.order - b.order))
    const leftVisibleWindows = computed(() => visibleWindows.value.filter(window => window.side === 'left'))
    const rightVisibleWindows = computed(() => visibleWindows.value.filter(window => window.side === 'right'))
    const leftSideWindows = computed(() => leftVisibleWindows.value.filter(window => window.position !== 'bottom'))
    const rightSideWindows = computed(() => rightVisibleWindows.value.filter(window => window.position !== 'bottom'))
    const bottomVisibleWindows = computed(() => visibleWindows.value.filter(window => window.position === 'bottom'))
    const activeLeftSize = computed(() => maxVisibleSize('left', false))
    const activeRightSize = computed(() => maxVisibleSize('right', false))
    const activeBottomSize = computed(() => {
        const sizes = bottomVisibleWindows.value.map(window => window.size)
        return sizes.length ? Math.max(...sizes) : 0
    })

    function hydrate(nextProjectId: string) {
        if (projectId.value === nextProjectId) return
        projectId.value = nextProjectId
        windows.value = cloneDefaults()

        if (!process.client) return
        try {
            const raw = window.localStorage.getItem(`${STORAGE_PREFIX}${nextProjectId}`)
            const parsed = raw ? JSON.parse(raw) as Array<Partial<ToolWindowState> & { pinned?: unknown }> : []
            if (!Array.isArray(parsed)) return
            windows.value = normalizeVisibleCollisions(DEFAULT_WINDOWS.map(fallback => {
                const saved = parsed.find(item => item.id === fallback.id) ?? {}
                return normalizeWindow(saved, fallback)
            }))
        } catch {
            windows.value = cloneDefaults()
        }
    }

    function getWindow(id: ToolWindowId) {
        return windows.value.find(window => window.id === id)
    }

    function windowsInSlot(side: ToolWindowSide, position: ToolWindowPosition) {
        return windows.value.filter(window => window.side === side && window.position === position)
    }

    function visibleInSlot(side: ToolWindowSide, position: ToolWindowPosition) {
        return windowsInSlot(side, position).filter(window => window.visible)
    }

    function activeWindowForSlot(side: ToolWindowSide, position: ToolWindowPosition) {
        return visibleInSlot(side, position).sort((a, b) => a.order - b.order)[0] ?? null
    }

    function open(id: ToolWindowId) {
        const window = getWindow(id)
        if (!window) return
        closeSlotPeers(window)
        window.visible = true
        persist()
    }

    function close(id: ToolWindowId) {
        const window = getWindow(id)
        if (!window) return
        window.visible = false
        persist()
    }

    function toggleVisible(id: ToolWindowId) {
        const window = getWindow(id)
        if (!window) return
        if (window.visible) {
            window.visible = false
        } else {
            closeSlotPeers(window)
            window.visible = true
        }
        persist()
    }

    function setSlot(id: ToolWindowId, side: ToolWindowSide, position: ToolWindowPosition) {
        const window = getWindow(id)
        if (!window) return
        window.side = side
        window.position = position
        window.size = clampWindowSize(position, window.size)
        closeSlotPeers(window)
        window.visible = true
        persist()
    }

    function setSide(id: ToolWindowId, side: ToolWindowSide) {
        const window = getWindow(id)
        if (!window) return
        setSlot(id, side, window.position)
    }

    function setPosition(id: ToolWindowId, position: ToolWindowPosition) {
        const window = getWindow(id)
        if (!window) return
        setSlot(id, window.side, position)
    }

    function setSize(id: ToolWindowId, size: number) {
        const window = getWindow(id)
        if (!window || !Number.isFinite(size)) return
        const nextSize = clampWindowSize(window.position, size)

        if (window.position === 'bottom') {
            windows.value.forEach(item => {
                if (item.visible && item.position === 'bottom') item.size = nextSize
            })
        } else {
            windows.value.forEach(item => {
                if (item.visible && item.side === window.side && item.position !== 'bottom') item.size = nextSize
            })
        }

        window.size = nextSize
        persist()
    }

    function resetForProject() {
        windows.value = cloneDefaults()
        persist()
    }

    function closeSlotPeers(target: ToolWindowState) {
        windows.value.forEach(window => {
            if (window.id !== target.id && window.side === target.side && window.position === target.position) {
                window.visible = false
            }
        })
    }

    function normalizeVisibleCollisions(items: ToolWindowState[]) {
        const seen = new Set<string>()
        return items.map(window => {
            if (!window.visible) return window
            const slotKey = `${window.side}:${window.position}`
            if (seen.has(slotKey)) return {...window, visible: false}
            seen.add(slotKey)
            return window
        })
    }

    function maxVisibleSize(side: ToolWindowSide, includeBottom: boolean) {
        const sizes = windows.value
            .filter(window => window.side === side && window.visible && (includeBottom || window.position !== 'bottom'))
            .map(window => window.size)
        return sizes.length ? Math.max(...sizes) : 0
    }

    function persist() {
        if (!process.client || !projectId.value) return
        window.localStorage.setItem(`${STORAGE_PREFIX}${projectId.value}`, JSON.stringify(windows.value))
    }

    return {
        projectId,
        windows,
        visibleWindows,
        hiddenWindows,
        leftWindows,
        rightWindows,
        leftVisibleWindows,
        rightVisibleWindows,
        leftSideWindows,
        rightSideWindows,
        bottomVisibleWindows,
        activeLeftSize,
        activeRightSize,
        activeBottomSize,
        hydrate,
        getWindow,
        activeWindowForSlot,
        open,
        close,
        toggleVisible,
        setSlot,
        setSide,
        setPosition,
        setSize,
        resetForProject
    }
})
