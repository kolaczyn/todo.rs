<script lang="ts">
	import { goto } from '$app/navigation';
	import type { UserDto } from '../../types';

	let email = '';
	let password = '';
	let jwt: null | string = null;

	const fetchRegister = async () => {
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
		return response;
	};

	const handleClick = async () => {
		const response = await fetchRegister();
		jwt = response.jwt;

		setTimeout(() => {
			goto('/login');
		}, 2_000);
	};
</script>

<b>Register form</b>
<input bind:value={email} type="email" />
<input bind:value={password} type="password" />
<button on:click={handleClick}>Register</button>
<div>
	{jwt ? `Your jwt is ${jwt}. Redirecting to login page in 2 seconds...` : "You don't have a jwt"}
</div>
