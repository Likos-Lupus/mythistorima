export type CardType = 'all' | 'character' | 'location' | 'concept'

export interface SettingCard {
    id: string
    projectId: string
    type: Exclude<CardType, 'all'> | string
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
    type: Exclude<CardType, 'all'> | string
    name: string
    aliasesJson?: string | null
    description?: string | null
    fieldsJson?: string | null
    avatarAssetId?: string | null
}

export interface UpdateCardInput {
    cardId: string
    type?: Exclude<CardType, 'all'> | string | null
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
