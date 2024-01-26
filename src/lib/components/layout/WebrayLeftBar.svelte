<script lang="ts">
	import WebrayDrawer from '../drawer/WebrayDrawer.svelte';

	import { WebrayEditor, ID } from '../../editor';
</script>

<div class="flex h-full flex-col border-r border-surface-600">
	<WebrayDrawer windows={[ID.w_scene, ID.w_materials]} />
</div>
