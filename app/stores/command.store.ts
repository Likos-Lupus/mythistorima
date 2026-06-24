import {appCommandRegistry, commandDefinitionMap, defaultShortcutMap} from '~/constants/commandRegistry'
import type {AppCommandId, ShortcutConflict, ShortcutMap} from '~/types/command'
import {normalizeShortcut} from '~/utils/shortcut'
import {useSettingsStore} from '~/stores/settings.store'

const SHORTCUT_SETTING_KEY = 'app.shortcuts.v1'

function parseShortcutMap(raw: string | undefined): Partial<ShortcutMap> {
    if (!raw) return {}
    try {
        const parsed = JSON.parse(raw) as Record<string, unknown>
        const result: Partial<ShortcutMap> = {}
        for (const command of appCommandRegistry) {
            const value = parsed[command.id]
            if (typeof value === 'string') {
                result[command.id] = normalizeShortcut(value)
            }
        }
        return result
    } catch {
        return {}
    }
}

export const useCommandStore = defineStore('command', () => {
    const settingsStore = useSettingsStore()
    const isPaletteOpen = ref(false)
    const shortcuts = ref<ShortcutMap>(defaultShortcutMap())
    const loaded = ref(false)
    const saving = ref(false)
    const recordingCommandId = ref<AppCommandId | null>(null)

    const conflicts = computed<ShortcutConflict[]>(() => {
        const groups = new Map<string, AppCommandId[]>()
        for (const command of appCommandRegistry) {
            const shortcut = normalizeShortcut(shortcuts.value[command.id])
            if (!shortcut) continue
            const ids = groups.get(shortcut) ?? []
            ids.push(command.id)
            groups.set(shortcut, ids)
        }
        return [...groups.entries()]
            .filter(([, ids]) => ids.length > 1)
            .map(([shortcut, commandIds]) => ({shortcut, commandIds}))
    })

    function loadShortcuts() {
        const stored = parseShortcutMap(settingsStore.settings[SHORTCUT_SETTING_KEY]?.valueJson)
        shortcuts.value = {
            ...defaultShortcutMap(),
            ...stored
        }
        loaded.value = true
        return shortcuts.value
    }

    async function setShortcut(commandId: AppCommandId, shortcut: string) {
        const normalized = normalizeShortcut(shortcut)
        const conflict = findConflict(commandId, normalized)
        if (conflict) {
            const command = commandDefinitionMap.get(conflict)
            throw new Error(`快捷键已被“${command?.title ?? conflict}”使用`)
        }

        saving.value = true
        const previous = shortcuts.value
        try {
            shortcuts.value = {...shortcuts.value, [commandId]: normalized}
            await persist()
        } catch (error) {
            shortcuts.value = previous
            throw error
        } finally {
            saving.value = false
        }
    }

    async function clearShortcut(commandId: AppCommandId) {
        saving.value = true
        const previous = shortcuts.value
        try {
            shortcuts.value = {...shortcuts.value, [commandId]: ''}
            await persist()
        } catch (error) {
            shortcuts.value = previous
            throw error
        } finally {
            saving.value = false
        }
    }

    async function resetShortcut(commandId: AppCommandId) {
        const defaultValue = commandDefinitionMap.get(commandId)?.defaultShortcut ?? ''
        await setShortcut(commandId, defaultValue)
    }

    async function resetAllShortcuts() {
        saving.value = true
        const previous = shortcuts.value
        try {
            shortcuts.value = defaultShortcutMap()
            await persist()
        } catch (error) {
            shortcuts.value = previous
            throw error
        } finally {
            saving.value = false
        }
    }

    function findConflict(commandId: AppCommandId, shortcut: string): AppCommandId | null {
        const normalized = normalizeShortcut(shortcut)
        if (!normalized) return null
        const match = appCommandRegistry.find(
            command => command.id !== commandId
                && normalizeShortcut(shortcuts.value[command.id]) === normalized
        )
        return match?.id ?? null
    }

    function shortcutFor(commandId: AppCommandId) {
        return shortcuts.value[commandId] ?? ''
    }

    function beginShortcutRecording(commandId: AppCommandId) {
        recordingCommandId.value = commandId
    }

    function endShortcutRecording() {
        recordingCommandId.value = null
    }

    function openPalette() {
        isPaletteOpen.value = true
    }

    function closePalette() {
        isPaletteOpen.value = false
    }

    function togglePalette() {
        isPaletteOpen.value = !isPaletteOpen.value
    }

    async function persist() {
        await settingsStore.setAppSetting({
            key: SHORTCUT_SETTING_KEY,
            valueJson: JSON.stringify(shortcuts.value)
        })
    }

    return {
        isPaletteOpen,
        shortcuts,
        loaded,
        saving,
        recordingCommandId,
        conflicts,
        loadShortcuts,
        setShortcut,
        clearShortcut,
        resetShortcut,
        resetAllShortcuts,
        findConflict,
        shortcutFor,
        beginShortcutRecording,
        endShortcutRecording,
        openPalette,
        closePalette,
        togglePalette
    }
})
