<script lang="ts">
	import type { UserDto } from '../../types';
	import AuthForm from './AuthForm.svelte';
	import { goto } from '$app/navigation';
	import { PUBLIC_API_URL } from '$env/static/public';
	import { jwtStore } from '$lib/store/jwtStore';

	const handleSubmit = async (email: string, password: string) => {
		const url = `${PUBLIC_API_URL}/v1/auth/login`;
		const response: UserDto = await fetch(url, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				email,
				password
			})
		}).then((x) => x.json());
		const jwt = response.jwt;
		jwtStore.setAuth(jwt);
		goto('/');
	};
</script>

<h1 class="is-size-2">Login Form</h1>
<AuthForm {handleSubmit} />
