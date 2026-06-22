import type {
    CreateOutlineNodeInput,
    LinkOutlineNodeInput,
    MoveOutlineNodeInput,
    OutlineNode,
    OutlineNodeStatus,
    OutlineTreeNode,
    UpdateOutlineNodeInput
} from '~/types/outline'
import type {NovelDocument} from '~/types/document'

function normalizeParentId(parentId?: string | null) {
    return parentId && parentId.length > 0 ? parentId : null
}

function sortOutlineList(items: OutlineNode[]) {
    return [...items].sort((a, b) => {
        const parentA = a.parentId ?? ''
        const parentB = b.parentId ?? ''
        if (parentA !== parentB) return parentA.localeCompare(parentB)
        if (a.sortOrder !== b.sortOrder) return a.sortOrder - b.sortOrder
        return a.createdAt - b.createdAt
    })
}

function buildOutlineTree(items: OutlineNode[], documents: NovelDocument[] = []) {
    const documentMap = new Map(documents.map(document => [document.id, document]))
    const nodes = new Map<string, OutlineTreeNode>()
    const roots: OutlineTreeNode[] = []

    for (const item of items) {
        nodes.set(item.id, {
            ...item,
            parentId: normalizeParentId(item.parentId),
            linkedDocument: item.linkedDocumentId ? documentMap.get(item.linkedDocumentId) ?? null : null,
            children: [],
            depth: 0
        })
    }

    for (const node of nodes.values()) {
        const parentId = normalizeParentId(node.parentId)
        const parent = parentId ? nodes.get(parentId) : null
        if (parent) {
            parent.children.push(node)
        } else {
            roots.push(node)
        }
    }

    const sortNodes = (list: OutlineTreeNode[], depth: number) => {
        list.sort((a, b) => {
            if (a.sortOrder !== b.sortOrder) return a.sortOrder - b.sortOrder
            return a.createdAt - b.createdAt
        })
        for (const node of list) {
            node.depth = depth
            sortNodes(node.children, depth + 1)
        }
    }

    sortNodes(roots, 0)
    return roots
}

function flattenTree(nodes: OutlineTreeNode[]) {
    const result: OutlineTreeNode[] = []
    const walk = (items: OutlineTreeNode[]) => {
        for (const item of items) {
            result.push(item)
            walk(item.children)
        }
    }
    walk(nodes)
    return result
}

export const useOutlineStore = defineStore('outline', () => {
    const nodes = ref<OutlineNode[]>([])
    const activeOutlineNodeId = ref<string | null>(null)
    const statusFilter = ref<OutlineNodeStatus | 'all'>('all')
    const loading = ref(false)
    const saving = ref(false)
    const {call} = useTauriInvoke()

    const filteredNodes = computed(() => {
        if (statusFilter.value === 'all') return nodes.value
        return nodes.value.filter(node => node.status === statusFilter.value)
    })

    const activeNode = computed(() => nodes.value.find(node => node.id === activeOutlineNodeId.value) ?? null)

    function treeWithDocuments(documents: NovelDocument[]) {
        return buildOutlineTree(filteredNodes.value, documents)
    }

    function flatTreeWithDocuments(documents: NovelDocument[]) {
        return flattenTree(treeWithDocuments(documents))
    }

    async function loadOutlineNodes(projectId: string) {
        loading.value = true
        try {
            nodes.value = sortOutlineList(await call<OutlineNode[]>('list_outline_nodes', {projectId}))
            if (!activeOutlineNodeId.value || !nodes.value.some(node => node.id === activeOutlineNodeId.value)) {
                activeOutlineNodeId.value = nodes.value[0]?.id ?? null
            }
            return nodes.value
        } finally {
            loading.value = false
        }
    }

    function selectOutlineNode(outlineNodeId: string | null) {
        activeOutlineNodeId.value = outlineNodeId
    }

    async function createOutlineNode(input: CreateOutlineNodeInput) {
        saving.value = true
        try {
            const node = await call<OutlineNode>('create_outline_node', {input})
            await loadOutlineNodes(node.projectId)
            activeOutlineNodeId.value = node.id
            return node
        } finally {
            saving.value = false
        }
    }

    async function updateOutlineNode(input: UpdateOutlineNodeInput) {
        saving.value = true
        try {
            const node = await call<OutlineNode>('update_outline_node', {input})
            const index = nodes.value.findIndex(item => item.id === node.id)
            if (index >= 0) nodes.value[index] = node
            else nodes.value.push(node)
            nodes.value = sortOutlineList(nodes.value)
            activeOutlineNodeId.value = node.id
            return node
        } finally {
            saving.value = false
        }
    }

    async function deleteOutlineNode(outlineNodeId: string) {
        const deletingIds = new Set<string>()
        const collect = (id: string) => {
            deletingIds.add(id)
            for (const child of nodes.value.filter(item => normalizeParentId(item.parentId) === id)) {
                collect(child.id)
            }
        }
        collect(outlineNodeId)
        await call<boolean>('delete_outline_node', {outlineNodeId})
        nodes.value = nodes.value.filter(node => !deletingIds.has(node.id))
        if (activeOutlineNodeId.value && deletingIds.has(activeOutlineNodeId.value)) {
            activeOutlineNodeId.value = nodes.value[0]?.id ?? null
        }
    }

    async function moveOutlineNode(input: MoveOutlineNodeInput) {
        const moved = await call<OutlineNode[]>('move_outline_node', {input})
        nodes.value = sortOutlineList(moved)
        return nodes.value
    }

    async function linkOutlineNodeToDocument(input: LinkOutlineNodeInput) {
        const node = await call<OutlineNode>('link_outline_node_to_document', {input})
        const index = nodes.value.findIndex(item => item.id === node.id)
        if (index >= 0) nodes.value[index] = node
        else nodes.value.push(node)
        activeOutlineNodeId.value = node.id
        return node
    }

    async function unlinkOutlineNodeDocument(outlineNodeId: string) {
        const node = await call<OutlineNode>('unlink_outline_node_document', {outlineNodeId})
        const index = nodes.value.findIndex(item => item.id === node.id)
        if (index >= 0) nodes.value[index] = node
        activeOutlineNodeId.value = node.id
        return node
    }

    return {
        nodes,
        activeOutlineNodeId,
        activeNode,
        statusFilter,
        loading,
        saving,
        filteredNodes,
        treeWithDocuments,
        flatTreeWithDocuments,
        loadOutlineNodes,
        selectOutlineNode,
        createOutlineNode,
        updateOutlineNode,
        deleteOutlineNode,
        moveOutlineNode,
        linkOutlineNodeToDocument,
        unlinkOutlineNodeDocument
    }
})
