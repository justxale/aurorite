export const useAuthenticationStore = defineStore('authentication', () => {
    const token = useCookie('access_token')
    const username = ref('')
    const displayName = ref('')

    async function fetchToken(login: string, password: string) {
        const data = await $fetch<{ access_token: string }>('http://localhost:11811/client/auth/login', {
                method: 'POST',
                body: {login, password},
            }
        )

        token.value = data.access_token
    }

    async function fetchUser() {
        const userInfo = await $fetch<{
            username: string,
            display_name: string,}>(
                'http://localhost:11811/client/me', {
            headers: {Authorization: `Bearer ${token.value}`}
        })

        username.value = userInfo.username
        displayName.value = userInfo.display_name
    }

    return {fetchToken, fetchUser, username, displayName}
})