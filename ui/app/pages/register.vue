<script setup lang="ts">

import auroriteDark from "~/assets/aurorite-full-dark.svg";
import auroriteLight from "~/assets/aurorite-full.svg";
import auroriteMobile from "~/assets/aurorite.svg";
import * as z from 'zod'
import {FormControl, FormField, FormMessage} from "~/components/ui/form";
import {Input} from "~/components/ui/input";
import {Button} from "~/components/ui/button";
import {Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger} from "~/components/ui/menubar";
import {LanguagesIcon, LucideCheck, MoonIcon, SunIcon} from "@lucide/vue";
import {toTypedSchema} from "@vee-validate/zod";
import {useForm} from "vee-validate";
import {toast} from "vue-sonner";
import type {FetchError} from 'ofetch';
import type { User } from "~/components/user";

const {t} = useI18n();

const {locale, changeTheme, changeLanguage} = changeFunc()

const formSchema = toTypedSchema(
    z.object({
        nickname: z.string({message: 'aurorite.errors.formRequired'}),
        display_name: z.string({message: 'aurorite.errors.formRequired'}),
        password: z.string({message: 'aurorite.errors.formRequired'}).min(8, 'aurorite.errors.formPasswordCharacters'),
    })
)

const form = useForm({
    validationSchema: formSchema,
})

const token = useCookie('access_token')
const nickname = ref('')
const display_name = ref('')
const password = ref('')

const onSubmit = form.handleSubmit((values) => {
    console.log('Form submitted!', values)
})


const createClient = form.handleSubmit(async () => {
    try {
        await $fetch<User>('http://localhost:11811/client/auth/register', {
            headers: {Authorization: `Bearer ${token.value}`},
            method: 'POST',
            body: {nickname: nickname.value, display_name: display_name.value, password: password.value}})
    } catch(e) {
        const error = e as FetchError

        if (error.status === 404) {
            toast.error(t('aurorite.errors.notFound', {
                username: nickname.value
            }))
        } else if (error.status === 401) {
            toast.error(`User ${nickname.value} is unauthorized`)

        } else if (error.status === 500) {
            showError({
                status: 500,
                statusText: 'Internal Server Error',
            })

        } else if (error.status === 403) {
                showError({
                    status: 403,
                    statusText: 'Forbidden',
                })

        } else {
            showError({
                status: error.status,
                statusText: error.statusText,
            })
        }
    }
})

</script>

<template>
    <div class="w-full min-h-screen flex justify-center items-center">
        <div class="w-[80%] md:w-[50%] lg:w-[40%] h-screen/2 md:h-screen/2.5 justify-items-center">
            <div class="w-full h-full bg-card rounded-[10px]">
                <div class="flex w-full justify-center pt-2">
                    <img :src="auroriteDark" alt="logo" class="h-20 hidden dark:md:block">
                    <img :src="auroriteLight" alt="logo" class="h-20 hidden md:block dark:hidden">
                    <img :src="auroriteMobile" alt="logo" class="h-20 block md:hidden">
                </div>
                <form @submit="onSubmit">
                    <FormField v-slot="{ componentField }" name="nickname">
                        <FormItem>
                            <FormLabel class="pt-5 pb-1 mx-6 ps-2 md:ps-4 text-[1rem] text-card-foreground">
                                {{ t('aurorite.ui.nickname') }}</FormLabel>
                            <FormControl>
                                <Input
                                    v-model="nickname" :placeholder="t('aurorite.ui.nickname')"
                                    class="w-[calc(100%-48px)] mt-1 mx-6 p-1 md:p-2 ps-2 md:ps-4 rounded-lg bg-input text-card-foreground"
                                    type="text"
                                    v-bind="componentField"
                                />
                            </FormControl>
                            <FormMessage class="mx-6 ps-2 md:ps-4 text-[1rem]"/>
                        </FormItem>
                    </FormField>
                    <FormField v-slot="{ componentField }" name="display_name">
                        <FormItem>
                            <FormLabel class="pt-5 pb-1 mx-6 ps-2 md:ps-4 text-[1rem] text-card-foreground">{{ t('aurorite.ui.displayName') }}</FormLabel>
                            <FormControl>
                                <Input
                                    v-model="display_name" :placeholder="t('aurorite.ui.displayName')"
                                    class="w-[calc(100%-48px)] mt-1 mx-6 p-1 md:p-2 ps-2 md:ps-4 rounded-lg bg-input text-card-foreground"
                                    type="text"
                                    v-bind="componentField"
                                />
                            </FormControl>
                            <FormMessage class="mx-6 ps-2 md:ps-4 text-[1rem]"/>
                        </FormItem>
                    </FormField>
                    <FormField v-slot="{ componentField }" name="password">
                        <FormItem>
                            <FormLabel class="pt-5 pb-1 mx-6 ps-2 md:ps-4 text-[1rem] text-card-foreground">{{ t('aurorite.ui.password') }}</FormLabel>
                            <FormControl>
                                <Input
                                    v-model="password" :placeholder="t('aurorite.ui.password')"
                                    class="w-[calc(100%-48px)] mt-1 mx-6 p-1 md:p-2 ps-2 md:ps-4 rounded-lg bg-input text-card-foreground"
                                    type="text"
                                    v-bind="componentField"
                                />
                            </FormControl>
                            <FormMessage class="mx-6 ps-2 md:ps-4 text-[1rem]"/>
                        </FormItem>
                    </FormField>
                    <div class="flex items-center justify-center h-[30%] md:h-[35%] mt-[5%] md:mt-[4%] pb-[1%] md:pb-[2%]">
                        <Button @click="createClient">
                            {{ t('aurorite.ui.register') }}
                        </Button>
                    </div>
                </form>
                <div v-if="useRoute().path === '/register'" class="flex items-center justify-center text-2xl pb-5">
                    <Menubar class="border-0 bg-card hover:bg-background gap-0 p-1 mx-1">
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
                    <div class="flex p-1 mx-1" @click="changeTheme">
                        <MoonIcon class="cursor-pointer hover:bg-background text-card-foreground h-8 w-8 p-1 rounded-sm hidden dark:block" />
                        <SunIcon class="cursor-pointer hover:bg-background text-card-foreground h-8 w-8 p-1 rounded-sm block dark:hidden" />
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
