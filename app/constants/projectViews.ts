export type ProjectPrimaryView =
    | 'overview'
    | 'outline'
    | 'resources'
    | 'writing'
    | 'export'

export type ProjectWorkspaceMode =
    | 'dashboard'
    | 'stats'
    | 'outline'
    | 'board'
    | 'timeline'
    | 'cards'
    | 'relations'
    | 'writing'
    | 'notes'
    | 'foreshadow'
    | 'proofreading'
    | 'search'
    | 'export'

export interface ProjectWorkspaceDefinition {
    mode: ProjectWorkspaceMode
    primaryView: ProjectPrimaryView
    label: string
    description: string
    icon: string
}

export interface ProjectPrimaryViewDefinition {
    id: ProjectPrimaryView
    label: string
    description: string
    icon: string
    defaultMode: ProjectWorkspaceMode
}

export const projectPrimaryViews: ProjectPrimaryViewDefinition[] = [
    {
        id: 'overview',
        label: '概览',
        description: '作品信息、统计与项目级设置',
        icon: 'i-lucide-layout-dashboard',
        defaultMode: 'dashboard'
    },
    {
        id: 'outline',
        label: '大纲',
        description: '看板、结构树与故事时间线',
        icon: 'i-lucide-git-branch',
        defaultMode: 'board'
    },
    {
        id: 'resources',
        label: '资料',
        description: '设定资料与关系网络',
        icon: 'i-lucide-library',
        defaultMode: 'cards'
    },
    {
        id: 'writing',
        label: '写作',
        description: '正文编辑与创作辅助工具',
        icon: 'i-lucide-pen-line',
        defaultMode: 'writing'
    },
    {
        id: 'export',
        label: '导出',
        description: '模板、预览与发布格式',
        icon: 'i-lucide-file-output',
        defaultMode: 'export'
    }
]

export const projectWorkspaceDefinitions: ProjectWorkspaceDefinition[] = [
    {
        mode: 'dashboard',
        primaryView: 'overview',
        label: '项目',
        description: '编辑作品资料、目标和项目级选项。',
        icon: 'i-lucide-notebook-text'
    },
    {
        mode: 'stats',
        primaryView: 'overview',
        label: '统计',
        description: '查看字数、人物出场和结构统计。',
        icon: 'i-lucide-chart-no-axes-column'
    },
    {
        mode: 'board',
        primaryView: 'outline',
        label: '看板',
        description: '按计划、进行中和完成组织剧情节点。',
        icon: 'i-lucide-columns-3'
    },
    {
        mode: 'outline',
        primaryView: 'outline',
        label: '结构',
        description: '管理剧情节点并绑定卷、章和场景。',
        icon: 'i-lucide-list-tree'
    },
    {
        mode: 'timeline',
        primaryView: 'outline',
        label: '时间线',
        description: '管理故事事件、参与者和地点。',
        icon: 'i-lucide-clock-3'
    },
    {
        mode: 'cards',
        primaryView: 'resources',
        label: '设定',
        description: '管理人物、地点、组织、道具和概念。',
        icon: 'i-lucide-contact-round'
    },
    {
        mode: 'relations',
        primaryView: 'resources',
        label: '关系',
        description: '查看和编辑设定之间的关系网络。',
        icon: 'i-lucide-share-2'
    },
    {
        mode: 'writing',
        primaryView: 'writing',
        label: '正文',
        description: '使用文档树和富文本编辑器创作。',
        icon: 'i-lucide-file-pen-line'
    },
    {
        mode: 'notes',
        primaryView: 'writing',
        label: '事项',
        description: '管理备忘、待办、问题和灵感。',
        icon: 'i-lucide-list-checks'
    },
    {
        mode: 'foreshadow',
        primaryView: 'writing',
        label: '伏笔',
        description: '追踪伏笔的提出、计划和回收。',
        icon: 'i-lucide-sparkles'
    },
    {
        mode: 'proofreading',
        primaryView: 'writing',
        label: '校对',
        description: '检查重复、标点、长句和名称一致性。',
        icon: 'i-lucide-spell-check-2'
    },
    {
        mode: 'search',
        primaryView: 'writing',
        label: '搜索',
        description: '搜索正文、资料、事项和项目结构。',
        icon: 'i-lucide-search'
    },
    {
        mode: 'export',
        primaryView: 'export',
        label: '导出',
        description: '选择模板、范围和输出格式。',
        icon: 'i-lucide-file-output'
    }
]

export function getPrimaryViewForWorkspace(mode: ProjectWorkspaceMode): ProjectPrimaryView {
    return projectWorkspaceDefinitions.find(item => item.mode === mode)?.primaryView ?? 'writing'
}

export function getPrimaryViewDefinition(id: ProjectPrimaryView) {
    return projectPrimaryViews.find(item => item.id === id) ?? projectPrimaryViews[0]!
}

export function getWorkspaceDefinition(mode: ProjectWorkspaceMode) {
    return projectWorkspaceDefinitions.find(item => item.mode === mode) ?? projectWorkspaceDefinitions[0]!
}

export function getSecondaryWorkspaces(primaryView: ProjectPrimaryView) {
    return projectWorkspaceDefinitions.filter(item => item.primaryView === primaryView)
}
