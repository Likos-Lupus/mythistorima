export type BuiltinCardType = 'character' | 'location' | 'concept' | 'organization' | 'item' | 'event'
export type CardType = 'all' | BuiltinCardType | (string & {})
export type ConcreteCardType = BuiltinCardType | (string & {})

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


export type CardSchemaFieldType = 'text' | 'textarea' | 'number' | 'tags'

export interface CardSchemaField {
    key: string
    label: string
    type: CardSchemaFieldType
    placeholder?: string
}

export interface CardTypeDefinition {
    id: string
    projectId?: string | null
    name: string
    icon: string
    color: string
    schemaJson: string
    sortOrder: number
    isBuiltin: boolean
}

export function parseCardSchemaFields(schemaJson: string): CardSchemaField[] {
    try {
        const parsed = JSON.parse(schemaJson)
        const fields = Array.isArray(parsed?.fields) ? parsed.fields : []
        return fields
            .filter((field: unknown): field is CardSchemaField => {
                return !!field
                    && typeof field === 'object'
                    && typeof (field as CardSchemaField).key === 'string'
                    && typeof (field as CardSchemaField).label === 'string'
            })
            .map(field => ({
                key: field.key,
                label: field.label,
                type: ['textarea', 'number', 'tags'].includes(field.type) ? field.type : 'text',
                placeholder: field.placeholder ?? ''
            }))
    } catch {
        return []
    }
}

export function fieldsJsonFromDefinition(definition: CardTypeDefinition) {
    const result: Record<string, string | number | string[]> = {}
    for (const field of parseCardSchemaFields(definition.schemaJson)) {
        result[field.key] = field.type === 'number' ? 0 : field.type === 'tags' ? [] : ''
    }
    return JSON.stringify(result)
}

export const builtinCardTypeDefinitions: CardTypeDefinition[] = [
    {
        id: 'character',
        name: '人物',
        icon: 'i-lucide-user-round',
        color: 'primary',
        sortOrder: 0,
        isBuiltin: true,
        schemaJson: JSON.stringify({
            fields: [
                {key: 'role', label: '定位', type: 'text', placeholder: '主角、配角、反派、导师…'},
                {key: 'motivation', label: '动机', type: 'textarea', placeholder: '这个人物最想得到什么？'},
                {key: 'notes', label: '备注', type: 'textarea', placeholder: '人物关系、口癖、秘密等。'}
            ]
        })
    },
    {
        id: 'location',
        name: '地点',
        icon: 'i-lucide-map-pin',
        color: 'info',
        sortOrder: 1,
        isBuiltin: true,
        schemaJson: JSON.stringify({
            fields: [
                {key: 'atmosphere', label: '氛围', type: 'textarea', placeholder: '视觉特征、气味、声音和整体感觉。'},
                {key: 'notes', label: '备注', type: 'textarea', placeholder: '与剧情、角色或伏笔相关的备注。'}
            ]
        })
    },
    {
        id: 'organization',
        name: '组织',
        icon: 'i-lucide-building-2',
        color: 'secondary',
        sortOrder: 2,
        isBuiltin: true,
        schemaJson: JSON.stringify({
            fields: [
                {key: 'scope', label: '范围', type: 'textarea', placeholder: '组织影响的地域、行业、阶层或势力范围。'},
                {key: 'goal', label: '目标', type: 'textarea', placeholder: '组织公开或隐藏的核心目标。'},
                {key: 'structure', label: '结构', type: 'textarea', placeholder: '首领、派系、层级、成员关系。'},
                {key: 'notes', label: '备注', type: 'textarea'}
            ]
        })
    },
    {
        id: 'item',
        name: '道具',
        icon: 'i-lucide-package',
        color: 'warning',
        sortOrder: 3,
        isBuiltin: true,
        schemaJson: JSON.stringify({
            fields: [
                {key: 'owner', label: '持有者', type: 'text'},
                {key: 'power', label: '作用', type: 'textarea'},
                {key: 'limitations', label: '限制', type: 'textarea'},
                {key: 'notes', label: '备注', type: 'textarea'}
            ]
        })
    },
    {
        id: 'event',
        name: '事件',
        icon: 'i-lucide-calendar-clock',
        color: 'success',
        sortOrder: 4,
        isBuiltin: true,
        schemaJson: JSON.stringify({
            fields: [
                {key: 'time', label: '时间', type: 'text'},
                {key: 'cause', label: '起因', type: 'textarea'},
                {key: 'consequence', label: '结果', type: 'textarea'},
                {key: 'notes', label: '备注', type: 'textarea'}
            ]
        })
    },
    {
        id: 'concept',
        name: '概念',
        icon: 'i-lucide-lightbulb',
        color: 'neutral',
        sortOrder: 5,
        isBuiltin: true,
        schemaJson: JSON.stringify({
            fields: [
                {key: 'rules', label: '规则', type: 'textarea'},
                {key: 'limits', label: '限制', type: 'textarea'},
                {key: 'notes', label: '备注', type: 'textarea'}
            ]
        })
    }
]

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
    ...builtinCardTypeDefinitions.map(type => ({value: type.id, label: type.name}))
]

export function isConcreteCardType(type: string): type is ConcreteCardType {
    return type !== 'all' && type.trim().length > 0
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
            return builtinCardTypeDefinitions.find(option => option.id === type)?.name ?? type
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
