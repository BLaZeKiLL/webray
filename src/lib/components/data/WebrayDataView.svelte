<script lang="ts">
	import { WebrayEditor } from '../../editor';
	import WebrayProperty from './WebrayProperty.svelte';

	export let data_type: string;
	export let bind_path: string;
	export let card_type: string = 'bg-initial';
	export let prop_prefix: string = '';

	$: _data_type = WebrayEditor.getDataType(data_type);
</script>

{#if _data_type.properties.length !== 0}
	<div class="card flex snap-start flex-col gap-2 p-2 {card_type}">
		{#each _data_type.properties as property}
			<WebrayProperty {property} {bind_path} {prop_prefix}/>
		{/each}
	</div>
{/if}
