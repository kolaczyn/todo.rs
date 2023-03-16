<script lang="ts">
	import type { UserDto } from '../../types';
	import Cookies from 'js-cookie';
	import AuthForm from './AuthForm.svelte';

	const handleSubmit = async (email: string, password: string) => {
		const url = 'http://localhost:8080/v1/auth/login';
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
	};
</script>

<h1 class="is-size-2">Login Form</h1>
<AuthForm {handleSubmit} />
