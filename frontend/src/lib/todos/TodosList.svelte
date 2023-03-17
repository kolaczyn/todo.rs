<script lang="ts">
	import { PUBLIC_API_URL } from '$env/static/public';
	import { jwtStore } from '$lib/store/jwtStore';
	import type { Todo } from '../../types';

	export let todos: Todo[] = [];
	export let refetchCallback: () => Promise<void>;

	const setTodoCompletedStatus = async (id: number, newCompleted: boolean) => {
		await fetch(`${PUBLIC_API_URL}/v1/todos/${id}`, {
			method: 'PATCH',
			headers: {
				Authorization: `Bearer ${$jwtStore}`
			},
			body: JSON.stringify({
				completed: newCompleted
			})
		}).then((x) => x.json());
		await refetchCallback();
	};

	const deleteTodo = async (id: number) => {
		await fetch(`${PUBLIC_API_URL}/v1/todos/${id}`, {
			method: 'DELETE',
			headers: {
				Authorization: `Bearer ${$jwtStore}`
			}
		}).then((x) => x.json());

		await refetchCallback();
	};
</script>

{#each todos as todo}
	<div class="py-4">
		<div class={todo.completed ? 'is-crossed-out' : ''}>{todo.label}</div>
		<button class="button" on:click={() => setTodoCompletedStatus(todo.id, !todo.completed)}
			>Mark as {todo.completed ? 'not completed' : 'completed'}</button
		>
		<button class="button is-danger" on:click={() => deleteTodo(todo.id)}>Delete</button>
	</div>
{/each}
