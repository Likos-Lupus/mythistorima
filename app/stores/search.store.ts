import type {SearchProjectInput, SearchResult, SearchScope} from '~/types/search'

export const useSearchStore = defineStore('search', () => {
    const query = ref('')
    const scopes = ref<SearchScope[]>(['all'])
    const results = ref<SearchResult[]>([])
    const loading = ref(false)
    const error = ref<string | null>(null)
    const {call} = useTauriInvoke()

    async function searchProject(input: SearchProjectInput) {
        loading.value = true
        error.value = null
        try {
            query.value = input.query
            scopes.value = input.scopes.length ? input.scopes : ['all']
            results.value = await call<SearchResult[]>('search_project', {input})
            return results.value
        } catch (err) {
            error.value = errorMessage(err, '搜索失败')
            results.value = []
            throw err
        } finally {
            loading.value = false
        }
    }

    async function rebuildIndex(projectId: string) {
        await call<boolean>('rebuild_search_index', {projectId})
    }

    function clear() {
        results.value = []
        error.value = null
    }

    return {query, scopes, results, loading, error, searchProject, rebuildIndex, clear}
})

function errorMessage(error: unknown, fallback: string) {
    return typeof error === 'object' && error && 'message' in error
        ? String((error as { message?: string }).message)
        : fallback
}
