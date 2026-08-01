export function changeFunc(){
    const currentTheme = useColorMode()
    const { locale, setLocale } = useI18n()

    function changeTheme() {
        currentTheme.preference = currentTheme.value === 'dark' ? 'light' : 'dark';
    }

    function changeLanguage(language: 'ru' | 'en') {
        setLocale(language)
    }

    return {
        locale, changeTheme, changeLanguage
    }
}
