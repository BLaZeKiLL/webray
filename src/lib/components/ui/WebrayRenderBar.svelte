<script lang="ts">
	import WebrayToolbar from '../toolbar/WebrayToolbar.svelte';

	import Download from '~icons/uil/image-download';
	import Render from '~icons/material-symbols/rocket-launch';
</script>

<WebrayToolbar>
	<span slot="lead">
		<Render style="font-size: 1.5rem" />
	</span>
	<span slot="trail">
		<Download style="font-size: 1.5rem" />
	</span>
</WebrayToolbar>
