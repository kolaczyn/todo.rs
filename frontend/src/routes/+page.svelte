<script>
	import AppLayout from '$lib/layout/AppLayout.svelte';
	import { jwtStore } from '$lib/store/jwtStore';
	import Todos from '$lib/todos/Todos.svelte';

	export let data;
</script>

<AppLayout isLoggedIn={data.isLoggedIn}>
	{#if $jwtStore}
		<Todos />
	{:else}
		<h1>You can't access the app, unless you <a href="/register">register</a></h1>
	{/if}
</AppLayout>
