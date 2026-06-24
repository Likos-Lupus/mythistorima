import {performance} from 'node:perf_hooks'

const DOCUMENT_COUNT = 300
const CARD_COUNT = 1000
const RELATION_COUNT = 2500
const GRAPH_NODE_LIMIT = 160

const documents = Array.from({length: DOCUMENT_COUNT}, (_, index) => ({
    id: `document-${index + 1}`,
    title: `Chapter ${index + 1}`,
    text: `Character ${index % 80} visits Location ${index % 40} and resolves clue ${index}.`
}))

const cards = Array.from({length: CARD_COUNT}, (_, index) => ({
    id: `card-${index + 1}`,
    name: `Character ${index + 1}`,
    type: index % 4 === 0 ? 'location' : 'character',
    description: `Large project setting card ${index + 1}`
}))

const relations = Array.from({length: RELATION_COUNT}, (_, index) => ({
    id: `relation-${index + 1}`,
    sourceCardId: cards[index % CARD_COUNT].id,
    targetCardId: cards[(index * 17 + 11) % CARD_COUNT].id,
    description: `Relation ${index + 1}`
}))

function measure(label, callback) {
    const start = performance.now()
    const result = callback()
    return {
        label,
        durationMs: Number((performance.now() - start).toFixed(3)),
        resultCount: Array.isArray(result) ? result.length : Number(result)
    }
}

const results = [
    measure('command-palette-filter', () => {
        const query = 'character 77'
        return cards
            .filter(card => `${card.name} ${card.description}`.toLowerCase().includes(query))
            .slice(0, 80)
    }),
    measure('document-search-filter', () => {
        const query = 'location 12'
        return documents.filter(document => `${document.title} ${document.text}`.toLowerCase().includes(query))
    }),
    measure('relation-graph-limit', () => cards.filter(card => card.type === 'character').slice(0, GRAPH_NODE_LIMIT)),
    measure('relation-edge-map', () => {
        const visible = new Set(cards.slice(0, GRAPH_NODE_LIMIT).map(card => card.id))
        return relations.filter(relation => visible.has(relation.sourceCardId) && visible.has(relation.targetCardId))
    })
]

const report = {
    dataset: {
        documents: documents.length,
        cards: cards.length,
        relations: relations.length,
        graphNodeLimit: GRAPH_NODE_LIMIT
    },
    results,
    totalDurationMs: Number(results.reduce((sum, item) => sum + item.durationMs, 0).toFixed(3))
}

console.log(JSON.stringify(report, null, 2))
