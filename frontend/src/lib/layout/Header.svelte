<script lang="ts">
	export let isLoggedIn = false;
</script>

<header class="p-3 has-background-black-bis has-text-centered has-text-white">
	<div class="container is-flex is-justify-content-space-between">
		<a href="/" class="has-text-weight-bold">TodoApp</a>
		<nav>
			{#if isLoggedIn}
				<a href="/logout">Logout</a>
			{:else}
				<a href="/login">Login</a>
				<a href="/register" class="ml-2">Register</a>
			{/if}
		</nav>
	</div>
</header>
