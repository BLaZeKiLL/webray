<script lang="ts">
	import WebrayToolbar from '../toolbar/WebrayToolbar.svelte';

	import { WebrayEditor, ID } from '../../editor';
	import WebrayIcon from '../ui/WebrayIcon.svelte';

	const imageToolbar = WebrayEditor.getToolbar(ID.t_image_bar);
</script>

<WebrayToolbar>
	<div slot="lead" class="btn-group variant-ghost-surface rounded-none">
		{#each imageToolbar.lead as tool}
			<button class="!p-2">
				<WebrayIcon icon={tool.icon} tooltip={tool.tooltip} />
			</button>
		{/each}
	</div>
	<div slot="trail" class="btn-group variant-ghost-surface rounded-none">
		{#each imageToolbar.trail as tool}
			<button class="!p-2">
				<WebrayIcon icon={tool.icon} tooltip={tool.tooltip} />
			</button>
		{/each}
	</div>
</WebrayToolbar>
