<script setup lang="ts">
import { Input } from '../components/ui/input'
import { Button } from '../components/ui/button'
import { useForm } from "vee-validate"
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'
import { toTypedSchema } from '@vee-validate/zod'
import * as z from 'zod'
import type { FetchError } from 'ofetch'
import {toast, Toaster} from 'vue-sonner'
import 'vue-sonner/style.css';

const formSchema = toTypedSchema(
    z.object({
      login: z.string().min(3),
      password: z.string().min(8),
  })
)

const form = useForm({
  validationSchema: formSchema,
})

const username = ref('')
const password = ref('')

async function handlerRegister() {
  try{
    await $fetch("http://localhost:11811/client/auth/login", {
      method: 'POST',
      body: {
        login: username.value,
        password: password.value
      },
    })
  }
  catch(e) {
    const error = e as FetchError
    console.log(e)

    if (error.status === 404) {
      toast.error(`User ${username.value} not found`)
    }

    else if (error.status === 500) {
      showError({
        status: 500,
        statusText: 'Internal Server Error',
      })
    }

    else {
      showError({
        status: error.status,
        statusText: error.statusText,
      })
    }
  }
}
// await navigateTo('/<page>')

//   const logObject = form.handleSubmit((values) => {
//     login.value = values.login
//     password.value = values.password
//   })


const onSubmit = form.handleSubmit((values) => {
  console.log('Form submitted!', values)
})

</script>

<template>
    <div class="w-screen h-screen">
        <div class="absolute w-full h-full m-0 text-black rounded-sm border-solid [clip-path:polygon(0_0,100%_0,100%_15%,0_40%)] bg-[linear-gradient(180deg,#6dffaf_95%,transparent)] opacity-70"/>
        <div class="absolute w-full h-full m-0 text-black rounded-sm border-solid opacity-50 bg-[linear-gradient(360deg,#66a8ff_75%,transparent)] [clip-path:polygon(0_0,90%_0%,90%_70%,0_40%)]"/>
        <div class="absolute w-full h-full m-0 text-black rounded-sm border-solid [clip-path:polygon(0_40%,90%_17.5%,90%_70%,0_100%)] bg-[radial-gradient(ellipse_at_75%_50%,#c273ff_0,transparent_75%)] opacity-40"/>

        <div class="flex bg-grey h-full text-black rounded-b-xl border-solid">
            <div class="flex w-full h-full items-center justify-center">
                <div class="w-[80%] md:w-[50%] lg:w-[40%] h-screen/2 md:h-screen/2.5 justify-items-center top-[30%]">
                    <div class="relative w-full h-full bg-white mix-blend-normal rounded-[10px]">

                      <form @submit="onSubmit">
                        <FormField v-slot="{ componentField }" name="username">
                          <FormItem>
                            <FormLabel class="pt-5 pb-1 mx-2 ps-1 md:ps-4">Username</FormLabel>
                            <FormControl>
                              <Input v-model="username" type="text" placeholder="username" v-bind="componentField" class="w-[calc(100%-16px)] mt-1 mx-2 p-1 md:p-2 ps-1 md:ps-4 rounded-lg" />
                            </FormControl>
                            <FormMessage class="mx-2 ps-1 md:ps-4" />
                          </FormItem>
                        </FormField>
                        <FormField v-slot="{ componentField }" name="password">
                          <FormItem>
                            <FormLabel class="pt-5 pb-1 mx-2 ps-1 md:ps-4">Password</FormLabel>
                            <FormControl>
                              <Input v-model="password" type="text" placeholder="password" v-bind="componentField" class="w-[calc(100%-16px)] mt-1 mx-2 p-1 md:p-2 ps-1 md:ps-4 rounded-lg" />
                            </FormControl>
                            <FormMessage class="mx-2 ps-1 md:ps-4" />
                          </FormItem>
                        </FormField>
                        <div class="flex items-center justify-center h-[30%] md:h-[35%] mt-[3%] md:mt-[4%] pb-[1%] md:pb-[4%]">
                            <Button class="transition duration-300 ease-in-out cursor-pointer w-[50%] p-1 m-3 text-[1rem] bg-green rounded-lg mix-blend-normal hover:scale-110 hover:bg-[#A3FFCD]" @click="handlerRegister" >
                                Login
                            </Button>
                        </div>
                      </form>
                    </div>
                </div>
            </div>
          <Toaster position="top-center"/>
        </div>

    </div>
</template>

