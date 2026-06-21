import type {Phase1FeatureFlag} from '~/types/phase1'

export const phase1FeatureFlags: Phase1FeatureFlag[] = [
    {
        key: 'document-tree',
        label: '卷 / 章 / 场景文档树',
        week: 2,
        status: 'done',
        description: '已接入 DocumentTree，支持卷 / 章 / 场景、层级创建、重命名、删除、上移下移和状态切换。'
    },
    {
        key: 'setting-cards',
        label: '设定卡',
        week: 4,
        status: 'foundation',
        description: 'cards 与 card_references 表已就绪；Week 4-5 实现 CRUD、@ 引用和悬浮预览。'
    },
    {
        key: 'creative-notes',
        label: '备忘 / 待办 / 伏笔',
        week: 6,
        status: 'foundation',
        description: 'notes 表已支持项目、章节、段落锚点和状态；Week 6 实现工作区。'
    },
    {
        key: 'full-text-search',
        label: '全文搜索',
        week: 7,
        status: 'foundation',
        description: 'SQLite FTS5 search_index 已建立，正文保存时会同步基础索引。'
    },
    {
        key: 'settings-foundation',
        label: '主题与编辑器设置',
        week: 8,
        status: 'foundation',
        description: 'app_settings 与 project_settings 已建立，并写入默认主题、字号、行距、自动保存间隔。'
    }
]
