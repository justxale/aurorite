<script setup lang="ts">
import type { NuxtError } from '#app'
import NuxtLayout from "./layouts/default.vue";

declare module 'nuxt/app' {
  interface NuxtLayouts {
    'custom': unknown
  }
}

definePageMeta({
  layout: 'default'
})

const layout = 'default'

const props = defineProps<{ error: NuxtError }>()
const { t } = useI18n()
</script>

<template>
  <NuxtLayout :name="layout">
    <div class="flex w-full h-full items-center justify-center">
      <div class="w-[80%] md:w-[50%] lg:w-[40%] h-screen/2 md:h-screen/2.5 justify-center top-[30%]">
        <div class="relative w-full h-full bg-white mix-blend-normal rounded-[10px]">
          <div class="text-center p-10">
            <div v-if="error.statusCode === 500">
              <h1 class="p-5 font-bold text-2xl">{{ t('aurorite.errors.httpError', {statusCode: props.error.statusCode }) }}</h1>
              <p class="p-2 mb-5 font-bold text-xl">{{ t('aurorite.errors.error500') }}</p>
            </div>
            <h1 v-else class="p-5 font-bold text-2xl">{{ t('aurorite.errors.httpError', {statusCode: props.error.statusCode }) }}</h1>
            <p v-if="error.message" class="p-2 mb-5 font-bold text-xl">{{ error.message }}</p>
            <NuxtLink to="/" class="underline font-bold text-xl">{{ t('aurorite.ui.goBack') }}</NuxtLink>
          </div>
        </div>
      </div>
    </div>
  </NuxtLayout>
</template>
