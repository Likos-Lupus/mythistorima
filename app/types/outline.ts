import type {NovelDocument} from '~/types/document'

export type OutlineNodeType = 'plot' | 'conflict' | 'twist' | 'event' | 'arc' | 'theme' | 'note' | string
export type OutlineNodeStatus = 'planned' | 'drafting' | 'done' | 'archived' | string

export interface OutlineNode {
    id: string
    projectId: string
    parentId: string | null
    linkedDocumentId: string | null
    title: string
    type: OutlineNodeType
    summary: string
    status: OutlineNodeStatus
    sortOrder: number
    metadataJson: string
    createdAt: number
    updatedAt: number
}

export interface OutlineTreeNode extends OutlineNode {
    children: OutlineTreeNode[]
    depth: number
    linkedDocument?: NovelDocument | null
}

export interface CreateOutlineNodeInput {
    projectId: string
    parentId?: string | null
    linkedDocumentId?: string | null
    title: string
    type?: OutlineNodeType
    summary?: string
    status?: OutlineNodeStatus
    sortOrder?: number | null
    metadataJson?: string
}

export interface UpdateOutlineNodeInput {
    outlineNodeId: string
    title?: string
    type?: OutlineNodeType
    summary?: string
    status?: OutlineNodeStatus
    metadataJson?: string
}

export interface MoveOutlineNodeInput {
    outlineNodeId: string
    parentId?: string | null
    sortOrder: number
}

export interface LinkOutlineNodeInput {
    outlineNodeId: string
    documentId: string
}

export interface OutlineTypeOption {
    value: OutlineNodeType
    label: string
    description: string
}

export interface OutlineStatusOption {
    value: OutlineNodeStatus
    label: string
}
