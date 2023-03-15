<script>
	import Footer from './Footer.svelte';
	import Header from './Header.svelte';
</script>

<div class="has-background-black-ter has-text-white is-flex is-flex-direction-column full-height">
	<Header />
	<div class="is-flex-grow-1">
		<div class="container my-4">
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
