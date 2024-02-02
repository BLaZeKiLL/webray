<script lang="ts">
	import type { WebrayProperty } from '../../../editor';
	import scene from '$lib/store/scene.store';

	export let property: WebrayProperty;
	export let prop_prefix: string;
	export let bind_path: string;

	const prop_path = prop_prefix === '' ? property.name : `${prop_prefix}.${property.name}`;

	const store = scene.bind<string>(bind_path, prop_path)!;
</script>

<span class="flex flex-row items-center justify-stretch gap-1">
	<p class="mr-1 w-1/5 text-surface-200">{property.label}</p>
	<input
		class="webray-input input w-4/5 text-center text-surface-300"
		type="text"
		placeholder="name"
		bind:value={$store}
	/>
</span>
