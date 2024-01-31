<script lang="ts">
	import type { WebrayProperty } from "../../../editor";

	export let property: WebrayProperty;
	export let bind_path: string;

	$: value = property.initial;
</script>

<span class="flex flex-row items-center justify-stretch gap-1">
	<p class="mr-1 w-1/5 text-surface-200">{property.label}</p>
	<input
		class="webray-input input w-4/5 text-center text-surface-300"
		type="text"
		placeholder="name"
		bind:value
	/>
</span>
