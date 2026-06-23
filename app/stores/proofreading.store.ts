import type {
    CreateProofreadingRuleInput,
    ListProofreadingRulesInput,
    ProofreadingIssue,
    ProofreadingRule,
    RunProofreadingOnDocumentInput,
    RunProofreadingOnProjectInput,
    UpdateProofreadingRuleInput
} from '~/types/proofreading'

function sortRules(rules: ProofreadingRule[]) {
    const order = new Map([
        ['duplicate_word', 0],
        ['continuous_punctuation', 1],
        ['mixed_punctuation', 2],
        ['long_sentence', 3],
        ['long_paragraph', 4],
        ['sensitive_word', 5],
        ['name_consistency', 6],
        ['custom_regex', 7]
    ])

    return [...rules].sort((a, b) => {
        const builtInA = a.projectId ? 1 : 0
        const builtInB = b.projectId ? 1 : 0
        if (builtInA !== builtInB) return builtInA - builtInB
        const orderA = order.get(a.type) ?? 99
        const orderB = order.get(b.type) ?? 99
        if (orderA !== orderB) return orderA - orderB
        return a.name.localeCompare(b.name)
    })
}

function sortIssues(issues: ProofreadingIssue[]) {
    return [...issues].sort((a, b) => {
        if (a.documentId !== b.documentId) return a.documentId.localeCompare(b.documentId)
        return (a.startOffset ?? 0) - (b.startOffset ?? 0)
    })
}

export const useProofreadingStore = defineStore('proofreading', () => {
    const rules = ref<ProofreadingRule[]>([])
    const issues = ref<ProofreadingIssue[]>([])
    const activeRuleId = ref<string | null>(null)
    const activeIssueId = ref<string | null>(null)
    const loading = ref(false)
    const running = ref(false)
    const saving = ref(false)
    const lastRunScope = ref<'document' | 'project' | null>(null)

    const {call} = useTauriInvoke()

    const activeRule = computed(() => rules.value.find(rule => rule.id === activeRuleId.value) ?? null)
    const enabledRules = computed(() => rules.value.filter(rule => rule.enabled !== 0))
    const issueCountByDocument = computed(() => {
        const result = new Map<string, number>()
        for (const issue of issues.value) {
            result.set(issue.documentId, (result.get(issue.documentId) ?? 0) + 1)
        }
        return result
    })
    const issuesByDocument = computed(() => {
        const result = new Map<string, ProofreadingIssue[]>()
        for (const issue of issues.value) {
            const list = result.get(issue.documentId) ?? []
            list.push(issue)
            result.set(issue.documentId, list)
        }
        return result
    })

    async function loadRules(input: ListProofreadingRulesInput) {
        loading.value = true
        try {
            rules.value = sortRules(await call<ProofreadingRule[]>('list_proofreading_rules', {input}))
            if (!activeRuleId.value || !rules.value.some(rule => rule.id === activeRuleId.value)) {
                activeRuleId.value = rules.value.find(rule => rule.projectId)?.id ?? rules.value[0]?.id ?? null
            }
            return rules.value
        } finally {
            loading.value = false
        }
    }

    async function createRule(input: CreateProofreadingRuleInput) {
        saving.value = true
        try {
            const rule = await call<ProofreadingRule>('create_proofreading_rule', {input})
            rules.value = sortRules([rule, ...rules.value.filter(item => item.id !== rule.id)])
            activeRuleId.value = rule.id
            return rule
        } finally {
            saving.value = false
        }
    }

    async function updateRule(input: UpdateProofreadingRuleInput) {
        saving.value = true
        try {
            const rule = await call<ProofreadingRule>('update_proofreading_rule', {input})
            rules.value = sortRules(rules.value.map(item => item.id === rule.id ? rule : item))
            activeRuleId.value = rule.id
            return rule
        } finally {
            saving.value = false
        }
    }

    async function deleteRule(ruleId: string) {
        saving.value = true
        try {
            await call<boolean>('delete_proofreading_rule', {ruleId})
            rules.value = rules.value.filter(rule => rule.id !== ruleId)
            if (activeRuleId.value === ruleId) {
                activeRuleId.value = rules.value.find(rule => rule.projectId)?.id ?? rules.value[0]?.id ?? null
            }
        } finally {
            saving.value = false
        }
    }

    async function runOnDocument(input: RunProofreadingOnDocumentInput) {
        running.value = true
        try {
            issues.value = sortIssues(await call<ProofreadingIssue[]>('run_proofreading_on_document', {input}))
            lastRunScope.value = 'document'
            activeIssueId.value = issues.value[0]?.id ?? null
            return issues.value
        } finally {
            running.value = false
        }
    }

    async function runOnProject(input: RunProofreadingOnProjectInput) {
        running.value = true
        try {
            issues.value = sortIssues(await call<ProofreadingIssue[]>('run_proofreading_on_project', {input}))
            lastRunScope.value = 'project'
            activeIssueId.value = issues.value[0]?.id ?? null
            return issues.value
        } finally {
            running.value = false
        }
    }

    function selectRule(ruleId: string | null) {
        activeRuleId.value = ruleId
    }

    function selectIssue(issueId: string | null) {
        activeIssueId.value = issueId
    }

    function clearIssues() {
        issues.value = []
        activeIssueId.value = null
        lastRunScope.value = null
    }

    return {
        rules,
        issues,
        activeRuleId,
        activeRule,
        activeIssueId,
        enabledRules,
        issueCountByDocument,
        issuesByDocument,
        loading,
        running,
        saving,
        lastRunScope,
        loadRules,
        createRule,
        updateRule,
        deleteRule,
        runOnDocument,
        runOnProject,
        selectRule,
        selectIssue,
        clearIssues
    }
})
