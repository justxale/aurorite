// https://nuxt.com/docs/api/configuration/nuxt-config
import tailwindcss from "@tailwindcss/vite";

export default defineNuxtConfig({
    compatibilityDate: "2025-07-15",
    devtools: {enabled: true},

    modules: [
        "@nuxt/eslint",
        "@nuxt/fonts",
        "@nuxt/hints",
        "@nuxt/image",
        "@nuxtjs/color-mode",
        "@nuxtjs/i18n",
        "@pinia/nuxt",
        "shadcn-nuxt",
        "@vueuse/nuxt"
    ],
    vite: {
        plugins: [
            tailwindcss()
        ]
    },
    css: ["~/assets/css/tailwind.css"],
    shadcn: {
        prefix: "",
        componentDir: "@/components/ui"
    },
    i18n: {
        types: "composition",
        strategy: "no_prefix",
        detectBrowserLanguage: {
            useCookie: true,
            cookieKey: "locale",
            redirectOn: "root"
        },
        defaultLocale: "en",
        locales: [
            {
                code: "en",
                language: "en",
                files: ["en.json", { path: "loader.ts", cache: false }],
                name: "English"
            },
            {
                code: "ru",
                language: "ru",
                files: ["ru.json", { path: "loader.ts", cache: false }],
                name: "Русский"
            }
        ],
    }
})