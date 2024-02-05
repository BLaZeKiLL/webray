<script lang="ts">
	import { onMount } from 'svelte';
	import { initialize_kernel } from '$lib/wasm/webray';
	import { initialize_editor } from '../lib/editor';

	onMount(() => {
		initialize_editor();

		initialize_kernel();
	});
</script>

<img id="output-image-target" alt="webray render output" class="max-h-full" />
