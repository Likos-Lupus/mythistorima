import type {NovelDocument} from '~/types/document'
import type {OutlineNode, OutlineNodeStatus, OutlineTreeNode} from '~/types/outline'

export const OUTLINE_BOARD_STATUSES = ['planned', 'drafting', 'done'] as const
export type OutlineBoardStatus = typeof OUTLINE_BOARD_STATUSES[number]

export const OUTLINE_STATUS_OPTIONS: Array<{ value: OutlineNodeStatus | 'all'; label: string; description: string }> = [
    {value: 'all', label: '全部', description: '显示所有大纲节点'},
    {value: 'planned', label: '计划', description: '尚未开始写作或仍在规划的剧情'},
    {value: 'drafting', label: '推进中', description: '正在写作、修订或需要继续推进的剧情'},
    {value: 'done', label: '完成', description: '已经完成正文或剧情目标的节点'},
    {value: 'archived', label: '归档', description: '暂时搁置或保留历史记录的节点'}
]

export const OUTLINE_BOARD_COLUMNS: Array<{ status: OutlineBoardStatus; label: string; description: string }> = [
    {status: 'planned', label: '计划', description: '还在构思、待写或待拆分的节点'},
    {status: 'drafting', label: '推进中', description: '正在创作、调整或等待补完的节点'},
    {status: 'done', label: '完成', description: '已经落到正文或完成剧情目标的节点'}
]

export function normalizeOutlineParentId(parentId?: string | null) {
    return parentId && parentId.length > 0 ? parentId : null
}

export function outlineTypeLabel(type: string) {
    switch (type) {
        case 'conflict':
            return '冲突'
        case 'twist':
            return '转折'
        case 'event':
            return '事件'
        case 'arc':
            return '支线'
        case 'theme':
            return '主题'
        case 'note':
            return '备注'
        default:
            return '剧情'
    }
}

export function outlineStatusLabel(status: string) {
    switch (status) {
        case 'drafting':
            return '推进中'
        case 'done':
            return '完成'
        case 'archived':
            return '归档'
        default:
            return '计划'
    }
}

export function outlineStatusDescription(status: string) {
    return OUTLINE_STATUS_OPTIONS.find(option => option.value === status)?.description ?? status
}

export function documentTypeLabel(type: string) {
    switch (type) {
        case 'chapter':
            return '章'
        case 'scene':
            return '场景'
        case 'volume':
            return '卷'
        default:
            return '文档'
    }
}

export function flattenOutlineTree(items: OutlineTreeNode[]) {
    const result: OutlineTreeNode[] = []
    const walk = (nodes: OutlineTreeNode[]) => {
        for (const node of nodes) {
            result.push(node)
            walk(node.children)
        }
    }
    walk(items)
    return result
}

export function sortOutlineNodes(items: OutlineNode[]) {
    return [...items].sort((a, b) => {
        const parentA = normalizeOutlineParentId(a.parentId) ?? ''
        const parentB = normalizeOutlineParentId(b.parentId) ?? ''
        if (parentA !== parentB) return parentA.localeCompare(parentB)
        if (a.sortOrder !== b.sortOrder) return a.sortOrder - b.sortOrder
        return a.createdAt - b.createdAt
    })
}

export function buildOutlineDocumentMap(documents: NovelDocument[]) {
    return new Map(documents.map(document => [document.id, document]))
}

export function generateOutlineMermaid(nodes: OutlineNode[], documents: NovelDocument[] = []) {
    const sorted = sortOutlineNodes(nodes).filter(node => node.status !== 'archived')
    if (!sorted.length) {
        return [
            'flowchart TD',
            '  EMPTY["暂无大纲节点"]',
            '  classDef empty fill:#fff7ed,stroke:#d6a25d,color:#6d4325;',
            '  class EMPTY empty;'
        ].join('\n')
    }

    const documentMap = buildOutlineDocumentMap(documents)
    const idMap = new Map(sorted.map((node, index) => [node.id, `N${index + 1}`]))
    const roots = sorted.filter(node => !normalizeOutlineParentId(node.parentId) || !idMap.has(normalizeOutlineParentId(node.parentId)!))
    const lines = [
        'flowchart TD',
        '  START(["故事大纲"])'
    ]

    for (const node of sorted) {
        const mermaidId = idMap.get(node.id)!
        const linkedDocument = node.linkedDocumentId ? documentMap.get(node.linkedDocumentId) : null
        const documentHint = linkedDocument ? `<br/>📄 ${escapeMermaidLabel(documentTypeLabel(linkedDocument.type))}：${escapeMermaidLabel(linkedDocument.title)}` : ''
        const summaryHint = node.summary.trim() ? `<br/>${escapeMermaidLabel(node.summary.trim()).slice(0, 52)}` : ''
        const label = `${escapeMermaidLabel(outlineTypeLabel(node.type))}：${escapeMermaidLabel(node.title)}<br/>${escapeMermaidLabel(outlineStatusLabel(node.status))}${documentHint}${summaryHint}`
        lines.push(`  ${mermaidId}["${label}"]`)
    }

    for (const root of roots) {
        const rootId = idMap.get(root.id)
        if (rootId) lines.push(`  START --> ${rootId}`)
    }

    for (const node of sorted) {
        const parentId = normalizeOutlineParentId(node.parentId)
        const source = parentId ? idMap.get(parentId) : null
        const target = idMap.get(node.id)
        if (source && target) lines.push(`  ${source} --> ${target}`)
    }

    for (let index = 0; index < roots.length - 1; index += 1) {
        const current = idMap.get(roots[index].id)
        const next = idMap.get(roots[index + 1].id)
        if (current && next) lines.push(`  ${current} -. 同级顺序 .-> ${next}`)
    }

    lines.push('  classDef planned fill:#fff7ed,stroke:#d6a25d,color:#6d4325;')
    lines.push('  classDef drafting fill:#eef6ff,stroke:#6f9ed8,color:#24466d;')
    lines.push('  classDef done fill:#edf8f1,stroke:#76a884,color:#2f6641;')
    lines.push('  classDef start fill:#f9efe0,stroke:#b7834a,color:#6d4325;')
    lines.push('  class START start;')

    for (const node of sorted) {
        const mermaidId = idMap.get(node.id)
        if (mermaidId && OUTLINE_BOARD_STATUSES.includes(node.status as OutlineBoardStatus)) {
            lines.push(`  class ${mermaidId} ${node.status};`)
        }
    }

    return lines.join('\n')
}

function escapeMermaidLabel(value: string) {
    return value
        .replace(/["`]/g, '\'')
        .replace(/[<>]/g, '')
        .replace(/[\[\]]/g, ' ')
        .replace(/[\r\n]+/g, ' ')
        .replace(/\s+/g, ' ')
        .trim()
}
