<script>
	import LoginForm from '../../lib/auth/LoginForm.svelte';
	import AppLayout from '../../lib/layout/AppLayout.svelte';

	export let data;
</script>

<AppLayout isLoggedIn={data.isLoggedIn}>
	<LoginForm />
</AppLayout>
