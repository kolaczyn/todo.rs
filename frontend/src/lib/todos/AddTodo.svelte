<script lang="ts">
	import { getJwt } from '$lib/common/auth/getJwt';
	import type { Todo } from '../../types';

	export let refetchCallback: () => Promise<void>;
	let value = '';
	let isLoading = false;

	const handleClick = async () => {
		const endpoint: string = 'http://localhost:8080';

		isLoading = true;
		const response: Todo = await fetch(`${endpoint}/v1/todos`, {
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
