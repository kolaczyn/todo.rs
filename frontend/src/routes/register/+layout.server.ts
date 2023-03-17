import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies }) => {
	const jwt = cookies.get('jwt');
	// TODO should probably check the expiration date
	return {
		isLoggedIn: !!jwt
	};
};
