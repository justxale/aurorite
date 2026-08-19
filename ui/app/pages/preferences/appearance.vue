<script lang="ts" setup>
import {Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger} from "~/components/ui/menubar";
import {LucideCheck, ChevronDownIcon} from "@lucide/vue";


const {t} = useI18n()
const {locale, changeLanguage, currentTheme} = changeFunc()

function changeThemeList(theme: 'light' | 'dark' | 'system') {
    currentTheme.preference = theme
}

type Languages = 'ru' | 'en'

const langList = ref<{id: Languages, name: string}[]>([
    { id: 'en', name: 'English'},
    { id: 'ru', name: 'Русский'}
])

type Themes = 'light' | 'dark' | 'system'

const themeList = ref<{id: Themes, name: string}[]>([
    { id: 'light', name: 'aurorite.ui.lightTheme'},
    { id: 'dark', name: 'aurorite.ui.darkTheme'},
    { id: 'system', name: 'aurorite.ui.systemTheme'},
])

const selectedLanguage = computed(() => langList.value.find(lang => lang.id === locale.value)?.name)
const selectedTheme = computed(() => themeList.value.find(theme => theme.id === currentTheme.preference))
</script>

<template>
    <div>
        <div class="flex items-center mb-3">
            <h1 class="text-card-foreground p-0.5 mt-0.5">{{t('aurorite.ui.language')}}:</h1>
            <Menubar class="mx-4 px-0">
                <MenubarMenu>
                    <MenubarTrigger class="text-card-foreground border border-input px-2">{{ t(selectedLanguage!) }}<ChevronDownIcon class="w-4 ms-2"/></MenubarTrigger>
                    <MenubarContent class="bg-input">
                        <MenubarItem v-for="language in langList" :key="language.id" class="focus:bg-secondary" @click="changeLanguage(language.id)">{{t(language.name)}}
                            <LucideCheck v-if="locale === language.id" class="text-card-foreground" />
                        </MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
            </Menubar>
        </div>
        <div class="flex items-center mb-3">
            <h1 class="text-card-foreground p-1">{{t('aurorite.ui.theme')}}:</h1>
            <Menubar class="mx-4 px-0">
                <MenubarMenu>
                    <MenubarTrigger class="text-card-foreground border border-input px-2">{{t(selectedTheme!.name)}}<ChevronDownIcon class="w-4 ms-2"/></MenubarTrigger>
                    <MenubarContent class="bg-input">
                        <MenubarItem v-for="theme in themeList" :key="theme.id" class="cursor-pointer text-card-foreground focus:bg-secondary" @click="changeThemeList(theme.id)">{{t(theme.name)}}
                            <LucideCheck v-if="theme.id === currentTheme.preference" class="text-card-foreground" />
                        </MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
            </Menubar>
        </div>
    </div>
</template>

