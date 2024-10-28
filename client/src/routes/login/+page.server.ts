import { redirect } from '@sveltejs/kit';

import { APIClient } from "$lib/ApiClient";
let apiClient = new APIClient();

/** @type {import('./$types').PageServerLoad} */
export async function load({ locals }) {
	// if (locals.user) 
	//     redirect(307, '/');
}

/** @type {import('./$types').Actions} */
export const actions = {
	default: async ({ cookies, request }) => {
		const data = await request.formData();

		let email = data.get('email')?.toString() ?? "";
		let password = data.get('password')?.toString() ?? "";
		let response = await apiClient.userLogin(email, password);
		// console.log("Logged in: " + response[1].access_token);
		if (response)
			if (response[1]) {
				console.log("Logged in: " + response[1].access_token);
				cookies.set('session', response[1].access_token, { path: '/', maxAge: 60 * 60 * 24 * 7 },);
			}

		redirect(307, '/');
	}
};