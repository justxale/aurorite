// https://nuxt.com/docs/api/configuration/nuxt-config
import tailwindcss from "@tailwindcss/vite";

export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },

  modules: [
    '@nuxt/eslint',
    '@nuxt/fonts',
    '@nuxt/hints',
    '@nuxt/image',
    '@nuxtjs/color-mode',
    '@nuxtjs/i18n',
    '@pinia/nuxt',
    'shadcn-nuxt',
    '@vueuse/nuxt'
  ],
  vite: {
    plugins: [
        tailwindcss()
    ]
  },
  css: ["~/assets/css/tailwind.css"],
  shadcn: {
    prefix: '',
    componentDir: '@/components/ui'
  },
  i18n: {
    defaultLocale: 'en',
    locales: [
      {
        code: 'en',
        file: 'en.ts'
      },
      {
        code: 'ru',
        file: 'ru.ts'
      }
    ],
  }
})