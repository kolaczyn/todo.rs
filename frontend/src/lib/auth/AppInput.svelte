<script lang="ts">
	export let value: string;
	export let label: string;
	export let id: string;
</script>

<div class="field is-horizontal">
	<div class="field-label is-normal">
		<label for={id} class="label has-text-white">{label}</label>
	</div>
	<div class="field-body">
		<div class="field">
			<p class="control">
				<input bind:value {id} class="input" {...$$restProps} />
			</p>
		</div>
	</div>
</div>
