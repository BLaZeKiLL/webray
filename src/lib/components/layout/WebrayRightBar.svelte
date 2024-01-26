<script lang="ts">
	import WebrayField from '../fields/WebrayField.svelte';
	import WebrayDrawer from '../drawer/WebrayDrawer.svelte';

	import { WebrayEditor, ID, Icons } from '../../editor';

	const cameraWindow = WebrayEditor.getWindow(ID.w_camera);
	const cameraIcon = Icons.getIcon(cameraWindow.icon);

	const renderSettingsWindow = WebrayEditor.getWindow(ID.w_render_settings);
	const renderSettingsIcon = Icons.getIcon(renderSettingsWindow.icon);
</script>

<div class="flex h-full flex-col border-l border-surface-600">
	<div class="h-1/2">
		<WebrayDrawer>
			<h2 slot="header">
				<svelte:component this={cameraIcon} style="font-size: 1.5rem"/>
			</h2>

			{#each Array.from({ length: 10 }) as _, i}
				<WebrayField />
			{/each}
		</WebrayDrawer>
	</div>
	<hr />
	<div class="h-1/2">
		<WebrayDrawer>
			<h2 slot="header">
				<svelte:component this={renderSettingsIcon} style="font-size: 1.5rem"/>
			</h2>

			{#each Array.from({ length: 10 }) as _, i}
				<WebrayField />
			{/each}
		</WebrayDrawer>
	</div>
</div>
