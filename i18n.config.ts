export default defineI18nConfig(() => ({
    legacy: false,
    locale: 'zh-CN',
    fallbackLocale: 'en',
    messages: {
        'zh-CN': {
            app: {
                name: 'Mythistorima',
                tagline: '为小说作者设计的本地写作工具'
            }
        },
        en: {
            app: {
                name: 'Mythistorima',
                tagline: 'A local writing workspace for novelists'
            }
        }
    }
}))
