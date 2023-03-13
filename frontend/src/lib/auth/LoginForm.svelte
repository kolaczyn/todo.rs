<script lang="ts">
	import type { UserDto } from '../../types';

	let email = '';
	let password = '';
	let jwt: null | string = null;

	const fetchLogin = async () => {
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
		return response;
	};

	const handleClick = async () => {
		const response = await fetchLogin();
		jwt = response.jwt;
	};
</script>

<b>Login form</b>
<input bind:value={email} type="email" />
<input bind:value={password} type="password" />
<button on:click={handleClick}>Login</button>
<div>
	{jwt ? `Your jwt is ${jwt}` : "You don't have a jwt"}
</div>
