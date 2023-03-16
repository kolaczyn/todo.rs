<script lang="ts">
	import { getJwt } from '$lib/common/auth/getJwt';
	import type { Todo } from '../../types';

	export let todos: Todo[] = [];
	export let refetchCallback: () => Promise<void>;

	const setTodoCompletedStatus = async (id: number, newCompleted: boolean) => {
		await fetch(`http://localhost:8080/v1/todos/${id}`, {
			method: 'PATCH',
			headers: {
				Authorization: `Bearer ${getJwt()}`
			},
			body: JSON.stringify({
				completed: newCompleted
			})
		}).then((x) => x.json());
		await refetchCallback();
	};

	const deleteTodo = async (id: number) => {
		await fetch(`http://localhost:8080/v1/todos/${id}`, {
			method: 'DELETE',
			headers: {
				Authorization: `Bearer ${getJwt()}`
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
