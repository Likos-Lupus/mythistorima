import {builtinCardTypeDefinitions, type CardSchemaField, type CardTypeDefinition} from '~/types/card'
import {builtinRelationTypeDefinitions, type RelationTypeDefinition} from '~/types/relation'

interface ResourceDefinitionPayload {
    cardTypes: CardTypeDefinition[]
    relationTypes: RelationTypeDefinition[]
}

function storageKey(projectId: string) {
    return `mythistorima.resource-definitions.v1.${projectId}`
}

function makeId(prefix: string, name: string) {
    const normalized = name
        .trim()
        .toLowerCase()
        .replace(/[^a-z0-9\u4e00-\u9fa5]+/g, '-')
        .replace(/^-+|-+$/g, '')
        .slice(0, 28)
    return `${prefix}_${normalized || Date.now().toString(36)}`
}

function cloneFields(fields: CardSchemaField[]) {
    return fields.map(field => ({...field}))
}

function safeParse(raw: string | null): ResourceDefinitionPayload | null {
    if (!raw) return null
    try {
        const parsed = JSON.parse(raw)
        if (!parsed || typeof parsed !== 'object') return null
        return {
            cardTypes: Array.isArray(parsed.cardTypes) ? parsed.cardTypes : [],
            relationTypes: Array.isArray(parsed.relationTypes) ? parsed.relationTypes : []
        }
    } catch {
        return null
    }
}

