<script lang="ts">
	import WebrayField from '../fields/WebrayField.svelte';
	import WebrayDrawer from '../drawer/WebrayDrawer.svelte';

	import { WebrayEditor, ID, Icons } from '../../editor';

	const sceneWindow = WebrayEditor.getWindow(ID.w_scene);
	const sceneIcon = Icons.getIcon(sceneWindow.icon);
</script>

<div class="flex h-full flex-col border-r border-surface-600">
	<WebrayDrawer>
		<span slot="header" class="flex flex-row">
			<svelte:component this={sceneIcon} style="font-size: 1.5rem"/>
		</span>

		{#each Array.from({ length: 30 }) as _, i}
			<WebrayField />
		{/each}

		<span slot="footer"> + </span>
	</WebrayDrawer>
</div>
