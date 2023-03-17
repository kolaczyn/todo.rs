<script>
	import RegisterForm from '../../lib/auth/RegisterForm.svelte';
	import AppLayout from '../../lib/layout/AppLayout.svelte';

	export let data;
</script>

<AppLayout isLoggedIn={data.isLoggedIn}>
	<RegisterForm />
</AppLayout>
