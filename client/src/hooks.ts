import { APIClient } from '$lib/ApiClient';
import { redirect, type Handle } from '@sveltejs/kit'

const unProtectedRoutes: string[] = [
	'/',
	'/login',
	'/createAdmin',
	'/features',
	'/docs',
	'/deployment'
];

export const handle: Handle = async ({ event, resolve }) => {
	// get cookies from browser
	const session = event.cookies.get('session')
	if (!session) {

		console.log("No session cookie found");
		// if there is no session load page as normal
		if (!unProtectedRoutes.includes(event.url.pathname))
			return redirect(303, 'login');
	}
	// console.log("Session from the cookie: " + session);
	if (!session) {
		// if there is no session load page as normal
		if (event.url.pathname != '/login') {
			return redirect(303,'/login')
		}
		return resolve(event);
	}
	let apiClient = new APIClient();
	// find the user based on the session
	const user = await apiClient.getMyDetails(session);
	// if `user` exists set `events.local`
	if (user[1] ) { 
		event.locals.user = user[1];
		event.locals.cookie = session
	}

	// load page as normal
	return resolve(event);
}