<script lang="ts">
	import WebrayToolbar from '../toolbar/WebrayToolbar.svelte';

	import { WebrayEditor, ID, Icons } from '../../editor';
	import WebrayIconButton from '../ui/WebrayIconButton.svelte';

	const imageToolbar = WebrayEditor.getToolbar(ID.t_image_bar);
	const renderIcon = Icons.getIcon(imageToolbar.lead[0].icon);
	const downloadIcon = Icons.getIcon(imageToolbar.trail[0].icon);
</script>

<WebrayToolbar>
	<span slot="lead">
		{#each imageToolbar.lead as tool}
			<WebrayIconButton icon={tool.icon} tooltip={tool.tooltip} />
		{/each}
	</span>
	<span slot="trail">
		{#each imageToolbar.trail as tool}
			<WebrayIconButton icon={tool.icon} tooltip={tool.tooltip} />
		{/each}
	</span>
</WebrayToolbar>
