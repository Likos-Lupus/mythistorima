import type {CardReference, CardType, CreateCardInput, SettingCard, UpdateCardInput} from '~/types/card'

const cardTypeOrder = new Map([
    ['character', 0],
    ['location', 1],
    ['organization', 2],
    ['item', 3],
    ['event', 4],
    ['concept', 5]
])

function sortCards(cards: SettingCard[]) {
    return [...cards].sort((a, b) => {
        const orderA = cardTypeOrder.get(a.type) ?? 99
        const orderB = cardTypeOrder.get(b.type) ?? 99
        if (orderA !== orderB) return orderA - orderB
        return b.updatedAt - a.updatedAt
    })
}

function emptyCounts(): Record<CardType, number> {
    return {
        all: 0,
        character: 0,
        location: 0,
        organization: 0,
        item: 0,
        event: 0,
        concept: 0
    }
}

export const useCardStore = defineStore('card', () => {
    const cards = ref<SettingCard[]>([])
    const activeCardId = ref<string | null>(null)
    const typeFilter = ref<CardType>('all')
    const searchQuery = ref('')
    const references = ref<CardReference[]>([])
    const loading = ref(false)
    const saving = ref(false)

    const {call} = useTauriInvoke()

    const activeCard = computed(() => cards.value.find(card => card.id === activeCardId.value) ?? null)
    const filteredCards = computed(() => {
        const query = searchQuery.value.trim().toLowerCase()
        return cards.value.filter(card => {
            if (typeFilter.value !== 'all' && card.type !== typeFilter.value) return false
            if (!query) return true
            return [card.name, card.aliasesJson, card.description, card.fieldsJson]
                .some(value => value.toLowerCase().includes(query))
        })
    })
    const counts = computed(() => {
        const result = emptyCounts()
        result.all = cards.value.length
        for (const card of cards.value) {
            if (card.type in result && card.type !== 'all') {
                result[card.type as CardType] += 1
            }
        }
        return result
    })

    async function loadCards(projectId: string, _cardType: CardType = 'all') {
        loading.value = true
        try {
            cards.value = sortCards(await call<SettingCard[]>('list_cards', {
                projectId,
                cardType: null
            }))
            if (!activeCardId.value || !cards.value.some(card => card.id === activeCardId.value)) {
                activeCardId.value = cards.value[0]?.id ?? null
            }
            if (activeCardId.value) {
                await loadReferences(activeCardId.value)
            } else {
                references.value = []
            }
            return cards.value
        } finally {
            loading.value = false
        }
    }

    async function searchCards(projectId: string, query: string) {
        loading.value = true
        try {
            cards.value = sortCards(await call<SettingCard[]>('search_cards', {projectId, query}))
            if (!activeCardId.value || !cards.value.some(card => card.id === activeCardId.value)) {
                activeCardId.value = cards.value[0]?.id ?? null
            }
            return cards.value
        } finally {
            loading.value = false
        }
    }

    function selectCard(cardId: string | null) {
        activeCardId.value = cardId
        references.value = []
        if (cardId) {
            void loadReferences(cardId)
        }
    }

    function setTypeFilter(type: CardType) {
        typeFilter.value = type
        if (type !== 'all') {
            const first = filteredCards.value[0]
            if (first && !filteredCards.value.some(card => card.id === activeCardId.value)) {
                selectCard(first.id)
            }
        }
    }

    async function createCard(input: CreateCardInput) {
        saving.value = true
        try {
            const card = await call<SettingCard>('create_card', {input})
            cards.value = sortCards([card, ...cards.value.filter(item => item.id !== card.id)])
            activeCardId.value = card.id
            references.value = []
            return card
        } finally {
            saving.value = false
        }
    }

    async function updateCard(input: UpdateCardInput) {
        saving.value = true
        try {
            const card = await call<SettingCard>('update_card', {input})
            const index = cards.value.findIndex(item => item.id === card.id)
            if (index >= 0) {
                cards.value[index] = card
            } else {
                cards.value.push(card)
            }
            cards.value = sortCards(cards.value)
            activeCardId.value = card.id
            return card
        } finally {
            saving.value = false
        }
    }

    async function deleteCard(cardId: string) {
        await call<boolean>('delete_card', {cardId})
        cards.value = cards.value.filter(card => card.id !== cardId)
        if (activeCardId.value === cardId) {
            activeCardId.value = filteredCards.value[0]?.id ?? cards.value[0]?.id ?? null
        }
        references.value = []
    }

    async function loadReferences(cardId: string) {
        references.value = await call<CardReference[]>('list_card_references', {cardId})
        return references.value
    }

    return {
        cards,
        filteredCards,
        activeCardId,
        activeCard,
        typeFilter,
        searchQuery,
        references,
        counts,
        loading,
        saving,
        loadCards,
        searchCards,
        selectCard,
        setTypeFilter,
        createCard,
        updateCard,
        deleteCard,
        loadReferences
    }
})
