import type {Character} from "~/types/character"
import type {Reactive} from "vue";

export const useSessionStore = defineStore('sessionStore', () => {

    const characters: Reactive<Record<string, Character>> = reactive({})

    function reset(): void {
        for (const key of Object.keys(characters)) {
            // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
            delete characters[key]
        }
    }

    return { characters, reset }

})