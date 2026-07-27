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
import {useWindowSize} from '@vueuse/core'
import auroriteDark from '~/assets/aurorite-full-dark.svg'
import auroriteLight from '~/assets/aurorite-full.svg'
import auroriteMobile from '~/assets/aurorite.svg'

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

const authenticationStore = useAuthenticationStore()

const {width} = useWindowSize()
const colorMode = useColorMode()
</script>

<template>
    <div class="w-[80%] md:w-[50%] lg:w-[40%] h-screen/2 md:h-screen/2.5 justify-items-center">
        <div class="w-full h-full bg-card rounded-[10px]">
            <div class="flex w-full justify-center pt-2">
                <img v-if="width>768 && colorMode.value === 'light'" :src="auroriteDark" alt="logo" class="h-20">
                <img v-else-if="width>768 && colorMode.value === 'dark'" :src="auroriteLight" alt="logo" class="h-20">
                <img v-else :src="auroriteMobile" alt="logo" class="h-20">
            </div>
            <form @submit="onSubmit">
                <FormField v-slot="{ componentField }" name="username">
                    <FormItem>
                        <FormLabel class="pt-5 pb-1 mx-6 ps-2 md:ps-4 text-[1rem] text-card-foreground">
                            {{ t('aurorite.ui.username') }}
                        </FormLabel>
                        <FormControl>
                            <Input
                                v-model="username" :placeholder="t('aurorite.ui.username')"
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
                        <FormLabel class="pt-5 pb-1 mx-6 ps-2 md:ps-4 text-[1rem] text-card-foreground">
                            {{ t('aurorite.ui.password') }}
                        </FormLabel>
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
                <div
                    class="flex items-center justify-center h-[30%] md:h-[35%] mt-[3%] md:mt-[4%] pb-[1%] md:pb-[4%]">
                    <Button @click="handleSubmit">
                        {{ t('aurorite.ui.login') }}
                    </Button>
                </div>
            </form>
        </div>
    </div>
</template>

