<script lang="ts">
	import type { UserDto } from '../../types';
	import Cookies from 'js-cookie';
	import AuthForm from './AuthForm.svelte';
	import { goto } from '$app/navigation';

	const handleSubmit = async (email: string, password: string) => {
		const url = 'http://localhost:8080/v1/auth/register';
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
		Cookies.set('jwt', jwt);
		goto('/');
	};
</script>

<h1 class="is-size-2">Register Form</h1>
<AuthForm {handleSubmit} />
