import type {Phase2WorkspaceMode} from '~/constants/phase2Features'
import type {ProofreadingIssue} from '~/types/proofreading'

export type DocumentAnchorSource =
    | 'search'
    | 'appearance'
    | 'foreshadow-setup'
    | 'foreshadow-payoff'
    | 'proofreading'
    | 'timeline'
    | 'outline'
    | 'note'
    | 'direct'

export interface DocumentOpenTarget {
    type: 'document'
    targetId: string
    paragraphId?: string | null
    startOffset?: number | null
    endOffset?: number | null
    source?: DocumentAnchorSource | null
    label?: string | null
}

export type OpenTarget =
    | DocumentOpenTarget
    | { type: 'card', targetId: string }
    | { type: 'note', targetId: string, documentId?: string | null, paragraphId?: string | null }
    | { type: 'outline', targetId: string }
    | { type: 'timeline', targetId: string }
    | { type: 'relation', targetId: string }
    | { type: 'foreshadow', targetId: string }
    | { type: 'proofreadingRule', targetId: string }
    | { type: 'proofreadingIssue', issue: ProofreadingIssue }
    | { type: 'exportTemplate', targetId: string }
    | { type: 'workspace', workspace: Phase2WorkspaceMode }

export interface PendingEditorNavigation {
    requestId: number
    documentId: string
    paragraphId?: string | null
    startOffset?: number | null
    endOffset?: number | null
    source?: DocumentAnchorSource | null
    label?: string | null
}
