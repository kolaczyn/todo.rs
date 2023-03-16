<script lang="ts">
	import AppInput from './AppInput.svelte';

	export let handleSubmit: (email: string, password: string) => Promise<unknown>;

	let email = '';
	let password = '';
	let isLoading = false;

	const handleClick = async () => {
		isLoading = true;
		await handleSubmit(email, password);
		isLoading = false;
	};
</script>

<AppInput bind:value={email} label="email" id="email" />
<AppInput bind:value={password} label="password" id="password" />

<div class="is-flex is-justify-content-space-between">
	<div />
	<button disabled={isLoading} class="button is-secondary" on:click={handleClick}>Submit</button>
</div>
