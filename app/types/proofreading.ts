export type ProofreadingRuleType =
    | 'duplicate_word'
    | 'continuous_punctuation'
    | 'mixed_punctuation'
    | 'long_sentence'
    | 'long_paragraph'
    | 'sensitive_word'
    | 'name_consistency'
    | 'custom_regex'

export type ProofreadingSeverity = 'info' | 'warning' | 'error'

export interface ProofreadingRule {
    id: string
    projectId?: string | null
    name: string
    type: ProofreadingRuleType | string
    pattern?: string | null
    configJson: string
    severity: ProofreadingSeverity | string
    enabled: number
    createdAt: number
    updatedAt: number
}

export interface ProofreadingIssue {
    id: string
    ruleId?: string | null
    documentId: string
    paragraphId?: string | null
    severity: ProofreadingSeverity | string
    message: string
    startOffset?: number | null
    endOffset?: number | null
    suggestion?: string | null
}

export interface CreateProofreadingRuleInput {
    projectId?: string | null
    name: string
    type: ProofreadingRuleType | string
    pattern?: string | null
    configJson?: string | null
    severity?: ProofreadingSeverity | string | null
    enabled?: number | null
}

export interface UpdateProofreadingRuleInput {
    ruleId: string
    name?: string | null
    type?: ProofreadingRuleType | string | null
    pattern?: string | null
    configJson?: string | null
    severity?: ProofreadingSeverity | string | null
    enabled?: number | null
}

export interface ListProofreadingRulesInput {
    projectId: string
    includeBuiltin?: boolean | null
}

export interface RunProofreadingOnDocumentInput {
    documentId: string
    enabledOnly?: boolean | null
    ruleIds?: string[] | null
}

export interface RunProofreadingOnProjectInput {
    projectId: string
    enabledOnly?: boolean | null
    ruleIds?: string[] | null
}

export interface ProofreadingRuleTypeOption {
    value: ProofreadingRuleType
    label: string
    description: string
}

export const proofreadingRuleTypeOptions: ProofreadingRuleTypeOption[] = [
    {value: 'duplicate_word', label: '重复词', description: '检查“的的”“然后然后”等连续重复。'},
    {value: 'continuous_punctuation', label: '连续标点', description: '检查“！！！”“？？？”等过长标点。'},
    {value: 'mixed_punctuation', label: '标点混用', description: '检查中英文标点连用或风格不统一。'},
    {value: 'long_sentence', label: '超长句', description: '按句子长度阈值提示拆句。'},
    {value: 'long_paragraph', label: '超长段落', description: '按段落长度阈值提示拆段。'},
    {value: 'sensitive_word', label: '敏感词', description: '按词表检查需要替换或确认的词。'},
    {value: 'name_consistency', label: '名称一致性', description: '根据人物设定卡检查疑似错写姓名。'},
    {value: 'custom_regex', label: '自定义正则', description: '使用自定义 pattern 命中问题片段。'}
]

export const proofreadingSeverityOptions: Array<{ value: ProofreadingSeverity, label: string }> = [
    {value: 'info', label: '提示'},
    {value: 'warning', label: '警告'},
    {value: 'error', label: '错误'}
]

export function proofreadingRuleTypeLabel(type: string) {
    return proofreadingRuleTypeOptions.find(option => option.value === type)?.label ?? type
}

export function proofreadingSeverityLabel(severity: string) {
    return proofreadingSeverityOptions.find(option => option.value === severity)?.label ?? severity
}

export function isBuiltinProofreadingRule(rule: ProofreadingRule) {
    return !rule.projectId
}
