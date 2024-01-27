<script lang="ts">
	import WebrayDrawer from '../drawer/WebrayDrawer.svelte';

	import { ID } from '../../editor';
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
