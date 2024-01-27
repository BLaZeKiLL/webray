<script lang="ts">
	export let label: string;
	export let tooltip: string;
	export let initial: any;
	export let meta: any;

	$: colorValue = initial;
</script>

<span class="flex flex-row items-center justify-stretch gap-1">
	<p class="mr-1 text-surface-200 w-1/5">{label}</p>
	<div class="w-4/5 flex flex-row items-center justify-stretch gap-2">
		<input
			class="webray-input input text-center text-surface-300"
			type="color"
			bind:value={colorValue}
		/>
		<input
			class="webray-input input grow text-center text-surface-300"
			type="text"
			bind:value={colorValue}
			readonly
			tabindex="-1"
			placeholder="#hex"
		/>
	</div>
</span>
