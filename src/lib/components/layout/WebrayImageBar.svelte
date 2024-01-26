<script lang="ts">
	import WebrayToolbar from '../toolbar/WebrayToolbar.svelte';

	import { WebrayEditor, ID, Icons } from '../../editor';

	const imageToolbar = WebrayEditor.getToolbar(ID.t_image_bar);
	const renderIcon = Icons.getIcon(imageToolbar.lead[0].icon);
	const downloadIcon = Icons.getIcon(imageToolbar.trail[0].icon);
</script>

<WebrayToolbar>
	<span slot="lead">
		<svelte:component this={renderIcon} style="font-size: 1.5rem"/>
	</span>
	<span slot="trail">
		<svelte:component this={downloadIcon} style="font-size: 1.5rem"/>
	</span>
</WebrayToolbar>
