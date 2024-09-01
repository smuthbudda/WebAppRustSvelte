import { APIClient } from '$lib/ApiClient.js';
import type { LoggedInUserDetails } from '$lib/types.js';
import { fail, redirect } from '@sveltejs/kit';

/** @type {import('./$types').PageServerLoad} */
export async function load({ parent }) {
	// const { user } = await parent();

	// if (user) 
    //     redirect(307, '/');
}

/** @type {import('./$types').Actions} */
export const actions = {
	default: async ({ cookies, request }) => {
		const data = await request.formData();


			let username : string = data.get('username')?.toString() ?? "";
			let email = data.get('email')?.toString() ?? ";"
            let first_name = data.get('first_name')?.toString() ?? "";
            let last_name =  data.get('last_name')?.toString() ?? "";
            let phone = data.get('phone')?.toString() ?? "";
			let password = data.get('password')?.toString() ?? "";


        let api = new APIClient();
		const body = await api.createNewUser(username, first_name, last_name, email, phone, password);

		if (body[0] == 200) {

		}

		// const value = btoa(JSON.stringify(body.user));
		// cookies.set('jwt', value, { path: '/' });

		redirect(307, '/');
	}
};