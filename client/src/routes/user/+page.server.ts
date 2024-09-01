import { APIClient } from "$lib/ApiClient";
import type { UpdateUserRequest } from "$lib/types.js";

/** @type {import('./$types').PageServerLoad} */
export async function load({ locals }) {
    return {
        user_name: locals.user.user_name,
        email: locals.user.email,
        first_name: locals.user.first_name,
        last_name: locals.user.last_name,
        phone: locals.user.phone,
        password: ""
    };
}

/** @type {import('./$types').Actions} */
export const actions = {
    default: async ({ cookies, request, locals }) => {
        const data = await request.formData();
        let user: UpdateUserRequest = {
            user_name: data.get('user_name') as string,
            email: data.get('email') as string,
            first_name: data.get('first_name') as string,
            last_name: data.get('last_name') as string,
            phone: data.get('phone') as string,
        };
        console.log(user);
        let apiClient = new APIClient();
        let result = await apiClient.updateMyDetails(cookies.get('session') ?? "", user, locals.user.id);
        // Handle form submission logic here
        if (result[1]) {
            locals.user = result[1]
        }
        console.log(result);
    }
};