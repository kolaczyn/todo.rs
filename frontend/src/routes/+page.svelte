<script lang="ts">
	import { onMount } from 'svelte';
	import type { Category, Todo } from '../types';

	const endpoint: string = 'http://localhost:8080';

	let todos: Todo[] = [];
	let categories: Category[] = [];

	const fetchTodos = async () => {
		const response: Todo[] = await fetch(`${endpoint}/v1/todos`).then((x) => x.json());
		todos = response;
	};

	const fetchCategories = async () => {
		const response: Category[] = await fetch(`${endpoint}/v1/categories`).then((x) => x.json());
		categories = response;
	};

	onMount(async () => {
		fetchTodos();
		fetchCategories();
	});
</script>

<h1>Welcome to the Todo App!</h1>
<h2>Todos:</h2>
{#each todos as todo}
	<li>{todo.id} {todo.label}</li>
{/each}
<h3>Categories:</h3>
{#each categories as category}
	<li style={`color: ${category.color};`}>{category.id} {category.label}</li>
{/each}
