export default defineNuxtConfig({
    ssr: false,
    telemetry: false,
    compatibilityDate: '2026-06-21',

    modules: [
        '@nuxt/ui',
        '@pinia/nuxt',
        '@nuxtjs/i18n'
    ],

    css: [
        '~/assets/css/main.css'
    ],

    devServer: {
        port: 3000,
        host: '0.0.0.0'
    },

    vite: {
        clearScreen: false,
        envPrefix: ['VITE_', 'TAURI_'],
        resolve: {
            dedupe: [
                '@tiptap/core',
                '@tiptap/extension-paragraph',
                '@tiptap/pm',
                '@tiptap/starter-kit',
                '@tiptap/vue-3',
                'prosemirror-model',
                'prosemirror-state',
                'prosemirror-transform',
                'prosemirror-view'
            ]
        },
        optimizeDeps: {
            include: [
                '@tiptap/core',
                '@tiptap/extension-drag-handle',
                '@tiptap/extension-drag-handle-vue-3',
                '@tiptap/extension-node-range',
                '@tiptap/extension-paragraph',
                '@tiptap/pm/model',
                '@tiptap/pm/state',
                '@tiptap/pm/transform',
                '@tiptap/pm/view',
                '@tiptap/starter-kit',
                '@tiptap/suggestion',
                '@tiptap/vue-3'
            ]
        },
        server: {
            strictPort: true,
            watch: {
                ignored: ['**/src-tauri/**']
            }
        }
    },

    i18n: {
        defaultLocale: 'zh-CN',
        strategy: 'no_prefix',
        locales: [
            {code: 'zh-CN', name: '简体中文'},
            {code: 'en', name: 'English'}
        ],
        vueI18n: './i18n.config.ts'
    },

    fonts: {
        provider: 'bunny'
    }
})
