import type {SaveState} from '~/composables/useAutoSave'

export const useEditorStore = defineStore('editor', () => {
    const saveState = ref<SaveState>('idle')
    const lastSavedAt = ref<number | null>(null)
    const currentCharacterCount = ref(0)
    const errorMessage = ref<string | null>(null)

    function setSaveState(state: SaveState) {
        saveState.value = state
    }

    function setLastSavedAt(value: number | null) {
        lastSavedAt.value = value
    }

    function setCurrentCharacterCount(value: number) {
        currentCharacterCount.value = value
    }

    function setErrorMessage(value: string | null) {
        errorMessage.value = value
    }

    return {
        saveState,
        lastSavedAt,
        currentCharacterCount,
        errorMessage,
        setSaveState,
        setLastSavedAt,
        setCurrentCharacterCount,
        setErrorMessage
    }
})
