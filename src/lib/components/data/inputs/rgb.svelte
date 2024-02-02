<script lang="ts">
	import type { WebrayProperty } from '../../../editor';
	import scene from '$lib/store/scene.store';

	export let property: WebrayProperty;
	export let prop_prefix: string;
	export let bind_path: string;

	const prop_path = prop_prefix === '' ? property.name : `${prop_prefix}.${property.name}`;

	const store = scene.bind<string>(bind_path, prop_path);
</script>

<span class="flex flex-row items-center justify-stretch gap-1">
	<p class="mr-1 w-1/5 text-surface-200">{property.label}</p>
	<div class="flex w-4/5 flex-row items-center justify-stretch gap-2">
		<input
			class="webray-input input text-center text-surface-300"
			type="color"
			bind:value={$store}
		/>
		<input
			class="webray-input input grow text-center text-surface-300"
			type="text"
			bind:value={$store}
			readonly
			tabindex="-1"
			placeholder="#hex"
		/>
	</div>
</span>
