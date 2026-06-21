import type {CreateDocumentInput, DocumentContent, NovelDocument, UpdateDocumentContentInput} from '~/types/document'
import type {DocumentStats} from '~/types/stats'

export const useDocumentStore = defineStore('document', () => {
    const documents = ref<NovelDocument[]>([])
    const activeDocumentId = ref<string | null>(null)
    const loading = ref(false)

    const activeDocument = computed(() => documents.value.find(item => item.id === activeDocumentId.value) ?? null)
    const {call} = useTauriInvoke()

    async function loadDocuments(projectId: string) {
        loading.value = true
        try {
            documents.value = await call<NovelDocument[]>('list_documents', {projectId})
            if (!activeDocumentId.value || !documents.value.some(item => item.id === activeDocumentId.value)) {
                activeDocumentId.value = documents.value[0]?.id ?? null
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
        documents.value = [...documents.value, document].sort((a, b) => a.sortOrder - b.sortOrder)
        activeDocumentId.value = document.id
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
        if (index >= 0) documents.value[index] = document
        return document
    }

    async function deleteDocument(documentId: string) {
        await call<boolean>('delete_document', {documentId})
        documents.value = documents.value.filter(item => item.id !== documentId)
        if (activeDocumentId.value === documentId) activeDocumentId.value = documents.value[0]?.id ?? null
    }

    async function loadDocumentStats(documentId: string) {
        return call<DocumentStats>('get_document_stats', {documentId})
    }

    return {
        documents,
        activeDocumentId,
        activeDocument,
        loading,
        loadDocuments,
        selectDocument,
        createDocument,
        getDocumentContent,
        updateDocumentContent,
        renameDocument,
        deleteDocument,
        loadDocumentStats
    }
})
