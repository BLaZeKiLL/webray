<script lang="ts">
	export let label: string;
	export let tooltip: string;
	export let initial: any;
	export let meta: any;
</script>

<span class="flex flex-row items-center justify-stretch gap-1">
	<p class="mr-1 text-surface-200 w-1/5">{label}</p>
	<div class="flex w-4/5 flex-row gap-1">
		<input class="webray-input input text-center text-surface-300" type="number" placeholder="X" />
		<input class="webray-input input text-center text-surface-300" type="number" placeholder="Y" />
		<input class="webray-input input text-center text-surface-300" type="number" placeholder="Z" />
	</div>
</span>
