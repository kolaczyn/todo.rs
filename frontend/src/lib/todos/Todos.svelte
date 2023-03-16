<script lang="ts">
	import { onMount } from 'svelte';
	import type { Category, Todo } from '../../types';
	import { getJwt } from '../common/auth/getJwt';
	import AddTodo from './AddTodo.svelte';
	import CategoriesList from './CategoriesList.svelte';
	import TodosList from './TodosList.svelte';

	const endpoint: string = 'http://localhost:8080';

	let todos: Todo[] = [];
	let categories: Category[] = [];

	const fetchTodos = async () => {
		const response: Todo[] = await fetch(`${endpoint}/v1/todos`, {
			headers: {
				Authorization: `Bearer ${getJwt()}`
			}
		}).then((x) => x.json());
		todos = response;
	};

	const fetchCategories = async () => {
		const response: Category[] = await fetch(`${endpoint}/v1/categories`, {
			headers: {
				Authorization: `Bearer ${getJwt()}`
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
