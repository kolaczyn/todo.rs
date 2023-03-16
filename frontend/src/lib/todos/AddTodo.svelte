<script lang="ts">
	import { PUBLIC_API_URL } from '$env/static/public';
	import { getJwt } from '$lib/common/auth/getJwt';
	import type { Todo } from '../../types';

	export let refetchCallback: () => Promise<void>;
	let value = '';
	let isLoading = false;

	const handleClick = async () => {
		isLoading = true;
		await fetch(`${PUBLIC_API_URL}/v1/todos`, {
			method: 'POST',
			headers: {
				Authorization: `Bearer ${getJwt()}`
			},
			body: JSON.stringify({
				label: value
			})
		}).then((x) => x.json());
		value = '';
		isLoading = false;
		await refetchCallback();
	};
</script>

<input bind:value disabled={isLoading} />
<button disabled={isLoading} on:click={handleClick}>Add Todo</button>
