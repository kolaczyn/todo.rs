<script>
	import { isAuthorized } from '$lib/common/auth/isAuthorized';
	import AppLayout from '$lib/layout/AppLayout.svelte';
	import Cookies from 'js-cookie';

	setTimeout(() => {
		if (isAuthorized()) {
			Cookies.remove('jwt');
			window.location.reload();
		}
	}, 1000);
</script>

<AppLayout>
	{#if isAuthorized()}
		<h1 class="is-size-1 has-text-centered">Logging out. Please wait a second...</h1>
	{:else}
		<h1 class="is-size-1 has-text-centered">You are logged out</h1>
	{/if}
</AppLayout>
