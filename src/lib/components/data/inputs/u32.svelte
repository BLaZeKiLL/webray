<script lang="ts">
	export let label: string;

	let n: number | null = null;
	let previousN: number | null = n;

	function validator(node: HTMLInputElement, _: number | null) {
		return {
			update(value: string | null) {
				n = value === null || n! < parseInt(node.min) ? previousN : parseInt(value);
				previousN = n;
			}
		};
	}
</script>

<span class="flex flex-row items-center justify-stretch gap-1">
	<p class="mr-1 text-surface-200">{label}</p>
	<input
		class="webray-input input text-center text-surface-300"
		type="number"
		min="0"
		use:validator={n} bind:value={n}
		placeholder="u32"
	/>
</span>
