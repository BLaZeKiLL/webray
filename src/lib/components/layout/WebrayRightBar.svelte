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
		<WebrayDrawer windows={[ID.w_camera]} />
	</div>
	<hr />
	<div class="h-1/2">
		<WebrayDrawer windows={[ID.w_render_settings]} />
	</div>
</div>