export const useResourceDefinitionsStore = defineStore('resource-definitions', () => {
    const projectId = ref<string | null>(null)
    const customCardTypes = ref<CardTypeDefinition[]>([])
    const customRelationTypes = ref<RelationTypeDefinition[]>([])

    const cardTypeDefinitions = computed(() => [
        ...builtinCardTypeDefinitions,
        ...customCardTypes.value
    ])

    const relationTypeDefinitions = computed(() => [
        ...builtinRelationTypeDefinitions,
        ...customRelationTypes.value
    ])

    const cardTypeItems = computed(() => cardTypeDefinitions.value.map(type => ({
        label: type.name,
        value: type.id,
        icon: type.icon
    })))

    const cardTypeFilterItems = computed(() => [
        {label: '全部', value: 'all', icon: 'i-lucide-list-filter'},
        ...cardTypeItems.value
    ])

    const relationTypeItems = computed(() => relationTypeDefinitions.value.map(type => ({
        label: type.name,
        value: type.id,
        icon: type.icon
    })))

    const relationTypeFilterItems = computed(() => [
        {label: '全部', value: 'all', icon: 'i-lucide-list-filter'},
        ...relationTypeItems.value
    ])

    function load(nextProjectId: string) {
        projectId.value = nextProjectId
        customCardTypes.value = []
        customRelationTypes.value = []
        if (!import.meta.client) return
        const payload = safeParse(localStorage.getItem(storageKey(nextProjectId)))
        customCardTypes.value = (payload?.cardTypes ?? []).filter(type => !type.isBuiltin)
        customRelationTypes.value = (payload?.relationTypes ?? []).filter(type => !type.isBuiltin)
    }

    function persist() {
        if (!import.meta.client || !projectId.value) return
        const payload: ResourceDefinitionPayload = {
            cardTypes: customCardTypes.value,
            relationTypes: customRelationTypes.value
        }
        localStorage.setItem(storageKey(projectId.value), JSON.stringify(payload))
    }

    function cardDefinitionFor(id: string) {
        return cardTypeDefinitions.value.find(type => type.id === id)
            ?? builtinCardTypeDefinitions.find(type => type.id === 'character')!
    }

    function relationDefinitionFor(id: string) {
        return relationTypeDefinitions.value.find(type => type.id === id)
            ?? builtinRelationTypeDefinitions.find(type => type.id === 'related_to')!
    }

    function createCardType(input: {
        name: string
        icon?: string
        color?: string
        fields?: CardSchemaField[]
    }) {
        const id = uniqueCardTypeId(makeId('card', input.name))
        const type: CardTypeDefinition = {
            id,
            name: input.name.trim() || '自定义设定',
            icon: input.icon?.trim() || 'i-lucide-tag',
            color: input.color?.trim() || 'neutral',
            sortOrder: cardTypeDefinitions.value.length,
            isBuiltin: false,
            schemaJson: JSON.stringify({
                fields: cloneFields(input.fields?.length ? input.fields : [
                    {key: 'notes', label: '备注', type: 'textarea'}
                ])
            })
        }
        customCardTypes.value.push(type)
        persist()
        return type
    }

    function updateCardType(id: string, input: {
        name: string
        icon?: string
        color?: string
        fields: CardSchemaField[]
    }) {
        const index = customCardTypes.value.findIndex(type => type.id === id)
        if (index < 0) return null
        const updated: CardTypeDefinition = {
            ...customCardTypes.value[index],
            name: input.name.trim() || customCardTypes.value[index].name,
            icon: input.icon?.trim() || 'i-lucide-tag',
            color: input.color?.trim() || 'neutral',
            schemaJson: JSON.stringify({fields: cloneFields(input.fields)})
        }
        customCardTypes.value[index] = updated
        persist()
        return updated
    }

    function deleteCardType(id: string) {
        customCardTypes.value = customCardTypes.value.filter(type => type.id !== id)
        persist()
    }

    function createRelationType(input: {
        name: string
        icon?: string
        color?: string
        direction?: string
        sourceTypeConstraints?: string[]
        targetTypeConstraints?: string[]
    }) {
        const id = uniqueRelationTypeId(makeId('relation', input.name))
        const type: RelationTypeDefinition = {
            id,
            name: input.name.trim() || '自定义关系',
            icon: input.icon?.trim() || 'i-lucide-git-commit-horizontal',
            color: input.color?.trim() || 'neutral',
            direction: input.direction || 'directed',
            sourceTypeConstraints: input.sourceTypeConstraints ?? [],
            targetTypeConstraints: input.targetTypeConstraints ?? [],
            sortOrder: relationTypeDefinitions.value.length,
            isBuiltin: false
        }
        customRelationTypes.value.push(type)
        persist()
        return type
    }

    function updateRelationType(id: string, input: {
        name: string
        icon?: string
        color?: string
        direction?: string
        sourceTypeConstraints?: string[]
        targetTypeConstraints?: string[]
    }) {
        const index = customRelationTypes.value.findIndex(type => type.id === id)
        if (index < 0) return null
        const updated: RelationTypeDefinition = {
            ...customRelationTypes.value[index],
            name: input.name.trim() || customRelationTypes.value[index].name,
            icon: input.icon?.trim() || 'i-lucide-git-commit-horizontal',
            color: input.color?.trim() || 'neutral',
            direction: input.direction || 'directed',
            sourceTypeConstraints: input.sourceTypeConstraints ?? [],
            targetTypeConstraints: input.targetTypeConstraints ?? []
        }
        customRelationTypes.value[index] = updated
        persist()
        return updated
    }

    function deleteRelationType(id: string) {
        customRelationTypes.value = customRelationTypes.value.filter(type => type.id !== id)
        persist()
    }

    function uniqueCardTypeId(seed: string) {
        let id = seed
        let index = 2
        const existing = new Set(cardTypeDefinitions.value.map(type => type.id))
        while (existing.has(id)) {
            id = `${seed}_${index}`
            index += 1
        }
        return id
    }

    function uniqueRelationTypeId(seed: string) {
        let id = seed
        let index = 2
        const existing = new Set(relationTypeDefinitions.value.map(type => type.id))
        while (existing.has(id)) {
            id = `${seed}_${index}`
            index += 1
        }
        return id
    }

    return {
        projectId,
        customCardTypes,
        customRelationTypes,
        cardTypeDefinitions,
        relationTypeDefinitions,
        cardTypeItems,
        cardTypeFilterItems,
        relationTypeItems,
        relationTypeFilterItems,
        load,
        cardDefinitionFor,
        relationDefinitionFor,
        createCardType,
        updateCardType,
        deleteCardType,
        createRelationType,
        updateRelationType,
        deleteRelationType
    }
})
