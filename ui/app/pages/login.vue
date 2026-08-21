<script lang="ts" setup>
import {Input} from '~/components/ui/input'
import {Button} from '~/components/ui/button'
import {useForm} from "vee-validate"
import {FormControl, FormField, FormItem, FormLabel, FormMessage,} from '@/components/ui/form'
import {toTypedSchema} from '@vee-validate/zod'
import * as z from 'zod'
import type {FetchError} from 'ofetch'
import {toast} from 'vue-sonner'
import 'vue-sonner/style.css'
import {useAuthenticationStore} from "~/stores/authenticationStore"
import auroriteDark from '~/assets/aurorite-full-dark.svg'
import auroriteLight from '~/assets/aurorite-full.svg'
import auroriteMobile from '~/assets/aurorite.svg'
import {Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger} from "~/components/ui/menubar"
import {LanguagesIcon, LucideCheck, MoonIcon, SunIcon} from "@lucide/vue"


const {t} = useI18n()

const formSchema = toTypedSchema(
    z.object({
        username: z.string({message: 'aurorite.errors.formRequired'}),
        password: z.string({message: 'aurorite.errors.formRequired'}).min(8, 'aurorite.errors.formPasswordCharacters'),
    })
)

const form = useForm({
    validationSchema: formSchema,
})

const username = ref('')
const password = ref('')

const handleSubmit = form.handleSubmit(async (values) => {
    try {
        const authenticationFetch = await authenticationStore.fetchToken(values.username, values.password)
        const authenticationStorage = await authenticationStore.fetchUser()
        await navigateTo('/')
        return {authenticationFetch, authenticationStorage}
    } catch (e) {
        const error = e as FetchError
        console.log(e)

        if (error.status === 404) {
            toast.error(t('aurorite.errors.notFound', {
                username: username.value
            }))
        } else if (error.status === 401) {
            toast.error(`User ${username.value} is unauthorized`)
        } else if (error.status === 500) {
            showError({
                status: 500,
                statusText: 'Internal Server Error',
            })
        } else {
            showError({
                status: error.status,
                statusText: error.statusText,
            })
        }
    }
})

const onSubmit = form.handleSubmit((values) => {
    console.log('Form submitted!', values)
})

const {locale, changeTheme, changeLanguage} = changeFunc()

const authenticationStore = useAuthenticationStore()

definePageMeta({
    layout: 'headerless',
})
</script>

<template>
    <div class="w-[80%] md:w-[50%] lg:w-[40%] h-screen/2 md:h-screen/2.5 justify-items-center">
        <div class="w-full h-full bg-card rounded-[10px]">
            <div class="flex w-full justify-center pt-2">
                <img :src="auroriteDark" alt="logo" class="h-20 hidden dark:md:block">
                <img :src="auroriteLight" alt="logo" class="h-20 hidden md:block dark:hidden">
                <img :src="auroriteMobile" alt="logo" class="h-20 block md:hidden">
            </div>
            <form @submit="onSubmit">
                <FormField v-slot="{ componentField }" name="username">
                    <FormItem>
                        <FormLabel>
                            {{ t('aurorite.ui.username') }}
                        </FormLabel>
                        <FormControl>
                            <Input
                                v-model="username" :placeholder="t('aurorite.ui.username')"
                                type="text"
                                v-bind="componentField"
                            />
                        </FormControl>
                        <FormMessage/>
                    </FormItem>
                </FormField>
                <FormField v-slot="{ componentField }" name="password">
                    <FormItem>
                        <FormLabel>
                            {{ t('aurorite.ui.password') }}
                        </FormLabel>
                        <FormControl>
                            <Input
                                v-model="password" :placeholder="t('aurorite.ui.password')"
                                class="w-[calc(100%-48px)] mt-1 mx-6 p-1 md:p-2 ps-2 md:ps-4 rounded-lg bg-input text-card-foreground"
                                type="password"
                                v-bind="componentField"
                            />
                        </FormControl>
                        <FormMessage/>
                    </FormItem>
                </FormField>
                <div class="flex items-center justify-center h-[30%] md:h-[35%] mt-[5%] md:mt-[4%] pb-[1%] md:pb-[4%]">
                    <Button @click="handleSubmit">
                        {{ t('aurorite.ui.login') }}
                    </Button>
                </div>
            </form>
            <div v-if="useRoute().path === '/login'" class="flex items-center justify-center text-2xl pb-5">
                <Menubar>
                    <MenubarMenu>
                        <MenubarTrigger><LanguagesIcon class="text-card-foreground"/></MenubarTrigger>
                        <MenubarContent>
                            <MenubarItem @click="changeLanguage('en')">EN
                                <LucideCheck v-if="locale === 'en'" class="text-card-foreground" />
                            </MenubarItem>
                            <MenubarItem @click="changeLanguage('ru')">RU
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
</template>

