<script>
	import RegisterForm from '../../lib/auth/RegisterForm.svelte';
	import AppLayout from '../../lib/layout/AppLayout.svelte';

	export let data;
</script>

<svelte:head>
	<title>Todo App | Register</title>
</svelte:head>
<AppLayout isLoggedIn={data.isLoggedIn}>
	<RegisterForm />
</AppLayout>
