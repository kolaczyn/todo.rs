<script>
	import { isAuthorized } from '$lib/common/auth/isAuthorized';
	import AppLayout from '$lib/layout/AppLayout.svelte';
	import Todos from '$lib/todos/Todos.svelte';
</script>

<AppLayout>
	{#if isAuthorized()}
		<Todos />
	{:else}
		<h1>You can't access the app, unless you <a href="/register">register</a></h1>
	{/if}
</AppLayout>
