<script lang="ts">
	export let label: string;
	export let tooltip: string;
	export let initial: any;
	export let meta: any;

	$: x_val = initial.x;
	$: y_val = initial.y;
	$: z_val = initial.z;
</script>

<span class="flex flex-row items-center justify-stretch gap-1">
	<p class="mr-1 text-surface-200 w-1/5">{label}</p>
	<div class="flex w-4/5 flex-row gap-1">
		<input class="webray-input input text-center text-surface-300" type="number" placeholder="X" bind:value={x_val}/>
		<input class="webray-input input text-center text-surface-300" type="number" placeholder="Y" bind:value={y_val}/>
		<input class="webray-input input text-center text-surface-300" type="number" placeholder="Z" bind:value={z_val}/>
	</div>
</span>
