import {appCommandRegistry} from '~/constants/commandRegistry'
import type {AppCommandId} from '~/types/command'
import {isEditableShortcutTarget, matchesShortcut} from '~/utils/shortcut'
import {useCommandStore} from '~/stores/command.store'

export function useAppShortcuts(
    handlers: Partial<Record<AppCommandId, () => void | Promise<void>>>
) {
    const commandStore = useCommandStore()

    function onKeydown(event: KeyboardEvent) {
        if (event.repeat || commandStore.recordingCommandId) return
        for (const command of appCommandRegistry) {
            const handler = handlers[command.id]
            if (!handler) continue
            const shortcut = commandStore.shortcutFor(command.id)
            if (!shortcut || !matchesShortcut(event, shortcut)) continue
            if (!command.allowInInput && isEditableShortcutTarget(event.target)) continue

            event.preventDefault()
            event.stopPropagation()
            void handler()
            return
        }
    }

    onMounted(() => {
        window.addEventListener('keydown', onKeydown, {capture: true})
    })

    onBeforeUnmount(() => {
        window.removeEventListener('keydown', onKeydown, {capture: true})
    })

    return {onKeydown}
}
