<script>
	import Cookies from 'js-cookie';
	import { jwtStore } from '$lib/store/jwtStore';
	import { onMount } from 'svelte';
	import Footer from './Footer.svelte';
	import Header from './Header.svelte';
	onMount(() => {
		const jwt = Cookies.get('jwt') ?? null;
		jwtStore.setAuth(jwt);
	});
</script>

<div class="has-background-black-ter has-text-white is-flex is-flex-direction-column full-height">
	<Header />
	<div class="is-flex-grow-1">
		<div class="container my-4 px-4">
			<slot />
		</div>
	</div>
	<Footer />
</div>

<style>
	.full-height {
		min-height: 100vh;
	}
</style>
