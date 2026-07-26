<script setup lang="ts">

import NuxtLayout from "~/layouts/default.vue"
import AuroriteHeader from '~/components/AuroriteHeader.vue';


const layout = 'default'
</script>

<template>
  <div class="relative w-full">
    <NuxtRouteAnnouncer />

    <main class="relative z-2">
      <div class="w-full h-full justify-items-center">
        <AuroriteHeader />
        <NuxtLayout :name="layout">
          <NuxtPage />
        </NuxtLayout>
      </div>
    </main>
  </div>
</template>
