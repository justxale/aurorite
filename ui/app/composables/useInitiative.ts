import { useSessionStore } from "~/stores/sessionStore";

export function useTurn() {
    const sessionStore = useSessionStore()

    const turn = computed(() => {
        return sessionStore.characters
    })

    return { turn }

}