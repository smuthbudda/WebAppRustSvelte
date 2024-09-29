// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
import type {APIClient} from "$lib/ApiClient";

declare global {
	namespace App {
		// interface Error {}
		interface Locals { 
			user: LoggedInUserDetails,
			cookie: string //Dont do this
		}
		interface PageData { }
		// interface PageState {}
		// interface Platform {}
	}
}

export { };
