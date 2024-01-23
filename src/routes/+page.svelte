<script lang="ts">
	import { onMount } from 'svelte';
    import { init, run } from '$lib/wasm/webray';

    onMount(() => {
        init();
        run(); // this doesn't block, I guess cause we use wasm_bindgen_futures
    });
</script>

<img width="1280" height="720" id="output-image-target" alt="webray render output">
