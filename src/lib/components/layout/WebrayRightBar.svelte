<script lang="ts">
	import WebrayField from '../fields/WebrayField.svelte';
	import WebrayDrawer from '../drawer/WebrayDrawer.svelte';

	import { WebrayEditor, ID } from '../../editor';
	import WebrayIconButton from '../ui/WebrayIconButton.svelte';

	const cameraWindow = WebrayEditor.getWindow(ID.w_camera);
	const renderSettingsWindow = WebrayEditor.getWindow(ID.w_render_settings);
</script>

<div class="flex h-full flex-col border-l border-surface-600">
	<div class="h-1/2">
		<WebrayDrawer>
			<h2 slot="header">
				<WebrayIconButton icon={cameraWindow.icon} tooltip={cameraWindow.tooltip}/>
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
				<WebrayIconButton icon={renderSettingsWindow.icon} tooltip={renderSettingsWindow.tooltip}/>
			</h2>

			{#each Array.from({ length: 10 }) as _, i}
				<WebrayField />
			{/each}
		</WebrayDrawer>
	</div>
</div>
