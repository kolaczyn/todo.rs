<script lang="ts">
	import { PUBLIC_API_URL } from '$env/static/public';
	import { jwtStore } from '$lib/store/jwtStore';
	import { onMount } from 'svelte';
	import type { Category, Todo } from '../../types';
	import AddTodo from './AddTodo.svelte';
	import CategoriesList from './CategoriesList.svelte';
	import TodosList from './TodosList.svelte';

	let todos: Todo[] = [];
	let categories: Category[] = [];

	const fetchTodos = async () => {
		const response: Todo[] = await fetch(`${PUBLIC_API_URL}/v1/todos`, {
			headers: {
				Authorization: `Bearer ${$jwtStore}`
			}
		}).then((x) => x.json());
		todos = response;
	};

	const fetchCategories = async () => {
		const response: Category[] = await fetch(`${PUBLIC_API_URL}/v1/categories`, {
			headers: {
				Authorization: `Bearer ${$jwtStore}`
			}
		}).then((x) => x.json());
		categories = response;
	};

	onMount(async () => {
		fetchTodos();
		fetchCategories();
	});
</script>

<h1 class="is-size-2">Welcome to the Todo App!</h1>
<AddTodo refetchCallback={fetchTodos} />
<TodosList {todos} refetchCallback={fetchTodos} />
<CategoriesList {categories} />
