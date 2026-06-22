import type {SettingCard} from '~/types/card'

export type CardRelationDirection = 'directed' | 'undirected' | 'bidirectional'

export type CardRelationType =
    | 'belongs_to'
    | 'owns'
    | 'located_at'
    | 'happens_at'
    | 'participates_in'
    | 'ally'
    | 'conflict'
    | 'family'
    | 'related_to'
    | string

export type CardGraphCardTypeFilter =
    | 'all'
    | 'character'
    | 'location'
    | 'concept'
    | 'organization'
    | 'item'
    | 'event'

export interface CardRelation {
    id: string
    projectId: string
    sourceCardId: string
    targetCardId: string
    relationType: CardRelationType
    description: string
    direction: CardRelationDirection | string
    metadataJson: string
    createdAt: number
    updatedAt: number
}

export interface CreateCardRelationInput {
    projectId: string
    sourceCardId: string
    targetCardId: string
    relationType: CardRelationType
    description?: string | null
    direction?: CardRelationDirection | string | null
    metadataJson?: string | null
}

export interface UpdateCardRelationInput {
    relationId: string
    sourceCardId?: string | null
    targetCardId?: string | null
    relationType?: CardRelationType | null
    description?: string | null
    direction?: CardRelationDirection | string | null
    metadataJson?: string | null
}

export interface CardGraph {
    cards: SettingCard[]
    relations: CardRelation[]
}

export interface RelationTypeOption {
    value: CardRelationType | 'all'
    label: string
    hint: string
}

export const relationTypeOptions: RelationTypeOption[] = [
    {value: 'all', label: '全部', hint: '显示全部关系'},
    {value: 'belongs_to', label: '属于', hint: '人物 A 属于组织 X'},
    {value: 'owns', label: '持有', hint: '人物 B 持有道具 Y'},
    {value: 'located_at', label: '位于', hint: '人物或组织位于地点 Z'},
    {value: 'happens_at', label: '发生于', hint: '事件 E 发生于地点 Z'},
    {value: 'participates_in', label: '参与', hint: '人物或组织参与事件'},
    {value: 'ally', label: '同盟', hint: '合作或同盟'},
    {value: 'conflict', label: '冲突', hint: '对立、敌意或竞争'},
    {value: 'family', label: '亲缘', hint: '亲属或家族关系'},
    {value: 'related_to', label: '相关', hint: '其他弱关系'}
]

export function relationTypeLabel(type: string) {
    return relationTypeOptions.find(option => option.value === type)?.label ?? type
}

export function relationDirectionLabel(direction: string) {
    switch (direction) {
        case 'undirected':
            return '无方向'
        case 'bidirectional':
            return '双向'
        default:
            return '单向'
    }
}
