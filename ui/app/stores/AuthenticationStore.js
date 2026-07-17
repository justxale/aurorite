export const useAuthenticationStore = defineStore('authentication', () => {
    const token = useCookie('access_token')
    const username = ref('')
    const displayName = ref('')

    async function authentication(login, password) {
        const data = await $fetch('http://localhost:11811/client/auth/login', {
                method: 'POST',
                body: {login, password},
            }
        )
        token.value = data.access_token
    }

    async function onStorage() {
        const userInfo = await $fetch('http://localhost:11811/client/me', {
            headers: {Authorization: `Bearer ${token.value}`}
        })
        console.log(userInfo)

        username.value = userInfo.username
        displayName.value = userInfo.display_name
    }

    return {authentication, onStorage, username, displayName}
})