import type {CardAppearance, ProjectAppearanceSummary} from '~/types/appearance'

export const useAppearanceStore = defineStore('appearance', () => {
    const summary = ref<ProjectAppearanceSummary | null>(null)
    const activeCardId = ref<string | null>(null)
    const cardAppearances = ref<CardAppearance[]>([])
    const loading = ref(false)
    const rebuilding = ref(false)
    const {call} = useTauriInvoke()

    const activeCardSummary = computed(() => summary.value?.cards.find(card => card.cardId === activeCardId.value) ?? null)
    const appearanceMap = computed(() => {
        const map = new Map<string, number>()
        for (const item of summary.value?.appearances ?? []) {
            map.set(`${item.documentId}:${item.cardId}`, item.mentionCount)
        }
        return map
    })

    async function loadSummary(projectId: string) {
        loading.value = true
        try {
            summary.value = await call<ProjectAppearanceSummary>('get_project_appearance_summary', {projectId})
            if (!activeCardId.value || !summary.value.cards.some(card => card.cardId === activeCardId.value)) {
                activeCardId.value = summary.value.cards[0]?.cardId ?? null
            }
            if (activeCardId.value) await loadCardAppearances(projectId, activeCardId.value)
            else cardAppearances.value = []
            return summary.value
        } finally {
            loading.value = false
        }
    }

    async function rebuildStats(projectId: string) {
        rebuilding.value = true
        try {
            summary.value = await call<ProjectAppearanceSummary>('rebuild_appearance_stats', {projectId})
            if (!activeCardId.value || !summary.value.cards.some(card => card.cardId === activeCardId.value)) {
                activeCardId.value = summary.value.cards[0]?.cardId ?? null
            }
            if (activeCardId.value) await loadCardAppearances(projectId, activeCardId.value)
            return summary.value
        } finally {
            rebuilding.value = false
        }
    }

    async function selectCard(projectId: string, cardId: string | null) {
        activeCardId.value = cardId
        if (!cardId) {
            cardAppearances.value = []
            return []
        }
        return loadCardAppearances(projectId, cardId)
    }

    async function loadCardAppearances(projectId: string, cardId: string) {
        cardAppearances.value = await call<CardAppearance[]>('list_card_appearances', {projectId, cardId})
        return cardAppearances.value
    }

    return {
        summary,
        activeCardId,
        activeCardSummary,
        cardAppearances,
        appearanceMap,
        loading,
        rebuilding,
        loadSummary,
        rebuildStats,
        selectCard,
        loadCardAppearances
    }
})
