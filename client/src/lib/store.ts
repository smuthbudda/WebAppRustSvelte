
import { type Readable, writable } from "svelte/store"
import type { LoggedInUserDetails, State } from "./types"

const initialState: LoggedInUserDetails = { user_name: "", id: 1 ,first_name: "", last_name: "", email: "", phone: "", password : "123123"}
export type MyStore = Readable<State> & {
    setDetails: (user_details: LoggedInUserDetails) => void,
}

function createStore(): MyStore {
    const { subscribe, update } = writable<State>(initialState)

    return {
        subscribe,
        setDetails: (user_details: LoggedInUserDetails) => {
            update((state) => {
                return {
                    ...state,
                    ...user_details
                }
            })
        }
    }
}

export const userDetails = createStore()
