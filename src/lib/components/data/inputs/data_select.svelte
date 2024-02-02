<script lang="ts">
	import type { WebrayProperty } from '../../../editor';
	import WebrayDataView from '../WebrayDataView.svelte';

	export let property: WebrayProperty;
	export let bind_path: string;

	const meta: { options: { label: string; value: string }[] } = property.meta;

	$: data_type_value = property.initial;
</script>

<div class="flex flex-col">
	<span class="flex flex-row items-center justify-stretch gap-1">
		<p class="mr-1 w-1/5 text-surface-200">{property.label}</p>
		<select class="webray-input select w-4/5 text-surface-300" bind:value={data_type_value}>
			{#each meta.options as option}
				<option value={option.value}>{option.label}</option>
			{/each}
		</select>
	</span>

	<div class="mt-2">
		<WebrayDataView data_type={data_type_value} bind_path="" card_type="variant-filled-surface" />
	</div>
</div>