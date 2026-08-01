<script setup lang="ts">
import {useAuthenticationStore} from "~/stores/authenticationStore";
import {navigateTo} from "nuxt/app";

const route = useRoute()
const authenticationStore = useAuthenticationStore()

if (route.path !== '/login') {
    try {
        const token = authenticationStore.token
        if (token) {
            try {
                await authenticationStore.fetchUser()
            }
            catch (error) {
                console.log(error)
                await navigateTo('/login')
            }
        }
        else {
            await navigateTo('/login')
        }
    }
    catch (error) {
        console.log(error)
        await navigateTo('/login')
    }
}

</script>

<template>
    <NuxtRouteAnnouncer/>
    <NuxtLayout>
        <NuxtPage />
    </NuxtLayout>
</template>
