<script lang="ts">
	import WebrayDataView from '../WebrayDataView.svelte';

    export let label: string;
	export let tooltip: string;
	export let initial: any;
    export let meta: {options: { label: string; data_type: string }[]};

	let data_type_value: string = meta.options[0].data_type;
</script>

<div class="flex flex-col">
	<span class="flex flex-row items-center justify-stretch gap-1">
		<p class="mr-1 text-surface-200 w-1/5">{label}</p>
		<select class="select webray-input w-4/5 text-surface-300" bind:value={data_type_value}>
			{#each meta.options as option}
				<option value={option.data_type}>{option.label}</option>
			{/each}
		</select>
	</span>

	<div class="mt-2">
        <WebrayDataView data_type={data_type_value} card_type="variant-filled-surface"/>
    </div>
</div>
