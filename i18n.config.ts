export default defineI18nConfig(() => ({
    legacy: false,
    locale: 'zh-CN',
    fallbackLocale: 'en',
    messages: {
        'zh-CN': {
            app: {
                name: 'Mythistorima',
                tagline: '为小说作者设计的本地写作工具',
                workspace: {
                    writing: '写作',
                    cards: '设定',
                    notes: '事项',
                    search: '搜索',
                    export: '导出',
                    settings: '设置'
                },
                common: {
                    loading: '正在加载…',
                    empty: '暂无内容',
                    retry: '重试',
                    close: '关闭',
                    save: '保存',
                    saved: '已保存',
                    failed: '操作失败'
                },
                settings: {
                    title: '主题与设置',
                    theme: '主题',
                    editor: '编辑器',
                    autosave: '自动保存',
                    language: '语言'
                }
            }
        },
        en: {
            app: {
                name: 'Mythistorima',
                tagline: 'A local writing workspace for novelists',
                workspace: {
                    writing: 'Writing',
                    cards: 'Story Bible',
                    notes: 'Notes',
                    search: 'Search',
                    export: 'Export',
                    settings: 'Settings'
                },
                common: {
                    loading: 'Loading…',
                    empty: 'Nothing here yet',
                    retry: 'Retry',
                    close: 'Close',
                    save: 'Save',
                    saved: 'Saved',
                    failed: 'Something went wrong'
                },
                settings: {
                    title: 'Theme & Settings',
                    theme: 'Theme',
                    editor: 'Editor',
                    autosave: 'Autosave',
                    language: 'Language'
                }
            }
        }
    }
}))
