<script setup lang="ts">
import {MoonIcon, SunIcon, LanguagesIcon, LucideCheck} from "@lucide/vue";
import {
  Menubar,
  MenubarContent,
  MenubarItem,
  MenubarMenu,
  MenubarTrigger
} from "~/components/ui/menubar";


const currentTheme = useColorMode()

function changeTheme() {
  currentTheme.preference = currentTheme.value === 'dark' ? 'light' : 'dark';
}

const { locale, setLocale } = useI18n();

function changeLanguage(language: 'ru' | 'en') {
  setLocale(language)
}

</script>

<template>
  <div class="absolute z-10 w-full bg-card flex items-center justify-center">
    <div class="w-full flex items-center justify-between">
      <img src="~/assets/aurorite.svg" alt="logo" class="h-8 mx-2">
      <div class="flex items-center justify-center text-2xl">
        <Menubar class="border-0 bg-card hover:bg-background gap-0">
          <MenubarMenu>
            <MenubarTrigger class="cursor-pointer p-1"><LanguagesIcon class="text-card-foreground"/></MenubarTrigger>
            <MenubarContent class="bg-card border-card">
              <MenubarItem class="cursor-pointer text-card-foreground" @click="changeLanguage('en')">EN
                <LucideCheck v-if="locale === 'en'" class="text-card-foreground" />
              </MenubarItem>
              <MenubarItem class="cursor-pointer text-card-foreground" @click="changeLanguage('ru')">RU
                <LucideCheck v-if="locale === 'ru'" class="text-card-foreground" />
              </MenubarItem>
            </MenubarContent>
          </MenubarMenu>
        </Menubar>
        <div class="flex p-1 mx-2" @click="changeTheme">
          <MoonIcon class="cursor-pointer hover:bg-background h-8 w-8 p-1 rounded-sm block dark:hidden" />
          <SunIcon class="cursor-pointer hover:bg-background h-8 w-8 p-1 rounded-sm hidden dark:block" />
        </div>
      </div>
    </div>
  </div>
</template>
