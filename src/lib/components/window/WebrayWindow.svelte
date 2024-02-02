<script lang="ts">
	import type { WebrayWindow } from '../../editor';
	import WebrayDataView from '../data/WebrayDataView.svelte';
	import WebrayListDataView from '../data/WebrayListDataView.svelte';

	export let win: WebrayWindow;
</script>

<div class="flex h-full flex-col gap-4">
	<div class="flex grow snap-y snap-mandatory flex-col gap-4 overflow-y-scroll scroll-smooth px-2">
		{#if win.data.type === 'list'}
			<WebrayListDataView data_type={win.data.data_type} bind_path={win.data.binding} />
		{:else}
			<WebrayDataView data_type={win.data.data_type} bind_path={win.data.binding} />
		{/if}
	</div>

	{#if win.data.type === 'list'}
		<button type="button" class="variant-ghost-primary btn mx-2">Add</button>
	{/if}
</div>
