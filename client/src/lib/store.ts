
import { type Readable, writable } from "svelte/store"
import type { State } from "./types"

const initialState: State = { username: "", id: ""}

export type MyStore = Readable<State> & { setName: (username: string) => void , setId: (id : string) => void
}

function createStore(): MyStore {
    const { subscribe, update } = writable<State>(initialState)

    return { subscribe, 
        setName: (username: string) => update((state) => ({ ...state, username })),
        setId: (id: string) => update((state) => ({ ...state, id })) }
}

export const userDetails = createStore()
