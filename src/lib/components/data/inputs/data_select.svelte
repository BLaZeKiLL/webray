<script lang="ts">
	import type { WebrayProperty } from '../../../editor';
	import WebrayDataView from '../WebrayDataView.svelte';
	import binder from "$lib/store/binder.store";

	export let property: WebrayProperty;
	export let bind_path: string;

	export let prop_prefix: string; // this will be 

	const prop_path = prop_prefix === '' ? property.name : `${prop_prefix}.${property.name}`;

	const store = binder.bind<string>(bind_path, prop_path)!;

	console.log($store);

	const meta: { options: { label: string; value: string }[] } = property.meta;
</script>

<div class="flex flex-col">
	<span class="flex flex-row items-center justify-stretch gap-1">
		<p class="mr-1 w-1/5 text-surface-200">{property.label}</p>
		<select class="webray-input select w-4/5 text-surface-300" bind:value={$store}>
			{#each meta.options as option}
				<option value={option.value}>{option.label}</option>
			{/each}
		</select>
	</span>

	<div class="mt-2">
		<WebrayDataView data_type={$store} {bind_path} prop_prefix={prop_path} card_type="variant-filled-surface" />
	</div>
</div>