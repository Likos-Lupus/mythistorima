import type {AppCommandDefinition, AppCommandId, ShortcutMap} from '~/types/command'

export const appCommandRegistry: AppCommandDefinition[] = [
    {
        id: 'commandPalette.open',
        title: '打开命令面板',
        description: '搜索章节、设定、事项并执行常用操作。',
        group: '导航',
        keywords: ['命令', '搜索', '快速打开', 'palette'],
        defaultShortcut: 'Mod+K',
        allowInInput: true
    },
    {
        id: 'document.createChapter',
        title: '创建章节',
        description: '在当前卷下或当前节点同级创建一个新章节。',
        group: '创作',
        keywords: ['新建', '章节', 'chapter'],
        defaultShortcut: 'Mod+Alt+N'
    },
    {
        id: 'card.createCharacter',
        title: '创建人物设定',
        description: '创建一张新人物卡并打开设定工作区。',
        group: '创作',
        keywords: ['新建', '人物', '角色', '设定'],
        defaultShortcut: 'Mod+Shift+N'
    },
    {
        id: 'editor.toggleFocus',
        title: '切换专注模式',
        description: '隐藏或恢复左右辅助面板。',
        group: '写作',
        keywords: ['专注', '沉浸', 'focus'],
        defaultShortcut: 'Mod+Shift+F',
        allowInInput: true
    },
    {
        id: 'proofreading.runCurrent',
        title: '校对当前章节',
        description: '使用启用的规则检查当前文档。',
        group: '检查',
        keywords: ['校对', '检查', '错字', 'proofreading'],
        defaultShortcut: 'Mod+Shift+P',
        allowInInput: true
    },
    {
        id: 'export.currentDocument',
        title: '导出当前章节',
        description: '将当前文档及其子文档快速导出为 TXT。',
        group: '输出',
        keywords: ['导出', '当前章节', 'txt', 'export'],
        defaultShortcut: 'Mod+Shift+E',
        allowInInput: true
    },
    {
        id: 'theme.cycle',
        title: '切换主题',
        description: '在纸张、明亮和夜间主题之间循环。',
        group: '界面',
        keywords: ['主题', '夜间', '明亮', 'paper', 'dark'],
        defaultShortcut: 'Mod+Shift+T',
        allowInInput: true
    },
    {
        id: 'navigation.settings',
        title: '打开设置',
        description: '打开主题、编辑器、语言和快捷键设置。',
        group: '导航',
        keywords: ['设置', '偏好', '快捷键', 'settings'],
        defaultShortcut: 'Mod+,',
        allowInInput: true
    }
]

export const commandDefinitionMap = new Map<AppCommandId, AppCommandDefinition>(
    appCommandRegistry.map(command => [command.id, command])
)

export function defaultShortcutMap(): ShortcutMap {
    return Object.fromEntries(
        appCommandRegistry.map(command => [command.id, command.defaultShortcut])
    ) as ShortcutMap
}
