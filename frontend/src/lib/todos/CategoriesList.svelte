<script lang="ts">
	import type { Category } from '../../types';

	export let categories: Category[] = [];
</script>

{#each categories as category}
	<li style={`color: ${category.color};`}>{category.id} {category.label}</li>
{/each}
