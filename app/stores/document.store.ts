import type {
    CreateDocumentInput,
    DocumentContent,
    DocumentTreeNode,
    MoveDocumentInput,
    NovelDocument,
    UpdateDocumentContentInput,
    UpdateDocumentStatusInput
} from '~/types/document'
import type {DocumentStats} from '~/types/stats'

function normalizeParentId(parentId?: string | null) {
    return parentId && parentId.length > 0 ? parentId : null
}

function sortDocumentList(items: NovelDocument[]) {
    return [...items].sort((a, b) => {
        const parentA = a.parentId ?? ''
        const parentB = b.parentId ?? ''
        if (parentA !== parentB) return parentA.localeCompare(parentB)
        if (a.sortOrder !== b.sortOrder) return a.sortOrder - b.sortOrder
        return a.createdAt - b.createdAt
    })
}

function buildTree(items: NovelDocument[]) {
    const nodes = new Map<string, DocumentTreeNode>()
    const roots: DocumentTreeNode[] = []

    for (const item of items) {
        nodes.set(item.id, {
            ...item,
            parentId: normalizeParentId(item.parentId),
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

    const sortNodes = (list: DocumentTreeNode[], depth: number) => {
        list.sort((a, b) => {
            if (a.sortOrder !== b.sortOrder) return a.sortOrder - b.sortOrder
            return a.createdAt - b.createdAt
        })
        for (const item of list) {
            item.depth = depth
            sortNodes(item.children, depth + 1)
        }
    }

    sortNodes(roots, 0)
    return roots
}

function flattenTree(nodes: DocumentTreeNode[]) {
    const result: DocumentTreeNode[] = []
    const walk = (list: DocumentTreeNode[]) => {
        for (const node of list) {
            result.push(node)
            walk(node.children)
        }
    }
    walk(nodes)
    return result
}

export const useDocumentStore = defineStore('document', () => {
    const documents = ref<NovelDocument[]>([])
    const activeDocumentId = ref<string | null>(null)
    const loading = ref(false)

    const activeDocument = computed(() => documents.value.find(item => item.id === activeDocumentId.value) ?? null)
    const documentTree = computed(() => buildTree(documents.value))
    const flatDocumentTree = computed(() => flattenTree(documentTree.value))
    const {call} = useTauriInvoke()

    async function loadDocuments(projectId: string) {
        loading.value = true
        try {
            documents.value = sortDocumentList(await call<NovelDocument[]>('get_document_tree', {projectId}))
            if (!activeDocumentId.value || !documents.value.some(item => item.id === activeDocumentId.value)) {
                activeDocumentId.value = flatDocumentTree.value[0]?.id ?? documents.value[0]?.id ?? null
            }
            return documents.value
        } finally {
            loading.value = false
        }
    }

    function selectDocument(documentId: string) {
        activeDocumentId.value = documentId
    }

    async function createDocument(input: CreateDocumentInput) {
        const document = await call<NovelDocument>('create_document', {input})
        await loadDocuments(document.projectId)
        activeDocumentId.value = document.id
        return document
    }

    async function moveDocument(input: MoveDocumentInput) {
        documents.value = sortDocumentList(await call<NovelDocument[]>('move_document', {input}))
        return documents.value
    }

    async function updateDocumentStatus(input: UpdateDocumentStatusInput) {
        const document = await call<NovelDocument>('update_document_status', {input})
        const index = documents.value.findIndex(item => item.id === document.id)
        if (index >= 0) {
            documents.value[index] = document
            documents.value = sortDocumentList(documents.value)
        }
        return document
    }

    async function getDocumentContent(documentId: string) {
        return call<DocumentContent>('get_document_content', {documentId})
    }

    async function updateDocumentContent(input: UpdateDocumentContentInput) {
        const content = await call<DocumentContent>('update_document_content', {input})
        const index = documents.value.findIndex(item => item.id === input.documentId)
        if (index >= 0) {
            documents.value[index] = {
                ...documents.value[index],
                characterCount: content.characterCount,
                updatedAt: content.updatedAt
            }
        }
        return content
    }

    async function renameDocument(documentId: string, title: string) {
        const document = await call<NovelDocument>('rename_document', {documentId, title})
        const index = documents.value.findIndex(item => item.id === documentId)
        if (index >= 0) {
            documents.value[index] = document
            documents.value = sortDocumentList(documents.value)
        }
        return document
    }

    async function deleteDocument(documentId: string) {
        const deletingIds = new Set<string>()
        const collect = (id: string) => {
            deletingIds.add(id)
            for (const child of documents.value.filter(item => normalizeParentId(item.parentId) === id)) {
                collect(child.id)
            }
        }
        collect(documentId)

        await call<boolean>('delete_document', {documentId})
        documents.value = documents.value.filter(item => !deletingIds.has(item.id))
        if (activeDocumentId.value && deletingIds.has(activeDocumentId.value)) {
            activeDocumentId.value = flatDocumentTree.value[0]?.id ?? documents.value[0]?.id ?? null
        }
    }

    async function loadDocumentStats(documentId: string) {
        return call<DocumentStats>('get_document_stats', {documentId})
    }

    return {
        documents,
        documentTree,
        flatDocumentTree,
        activeDocumentId,
        activeDocument,
        loading,
        loadDocuments,
        selectDocument,
        createDocument,
        moveDocument,
        updateDocumentStatus,
        getDocumentContent,
        updateDocumentContent,
        renameDocument,
        deleteDocument,
        loadDocumentStats
    }
})
