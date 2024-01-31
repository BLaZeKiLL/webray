<script lang="ts">
	import type { WebrayProperty } from "../../../editor";

	export let property: WebrayProperty;
	export let bind_path: string;

	$: x_val = property.initial.x;
	$: y_val = property.initial.y;
	$: z_val = property.initial.z;
</script>

<span class="flex flex-row items-center justify-stretch gap-1">
	<p class="mr-1 w-1/5 text-surface-200">{property.label}</p>
	<div class="flex w-4/5 flex-row gap-1">
		<input
			class="webray-input input text-center text-surface-300"
			type="number"
			placeholder="X"
			bind:value={x_val}
		/>
		<input
			class="webray-input input text-center text-surface-300"
			type="number"
			placeholder="Y"
			bind:value={y_val}
		/>
		<input
			class="webray-input input text-center text-surface-300"
			type="number"
			placeholder="Z"
			bind:value={z_val}
		/>
	</div>
</span>
