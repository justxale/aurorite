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
  locale.value = language
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
            <MenubarTrigger class="cursor-pointer p-1"><LanguagesIcon/></MenubarTrigger>
            <MenubarContent>
              <MenubarItem class="cursor-pointer" @click="changeLanguage('en')">EN
                <LucideCheck v-if="locale === 'en'" />
              </MenubarItem>
              <MenubarItem class="cursor-pointer" @click="changeLanguage('ru')">RU
                <LucideCheck v-if="locale === 'ru'" />
              </MenubarItem>
            </MenubarContent>
          </MenubarMenu>
        </Menubar>
        <div class="flex p-1 mx-2" @click="changeTheme">
          <MoonIcon v-if="currentTheme.value === 'light'" class="cursor-pointer hover:bg-background h-8 w-8 p-1 rounded-sm" />
          <SunIcon v-else class="cursor-pointer hover:bg-background h-8 w-8 p-1 rounded-sm" />
        </div>
      </div>
    </div>
  </div>
</template>
