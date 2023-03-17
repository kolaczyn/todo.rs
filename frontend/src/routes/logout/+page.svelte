<script>
	import AppLayout from '$lib/layout/AppLayout.svelte';
	import { jwtStore } from '$lib/store/jwtStore';

	setTimeout(() => {
		jwtStore.subscribe((jwt) => {
			if (jwt) window.location.reload();
		});
	}, 1000);
</script>

<AppLayout>
	{#if $jwtStore}
		<h1 class="is-size-1 has-text-centered">Logging out. Please wait a second...</h1>
	{:else}
		<h1 class="is-size-1 has-text-centered">You are logged out</h1>
	{/if}
</AppLayout>
