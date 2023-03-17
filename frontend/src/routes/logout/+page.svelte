<script>
	import AppLayout from '$lib/layout/AppLayout.svelte';
	import { jwtStore } from '$lib/store/jwtStore';
	import { onMount } from 'svelte';

	export let data;

	onMount(() => {
		setTimeout(() => {
			jwtStore.setAuth(null);
			window.location.reload();
		}, 1000);
	});
</script>

<svelte:head>
	<title>Todo App | Logout</title>
</svelte:head>
<AppLayout isLoggedIn={data.isLoggedIn}>
	{#if $jwtStore}
		<h1 class="is-size-1 has-text-centered">Logging out. Please wait a second...</h1>
	{:else}
		<h1 class="is-size-1 has-text-centered">You are logged out</h1>
	{/if}
</AppLayout>
