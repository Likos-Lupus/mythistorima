import type {OpenTarget} from '~/types/navigation'
import type {SearchResult} from '~/types/search'

export function searchResultToOpenTarget(result: SearchResult): OpenTarget {
    switch (result.targetType) {
        case 'card':
            return {type: 'card', targetId: result.targetId}
        case 'note':
            return {type: 'note', targetId: result.targetId}
        case 'outline':
            return {type: 'outline', targetId: result.targetId}
        case 'timeline':
            return {type: 'timeline', targetId: result.targetId}
        case 'relation':
            return {type: 'relation', targetId: result.targetId}
        case 'foreshadow':
            return {type: 'foreshadow', targetId: result.targetId}
        case 'proofreadingRule':
            return {type: 'proofreadingRule', targetId: result.targetId}
        case 'exportTemplate':
            return {type: 'exportTemplate', targetId: result.targetId}
        default:
            return {
                type: 'document',
                targetId: result.targetId,
                source: 'search',
                label: result.title
            }
    }
}
