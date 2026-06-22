import type {
    CardGraph,
    CardRelation,
    CardRelationType,
    CreateCardRelationInput,
    UpdateCardRelationInput
} from '~/types/relation'

function sortRelations(relations: CardRelation[]) {
    return [...relations].sort((a, b) => b.updatedAt - a.updatedAt || b.createdAt - a.createdAt)
}

export const useRelationStore = defineStore('relation', () => {
    const graph = ref<CardGraph>({cards: [], relations: []})
    const relations = ref<CardRelation[]>([])
    const activeRelationId = ref<string | null>(null)
    const selectedCardId = ref<string | null>(null)
    const relationTypeFilter = ref<CardRelationType | 'all'>('all')
    const cardTypeFilter = ref('all')
    const loading = ref(false)
    const saving = ref(false)

    const {call} = useTauriInvoke()

    const activeRelation = computed(() => relations.value.find(item => item.id === activeRelationId.value) ?? null)
    const cardsById = computed(() => new Map(graph.value.cards.map(card => [card.id, card])))
    const filteredRelations = computed(() => {
        return relations.value.filter(relation => {
            if (relationTypeFilter.value !== 'all' && relation.relationType !== relationTypeFilter.value) return false
            if (selectedCardId.value && relation.sourceCardId !== selectedCardId.value && relation.targetCardId !== selectedCardId.value) {
                return false
            }
            return true
        })
    })

    const relationCounts = computed(() => {
        const counts: Record<string, number> = {all: relations.value.length}
        for (const relation of relations.value) {
            counts[relation.relationType] = (counts[relation.relationType] ?? 0) + 1
        }
        return counts
    })

    async function loadGraph(projectId: string) {
        loading.value = true
        try {
            const payload = await call<CardGraph>('list_card_graph', {projectId})
            graph.value = payload
            relations.value = sortRelations(payload.relations)
            if (activeRelationId.value && !relations.value.some(item => item.id === activeRelationId.value)) {
                activeRelationId.value = null
            }
            if (selectedCardId.value && !graph.value.cards.some(card => card.id === selectedCardId.value)) {
                selectedCardId.value = null
            }
            return graph.value
        } finally {
            loading.value = false
        }
    }

    async function loadRelations(projectId: string) {
        relations.value = sortRelations(await call<CardRelation[]>('list_card_relations', {
            projectId,
            cardId: selectedCardId.value || null,
            relationType: relationTypeFilter.value === 'all' ? null : relationTypeFilter.value
        }))
        graph.value = {...graph.value, relations: relations.value}
        return relations.value
    }

    async function createRelation(input: CreateCardRelationInput) {
        saving.value = true
        try {
            const relation = await call<CardRelation>('create_card_relation', {input})
            relations.value = sortRelations([relation, ...relations.value.filter(item => item.id !== relation.id)])
            graph.value = {...graph.value, relations: relations.value}
            activeRelationId.value = relation.id
            selectedCardId.value = null
            return relation
        } finally {
            saving.value = false
        }
    }

    async function updateRelation(input: UpdateCardRelationInput) {
        saving.value = true
        try {
            const relation = await call<CardRelation>('update_card_relation', {input})
            const index = relations.value.findIndex(item => item.id === relation.id)
            if (index >= 0) {
                relations.value[index] = relation
            } else {
                relations.value.push(relation)
            }
            relations.value = sortRelations(relations.value)
            graph.value = {...graph.value, relations: relations.value}
            activeRelationId.value = relation.id
            return relation
        } finally {
            saving.value = false
        }
    }

    async function deleteRelation(relationId: string) {
        await call<boolean>('delete_card_relation', {relationId})
        relations.value = relations.value.filter(item => item.id !== relationId)
        graph.value = {...graph.value, relations: relations.value}
        if (activeRelationId.value === relationId) activeRelationId.value = relations.value[0]?.id ?? null
    }

    function selectRelation(relationId: string | null) {
        activeRelationId.value = relationId
    }

    function selectCard(cardId: string | null) {
        selectedCardId.value = selectedCardId.value === cardId ? null : cardId
    }

    function clearSelection() {
        activeRelationId.value = null
        selectedCardId.value = null
    }

    return {
        graph,
        relations,
        filteredRelations,
        relationCounts,
        cardsById,
        activeRelationId,
        activeRelation,
        selectedCardId,
        relationTypeFilter,
        cardTypeFilter,
        loading,
        saving,
        loadGraph,
        loadRelations,
        createRelation,
        updateRelation,
        deleteRelation,
        selectRelation,
        selectCard,
        clearSelection
    }
})
