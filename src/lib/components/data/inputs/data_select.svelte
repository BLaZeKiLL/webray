<script lang="ts">
	import { WebrayEditor, type WebrayProperty } from '../../../editor';
	import WebrayDataView from '../WebrayDataView.svelte';
	import scene from '$lib/store/scene.store';
	import { writable_derived } from '../../../store/writable-derived.store';
	import { tick } from 'svelte';

	export let property: WebrayProperty;
	export let bind_path: string;

	export let prop_prefix: string; // this will be empty most probably

	const meta: { options: { label: string; value: string }[] } = property.meta;

	const prop_path = prop_prefix === '' ? property.name : `${prop_prefix}.${property.name}`;

	const store = scene.bind(bind_path, prop_path, bind_path);

	// This will update type only other properties won't be reset, deterministic read depending on type
	// so old keys would persist, it is possible that union of keys of all types be present on the object
	// hence we use a validator to perform the reset
	const type = writable_derived<any, string>(store, 'type');

	function validator(_node: HTMLSelectElement, _val: string) {
		return {
			async update(val: string) {
				const initial = WebrayEditor.getDataType($type).properties.reduce((p, c) => {
					p[c.name] = c.initial;
					return p;
				}, {} as any);

				await tick();

				store.set({
					type: val,
					...initial
				});
			}
		};
	}
</script>

<div class="flex flex-col">
	<span class="flex flex-row items-center justify-stretch gap-1">
		<p class="mr-1 w-1/5 text-surface-200">{property.label}</p>
		<select
			class="webray-input select w-4/5 text-surface-300"
			bind:value={$type}
			use:validator={$type}
		>
			{#each meta.options as option}
				<option value={option.value}>{option.label}</option>
			{/each}
		</select>
	</span>

	<div class="mt-2">
		<WebrayDataView
			data_type={$type}
			{bind_path}
			prop_prefix={prop_path}
			card_type="variant-filled-surface"
		/>
	</div>
</div>
