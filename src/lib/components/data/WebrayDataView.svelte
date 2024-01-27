<script lang="ts">
	import { WebrayEditor } from '../../editor';
	import WebrayProperty from './WebrayProperty.svelte';

	export let data_type: string;
    export let card_type: string = 'bg-initial';
</script>

<div class="card flex snap-start flex-col gap-2 p-2 {card_type}">
	{#each WebrayEditor.getDataType(data_type).properties as property}
		<WebrayProperty
			label={property.label}
			type={property.type}
			tooltip={property.tooltip}
			initial={property.initial}
			meta={property.meta}
		/>
	{/each}
</div>
