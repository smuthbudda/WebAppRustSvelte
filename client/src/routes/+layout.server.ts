/** @type {import('./$types').LayoutServerLoad} */
export function load({ locals, cookies }) {
	return {
		user: locals.user
	};
}

