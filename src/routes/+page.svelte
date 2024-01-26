<script lang="ts">
	import { onMount } from 'svelte';
	import { init, run } from '$lib/wasm/webray';

	onMount(() => {
		init();
		// this doesn't block, I guess cause we use wasm_bindgen_futures
		// could try returning a promise
		run();
	});
</script>

<img id="output-image-target" alt="webray render output" class="max-h-full" />
