<script lang="ts">
	import type { WebrayProperty } from "../../../editor";
	import binder from "$lib/store/binder.store";
	import { derived } from "svelte/store";
	import type { vec3f } from "../../../types";

	export let property: WebrayProperty;
	export let bind_path: string;

	const store = binder.bind<vec3f>(bind_path, property.name)!;

	const x_val = derived(store, val => val.x);
	const y_val = derived(store, val => val.y);
	const z_val = derived(store, val => val.z);
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
