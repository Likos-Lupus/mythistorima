export default defineNuxtConfig({
    ssr: false,
    telemetry: false,
    compatibilityDate: '2026-06-21',

    modules: [
        '@nuxt/ui',
        '@pinia/nuxt',
        '@nuxtjs/i18n'
    ],

    css: ['~/assets/css/main.css'],

    devServer: {
        port: 3000,
        host: '0.0.0.0'
    },

    vite: {
        clearScreen: false,
        envPrefix: ['VITE_', 'TAURI_'],
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
