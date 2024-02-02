<script lang="ts">
	import type { WebrayProperty } from "../../../editor";
	import binder from "$lib/store/binder.store";

	export let property: WebrayProperty;
	export let prop_prefix: string;
	export let bind_path: string;

	const prop_path = prop_prefix === '' ? property.name : `${prop_prefix}.${property.name}`;

	const store = binder.bind<number>(bind_path, prop_path)!;

	function validator(node: HTMLInputElement, _: number) {
		return {
			update(val: string) {
				$store = Math.max(parseInt(node.min), Math.floor(parseFloat(val)));
			}
		};
	}
</script>

<span class="flex flex-row items-center justify-stretch gap-1">
	<p class="mr-1 w-1/5 text-surface-200">{property.label}</p>
	<input
		class="webray-input input w-4/5 text-center text-surface-300"
		type="number"
		min="0"
		use:validator={$store}
		bind:value={$store}
		placeholder="u32"
	/>
</span>
