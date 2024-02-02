<script lang="ts">
	import WebrayToolbar from '../toolbar/WebrayToolbar.svelte';

	import { WebrayEditor, ID } from '../../editor';

	const toolbar = WebrayEditor.getToolbar(ID.t_image_bar);
</script>

<WebrayToolbar {toolbar}/>
