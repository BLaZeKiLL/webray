<script lang="ts">
	export let label: string;

	let colorValue: string;
</script>

<span class="flex flex-row items-center justify-stretch gap-1">
	<p class="mr-1 text-surface-200">{label}</p>
	<div class="flex grow flex-row gap-2">
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
