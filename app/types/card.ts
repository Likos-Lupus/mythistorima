export type CardType = 'all' | 'character' | 'location' | 'concept' | 'organization' | 'item' | 'event'
export type ConcreteCardType = Exclude<CardType, 'all'>

export interface SettingCard {
    id: string
    projectId: string
    type: ConcreteCardType | string
    name: string
    aliasesJson: string
    description: string
    fieldsJson: string
    avatarAssetId?: string | null
    createdAt: number
    updatedAt: number
}

export interface CreateCardInput {
    projectId: string
    type: ConcreteCardType | string
    name: string
    aliasesJson?: string | null
    description?: string | null
    fieldsJson?: string | null
    avatarAssetId?: string | null
}

export interface UpdateCardInput {
    cardId: string
    type?: ConcreteCardType | string | null
    name?: string | null
    aliasesJson?: string | null
    description?: string | null
    fieldsJson?: string | null
    avatarAssetId?: string | null
}

export interface CardReference {
    id: string
    projectId: string
    documentId: string
    documentTitle?: string | null
    cardId: string
    displayText: string
    fromPos?: number | null
    toPos?: number | null
    paragraphId?: string | null
    createdAt: number
    updatedAt: number
}

export interface CardFieldDefinition {
    key: string
    label: string
    placeholder?: string
    multiline?: boolean
}

export const cardTypeOptions: Array<{ value: CardType, label: string }> = [
    {value: 'all', label: '全部'},
    {value: 'character', label: '人物'},
    {value: 'location', label: '地点'},
    {value: 'organization', label: '组织'},
    {value: 'item', label: '道具'},
    {value: 'event', label: '事件'},
    {value: 'concept', label: '概念'}
]

export function isConcreteCardType(type: string): type is ConcreteCardType {
    return ['character', 'location', 'concept', 'organization', 'item', 'event'].includes(type)
}

export function normalizeCardType(type: string): ConcreteCardType {
    return isConcreteCardType(type) ? type : 'character'
}

export function cardTypeLabel(type: string) {
    switch (type) {
        case 'character':
            return '人物'
        case 'location':
            return '地点'
        case 'organization':
            return '组织'
        case 'item':
            return '道具'
        case 'event':
            return '事件'
        case 'concept':
            return '概念'
        default:
            return '设定'
    }
}

export function defaultCardName(type: string) {
    switch (type) {
        case 'location':
            return '未命名地点'
        case 'organization':
            return '未命名组织'
        case 'item':
            return '未命名道具'
        case 'event':
            return '未命名事件'
        case 'concept':
            return '未命名概念'
        default:
            return '未命名人物'
    }
}

export function defaultFieldsJson(type: string) {
    switch (type) {
        case 'location':
            return JSON.stringify({atmosphere: '', notes: ''})
        case 'organization':
            return JSON.stringify({scope: '', goal: '', structure: '', notes: ''})
        case 'item':
            return JSON.stringify({owner: '', power: '', limitations: '', notes: ''})
        case 'event':
            return JSON.stringify({time: '', cause: '', consequence: '', notes: ''})
        case 'concept':
            return JSON.stringify({rules: '', limits: '', notes: ''})
        default:
            return JSON.stringify({role: '', motivation: '', notes: ''})
    }
}
