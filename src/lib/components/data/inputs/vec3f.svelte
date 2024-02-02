<script lang="ts">
	import type { WebrayProperty } from '../../../editor';
	import scene from '$lib/store/scene.store';
	import type { vec3f } from '../../../types';
	import { writable_derived } from '../../../store/writable-derived.store';

	export let property: WebrayProperty;
	export let prop_prefix: string;
	export let bind_path: string;

	const prop_path = prop_prefix === '' ? property.name : `${prop_prefix}.${property.name}`;

	const store = scene.bind<vec3f>(bind_path, prop_path)!;

	const x_val = writable_derived(store, 'x');
	const y_val = writable_derived(store, 'y');
	const z_val = writable_derived(store, 'z');
</script>

<span class="flex flex-row items-center justify-stretch gap-1">
	<p class="mr-1 w-1/5 text-surface-200">{property.label}</p>
	<div class="flex w-4/5 flex-row gap-1">
		<input
			class="webray-input input text-center text-surface-300"
			type="number"
			placeholder="X"
			bind:value={$x_val}
		/>
		<input
			class="webray-input input text-center text-surface-300"
			type="number"
			placeholder="Y"
			bind:value={$y_val}
		/>
		<input
			class="webray-input input text-center text-surface-300"
			type="number"
			placeholder="Z"
			bind:value={$z_val}
		/>
	</div>
</span>
