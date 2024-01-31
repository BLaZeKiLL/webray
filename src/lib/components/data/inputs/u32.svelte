<script lang="ts">
	import type { WebrayProperty } from "../../../editor";

	export let property: WebrayProperty;
	export let bind_path: string;

	$: value = property.initial;

	function validator(node: HTMLInputElement, _: number) {
		return {
			update(val: string) {
				value = Math.max(parseInt(node.min), Math.floor(parseFloat(val)));
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
		use:validator={value}
		bind:value
		placeholder="u32"
	/>
</span>
