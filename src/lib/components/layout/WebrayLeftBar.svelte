<script lang="ts">
	import WebrayField from '../fields/WebrayField.svelte';
	import WebrayDrawer from '../drawer/WebrayDrawer.svelte';

	import { WebrayEditor, ID } from '../../editor';
	
	import WebrayIconButton from '../ui/WebrayIconButton.svelte';

	const sceneWindow = WebrayEditor.getWindow(ID.w_scene);
</script>

<div class="flex h-full flex-col border-r border-surface-600">
	<WebrayDrawer>
		<span slot="header" class="flex flex-row">
			<WebrayIconButton icon={sceneWindow.icon} tooltip={sceneWindow.tooltip}/>
		</span>

		{#each Array.from({ length: 30 }) as _, i}
			<WebrayField />
		{/each}

		<span slot="footer"> + </span>
	</WebrayDrawer>
</div>
