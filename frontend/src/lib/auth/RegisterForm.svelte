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

<div class="field is-horizontal">
	<div class="field-label is-normal">
		<label for="email" class="label has-text-white">Email</label>
	</div>
	<div class="field-body">
		<div class="field">
			<p class="control">
				<input
					bind:value={email}
					id="email"
					class="input"
					type="email"
					placeholder="Recipient email"
				/>
			</p>
		</div>
	</div>
</div>

<div class="field is-horizontal">
	<div class="field-label is-normal">
		<label for="password" class="label has-text-white">Password</label>
	</div>
	<div class="field-body">
		<div class="field">
			<p class="control">
				<input bind:value={password} id="password" class="input" type="password" />
			</p>
		</div>
	</div>
</div>

<!-- <div class="container is-flex is-justify-content-space-between"> -->
<div class="is-flex is-justify-content-space-between">
	<div />
	<button class="button is-secondary" on:click={handleClick}>Register</button>
</div>
{#if jwt}
	<div>
		Your jwt is {jwt}. Redirecting to login page in 2 seconds...
	</div>
{/if}
