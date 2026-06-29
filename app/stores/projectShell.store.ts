import {
    getPrimaryViewDefinition,
    getPrimaryViewForWorkspace,
    getSecondaryWorkspaces,
    getWorkspaceDefinition,
    projectPrimaryViews,
    projectWorkspaceDefinitions,
    type ProjectPrimaryView,
    type ProjectWorkspaceMode
} from '~/constants/projectViews'
import type {CommandPaletteItem} from '~/types/command'

type CommandExecutor = (item: CommandPaletteItem) => void | Promise<void>

export const useProjectShellStore = defineStore('project-shell', () => {
    const workspaceMode = ref<ProjectWorkspaceMode>('writing')
    const workspaceTransitionDirection = ref<'forward' | 'backward'>('forward')
    const focusMode = ref(false)
    const settingsOpen = ref(false)
    const projectCharacterCount = ref(0)
    const feedback = ref<string | null>(null)
    const commandItems = shallowRef<CommandPaletteItem[]>([])

    let commandExecutor: CommandExecutor | null = null
    let feedbackTimer: ReturnType<typeof setTimeout> | null = null

    const activePrimaryView = computed(() => getPrimaryViewForWorkspace(workspaceMode.value))
    const activePrimaryDefinition = computed(() => getPrimaryViewDefinition(activePrimaryView.value))
    const activeWorkspaceDefinition = computed(() => getWorkspaceDefinition(workspaceMode.value))
    const secondaryWorkspaces = computed(() => getSecondaryWorkspaces(activePrimaryView.value))

    function primaryOrder(viewId: ProjectPrimaryView) {
        const index = projectPrimaryViews.findIndex(view => view.id === viewId)
        return index < 0 ? 0 : index
    }

    function workspaceOrder(mode: ProjectWorkspaceMode) {
        const index = projectWorkspaceDefinitions.findIndex(workspace => workspace.mode === mode)
        return index < 0 ? 0 : index
    }

    function setTransitionDirection(nextMode: ProjectWorkspaceMode) {
        const currentPrimary = getPrimaryViewForWorkspace(workspaceMode.value)
        const nextPrimary = getPrimaryViewForWorkspace(nextMode)
        const currentPrimaryOrder = primaryOrder(currentPrimary)
        const nextPrimaryOrder = primaryOrder(nextPrimary)
        if (currentPrimaryOrder !== nextPrimaryOrder) {
            workspaceTransitionDirection.value = nextPrimaryOrder > currentPrimaryOrder ? 'forward' : 'backward'
            return
        }
        workspaceTransitionDirection.value = workspaceOrder(nextMode) >= workspaceOrder(workspaceMode.value) ? 'forward' : 'backward'
    }

    function commitWorkspaceMode(mode: ProjectWorkspaceMode) {
        setTransitionDirection(mode)
        workspaceMode.value = mode
        if (mode !== 'writing') focusMode.value = false
    }

    function selectPrimaryView(viewId: ProjectPrimaryView) {
        const definition = getPrimaryViewDefinition(viewId)
        commitWorkspaceMode(definition.defaultMode)
    }

    function selectWorkspaceMode(mode: ProjectWorkspaceMode) {
        commitWorkspaceMode(mode)
    }

    function toggleFocusMode() {
        workspaceMode.value = 'writing'
        focusMode.value = !focusMode.value
    }

    function openSettings() {
        focusMode.value = false
        settingsOpen.value = true
    }

    function closeSettings() {
        settingsOpen.value = false
    }

    function setProjectCharacterCount(value: number) {
        projectCharacterCount.value = Math.max(0, value)
    }

    function setCommandContext(items: CommandPaletteItem[], executor: CommandExecutor) {
        commandItems.value = items
        commandExecutor = executor
    }

    function clearCommandContext() {
        commandItems.value = []
        commandExecutor = null
    }

    async function executeCommandItem(item: CommandPaletteItem) {
        if (!commandExecutor) return
        await commandExecutor(item)
    }

    function showFeedback(message: string, duration = 4000) {
        feedback.value = message
        if (feedbackTimer) clearTimeout(feedbackTimer)
        feedbackTimer = setTimeout(() => {
            feedback.value = null
            feedbackTimer = null
        }, duration)
    }

    function clearFeedback() {
        feedback.value = null
        if (feedbackTimer) {
            clearTimeout(feedbackTimer)
            feedbackTimer = null
        }
    }

    function reset() {
        workspaceMode.value = 'writing'
        focusMode.value = false
        settingsOpen.value = false
        projectCharacterCount.value = 0
        clearCommandContext()
        clearFeedback()
    }

    return {
        workspaceMode,
        workspaceTransitionDirection,
        focusMode,
        settingsOpen,
        projectCharacterCount,
        feedback,
        commandItems,
        activePrimaryView,
        activePrimaryDefinition,
        activeWorkspaceDefinition,
        secondaryWorkspaces,
        selectPrimaryView,
        selectWorkspaceMode,
        toggleFocusMode,
        openSettings,
        closeSettings,
        setProjectCharacterCount,
        setCommandContext,
        clearCommandContext,
        executeCommandItem,
        showFeedback,
        clearFeedback,
        reset
    }
})
