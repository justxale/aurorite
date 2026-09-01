import type {Reactive} from "vue";

export const useInitiativeStore = defineStore('initiative', () => {

    const order: Reactive<Record<string, number>> = reactive({})

    function reset() : void {
        for (const key of Object.keys(order)) {
            // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
            delete order[key]
        }
    }

    return { order, reset }
})