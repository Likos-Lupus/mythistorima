import type {DocumentOpenTarget, OpenTarget, PendingEditorNavigation} from '~/types/navigation'

export const useNavigationStore = defineStore('navigation', () => {
    const pendingEditorNavigation = ref<PendingEditorNavigation | null>(null)
    const lastTarget = ref<OpenTarget | null>(null)
    const navigationMessage = ref<string | null>(null)
    let requestSequence = 0

    function rememberTarget(target: OpenTarget) {
        lastTarget.value = target
    }

    function requestDocumentNavigation(target: DocumentOpenTarget) {
        requestSequence += 1
        pendingEditorNavigation.value = {
            requestId: requestSequence,
            documentId: target.targetId,
            paragraphId: target.paragraphId ?? null,
            startOffset: target.startOffset ?? null,
            endOffset: target.endOffset ?? null,
            source: target.source ?? null,
            label: target.label ?? null
        }
        lastTarget.value = target
    }

    function consumeEditorNavigation(requestId: number) {
        if (pendingEditorNavigation.value?.requestId === requestId) {
            pendingEditorNavigation.value = null
        }
    }

    function setNavigationMessage(message: string | null) {
        navigationMessage.value = message
    }

    return {
        pendingEditorNavigation,
        lastTarget,
        navigationMessage,
        rememberTarget,
        requestDocumentNavigation,
        consumeEditorNavigation,
        setNavigationMessage
    }
})
