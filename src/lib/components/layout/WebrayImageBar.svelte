<script lang="ts">
	import WebrayToolbar from '../toolbar/WebrayToolbar.svelte';

	import { WebrayEditor, ID } from '../../editor';
	import WebrayIconButton from '../ui/WebrayIconButton.svelte';

	const imageToolbar = WebrayEditor.getToolbar(ID.t_image_bar);
</script>

<WebrayToolbar>
	<svelte:fragment slot="lead">
		{#each imageToolbar.lead as tool}
			<WebrayIconButton icon={tool.icon} tooltip={tool.tooltip} />
		{/each}
	</svelte:fragment>
	<svelte:fragment slot="trail">
		{#each imageToolbar.trail as tool}
			<WebrayIconButton icon={tool.icon} tooltip={tool.tooltip} />
		{/each}
	</svelte:fragment>
</WebrayToolbar>
