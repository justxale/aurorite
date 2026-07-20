<script setup lang="ts">
import type { NuxtError } from '#app'

const props = defineProps<{ error: NuxtError }>()
const { t } = useI18n()
</script>

<template>
  <div class="text-center p-10">
    <div v-if="error.statusCode === 500">
      <h1 class="p-5 font-bold text-2xl">{{ t('aurorite.errors.httpError', {statusCode: props.error.statusCode }) }}</h1>
      <p class="p-2 mb-5 font-bold text-xl">{{ t('aurorite.errors.error500') }}</p>
    </div>
    <h1 v-else class="p-5 font-bold text-2xl">{{ t('aurorite.errors.httpError', {statusCode: props.error.statusCode }) }}</h1>
    <p v-if="error.message" class="p-2 mb-5 font-bold text-xl">{{ error.message }}</p>
    <NuxtLink to="/" class="underline font-bold text-xl">{{ t('aurorite.ui.goBack') }}</NuxtLink>
  </div>
</template>
