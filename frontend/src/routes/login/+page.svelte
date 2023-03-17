<script>
	import LoginForm from '../../lib/auth/LoginForm.svelte';
	import AppLayout from '../../lib/layout/AppLayout.svelte';

	export let data;
</script>

<svelte:head>
	<title>Todo App | Login</title>
</svelte:head>
<AppLayout isLoggedIn={data.isLoggedIn}>
	<LoginForm />
</AppLayout>
