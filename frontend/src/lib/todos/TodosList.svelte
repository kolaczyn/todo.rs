<script lang="ts">
	import { PUBLIC_API_URL } from '$env/static/public';
	import { jwtStore } from '$lib/store/jwtStore';
	import type { Category, Todo } from '../../types';

	export let todos: Todo[] = [];
	export let categories: Category[] = [];
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

	$: completedNum = todos.reduce((acc, todo) => (todo.completed ? acc + 1 : acc), 0);
	$: notCompletedNum = todos.reduce((acc, todo) => (!todo.completed ? acc + 1 : acc), 0);

	const deleteTodo = async (id: number) => {
		await fetch(`${PUBLIC_API_URL}/v1/todos/${id}`, {
			method: 'DELETE',
			headers: {
				Authorization: `Bearer ${$jwtStore}`
			}
		}).then((x) => x.json());

		await refetchCallback();
	};

	const setTodoCategory = async (todoId: number, categoryId: number | null) => {
		await fetch(`${PUBLIC_API_URL}/v1/todos/assign-to-category/${todoId}`, {
			method: 'PATCH',
			headers: {
				Authorization: `Bearer ${$jwtStore}`
			},
			body: JSON.stringify({
				category_id: categoryId
			})
		}).then((x) => x.json());
		await refetchCallback();
	};

	const getCategoriesOfTodo = (todo: Todo) =>
		categories.find((x) => x.id === todo.category_id) ?? null;
</script>

<div>
	<h2>Stats</h2>
	<span>completed {completedNum}</span>
	<span>not completed: {notCompletedNum}</span>
</div>
{#each todos as todo (todo.id)}
	<div class="py-4">
		<div class={todo.completed ? 'is-crossed-out' : ''}>{todo.label}</div>
		<div>
			<!-- TODO this is very unoptimized, will have to fix this later -->
			{#if getCategoriesOfTodo(todo)}
				<div>
					<div style={`color: ${getCategoriesOfTodo(todo)?.color}`}>
						{getCategoriesOfTodo(todo)?.label}
					</div>
					<button on:click={() => setTodoCategory(todo.id, null)} class="button is-danger"
						>Remove category</button
					>
				</div>
			{:else}
				<div>
					{#each categories as category (category.id)}
						<button class="button is-small" on:click={() => setTodoCategory(todo.id, category.id)}
							>{category.label}</button
						>
					{/each}
				</div>
			{/if}
		</div>
		<button class="button" on:click={() => setTodoCompletedStatus(todo.id, !todo.completed)}
			>Mark as {todo.completed ? 'not completed' : 'completed'}</button
		>
		<button class="button is-danger" on:click={() => deleteTodo(todo.id)}>Delete</button>
	</div>
{/each}
