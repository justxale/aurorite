<script setup lang="ts">
import type { NuxtError } from '#app'

const props = defineProps<{ error: NuxtError }>()
</script>

<template>
  <div class="text-center p-10">
    <div v-if="error.statusCode === 500">
      <h1 class="p-5 font-bold text-2xl">Произошла ошибка {{ props.error.statusCode }}</h1>
      <p class="p-2 mb-5 font-bold text-xl">Service Unavailable</p>
    </div>
    <h1 v-else class="p-5 font-bold text-2xl">Произошла ошибка {{ props.error.statusCode }}</h1>
    <p v-if="error.message" class="p-2 mb-5 font-bold text-xl">{{ error.message }}</p>
    <NuxtLink to="/" class="underline font-bold text-xl">Go back home</NuxtLink>
  </div>
</template>
