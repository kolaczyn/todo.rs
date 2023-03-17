import Cookies from 'js-cookie';
import { writable } from 'svelte/store';

const createJwt = () => {
	const { subscribe, set } = writable<string | null>(null);

	// wrapper around svelete store, keeping Cookies and store in sync
	const setAuth = (newJwt: string | null) => {
		if (newJwt === null) {
			Cookies.remove('jwt');
			set(null);
		} else {
			Cookies.set('jwt', newJwt);
			set(newJwt);
		}
	};

	return {
		subscribe,
		setAuth
	};
};

export const jwtStore = createJwt();
