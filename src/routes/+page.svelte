<script lang="ts">
	import { onMount } from 'svelte';
	import { initialize_kernel } from '$lib/wasm/webray';
	import { initialize_editor } from '../lib/editor';
	import { editorStore } from '../lib/store/editor.store';
	import { KernelState } from '../lib/types';
	import { ProgressRadial } from '@skeletonlabs/skeleton';

	onMount(() => {
		initialize_editor();

		initialize_kernel();
	});
</script>

<img
	id="output-image-target"
	alt="webray render output"
	class="max-h-full"
	class:hidden={$editorStore.kernel_state !== KernelState.DONE}
/>

{#if $editorStore.kernel_state === KernelState.RENDERING}
	<div class="max-h-full">
		<ProgressRadial meter="stroke-primary-500" track="stroke-primary-500/30" />
	</div>
{:else if $editorStore.kernel_state === KernelState.INITIAL}
	<div class="max-h-full">Press the image icon to render an image</div>
{/if}

<style>
</style>
