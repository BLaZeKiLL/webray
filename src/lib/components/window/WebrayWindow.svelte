<script lang="ts">
	import { ID, WebrayEditor, type WebrayWindow } from '../../editor';
	import WebrayDataView from '../data/WebrayDataView.svelte';
	import WebrayListDataView from '../data/WebrayListDataView.svelte';

	export let win: WebrayWindow;
</script>

<div class="flex h-full flex-col gap-4">
	<div class="flex grow snap-y snap-mandatory flex-col gap-4 overflow-y-auto scroll-smooth px-2">
		{#if win.data.type === 'list'}
			<WebrayListDataView data_type={win.data.data_type} bind_path={win.data.binding} />
		{:else}
			<WebrayDataView data_type={win.data.data_type} bind_path={win.data.binding} />
		{/if}
	</div>

	{#if win.data.type === 'list'}
		<button
			type="button"
			on:click={() =>
				WebrayEditor.invokeAction(ID.a_add_list_item, { bind_path: win.data.binding })}
			class="variant-ghost-primary btn mx-2">Add</button
		>
	{/if}
</div>
