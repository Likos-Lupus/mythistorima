export type Phase2WorkspaceMode =
    | 'dashboard'
    | 'writing'
    | 'outline'
    | 'board'
    | 'timeline'
    | 'cards'
    | 'relations'
    | 'stats'
    | 'notes'
    | 'foreshadow'
    | 'proofreading'
    | 'search'
    | 'export'
    | 'settings'

export interface Phase2FeatureFlag {
    key: string
    label: string
    week: number
    status: 'foundation' | 'planned' | 'active'
    description: string
}

export interface Phase2WorkspaceNavItem {
    mode: Phase2WorkspaceMode
    label: string
    description: string
    enabled: boolean
}

export interface Phase2WorkspaceNavGroup {
    label: string
    items: Phase2WorkspaceNavItem[]
}

export const phase2FeatureFlags: Phase2FeatureFlag[] = [
    {key: 'outline', label: '大纲系统', week: 2, status: 'active', description: '大纲树、章节绑定和剧情节点。'},
    {
        key: 'outlineBoard',
        label: '大纲看板',
        week: 3,
        status: 'foundation',
        description: '剧情节点看板与 Mermaid 预览。'
    },
    {key: 'timeline', label: '时间线', week: 5, status: 'active', description: '事件顺序、参与角色和地点过滤。'},
    {key: 'relations', label: '关系图', week: 4, status: 'active', description: '设定卡关系和人物关系图。'},
    {
        key: 'foreshadowThreads',
        label: '伏笔线程',
        week: 6,
        status: 'active',
        description: '提出、回收、状态追踪和跳转。'
    },
    {
        key: 'appearanceStats',
        label: '出场统计',
        week: 6,
        status: 'active',
        description: '角色在章节、卷和项目中的出场统计。'
    },
    {
        key: 'proofreading',
        label: '校对规则',
        week: 7,
        status: 'active',
        description: '重复词、标点、超长句和敏感词规则。'
    },
    {
        key: 'exportTemplates',
        label: '导出模板',
        week: 8,
        status: 'active',
        description: '可复用模板、DOCX、EPUB 与 Pixiv 发布格式。'
    },
    {
        key: 'commandPalette',
        label: '命令面板',
        week: 10,
        status: 'planned',
        description: '快速跳转章节、设定、事项与执行命令。'
    }
]

export const phase2WorkspaceGroups: Phase2WorkspaceNavGroup[] = [
    {
        label: '创作',
        items: [
            {mode: 'dashboard', label: '概览', description: '项目进度、目标与备份', enabled: true},
            {mode: 'writing', label: '写作', description: '文档树与富文本编辑器', enabled: true},
            {mode: 'outline', label: '大纲', description: '剧情节点与章节绑定', enabled: true},
            {mode: 'board', label: '看板', description: '大纲节点状态看板', enabled: true},
            {mode: 'timeline', label: '时间线', description: '故事事件顺序与参与者', enabled: true}
        ]
    },
    {
        label: '资料',
        items: [
            {mode: 'cards', label: '设定', description: '人物、地点、概念等设定卡', enabled: true},
            {mode: 'relations', label: '关系图', description: '设定卡之间的关系网络', enabled: true},
            {mode: 'stats', label: '统计', description: '角色出场与项目结构统计', enabled: true}
        ]
    },
    {
        label: '检查',
        items: [
            {mode: 'notes', label: '事项', description: '备忘、待办、伏笔、灵感', enabled: true},
            {mode: 'foreshadow', label: '伏笔', description: '伏笔线程和回收状态', enabled: true},
            {mode: 'proofreading', label: '校对', description: '本地文本规则检查', enabled: true},
            {mode: 'search', label: '搜索', description: '全文搜索和索引重建', enabled: true}
        ]
    },
    {
        label: '输出',
        items: [
            {mode: 'export', label: '导出', description: '导入导出与项目包', enabled: true},
            {mode: 'settings', label: '设置', description: '主题、字体、语言和自动保存', enabled: true}
        ]
    }
]
