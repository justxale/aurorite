<script setup lang="ts">
const {t} = useI18n()

</script>
<template>
    <div class="w-screen h-screen flex items-center justify-center">
        <div class="w-[95%] h-[80%] flex justify-between bg-card rounded-xl">
            <div class="w-[20%] mx-auto pt-0.5 mt-5 center">
                <div :class="{'bg-secondary rounded-sm': useRoute().path === '/preferences/appearance'}" class="p-1">
                    <NuxtLink :class="{'hover:bg-secondary': useRoute().path === '/preferences/appearance'}" to="/preferences/appearance" class="text-card-foreground p-1.5 hover:bg-background rounded-sm">{{t('aurorite.ui.appearance')}}</NuxtLink>
                </div>
            </div>
            <USeparator orientation="vertical" class="w-px h-full bg-input" />
            <div class="w-[75%] m-3 pt-1">
                <NuxtPage />
            </div>
        </div>
    </div>
</template>