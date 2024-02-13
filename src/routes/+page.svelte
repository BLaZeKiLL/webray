<script lang="ts">
	import { onMount } from 'svelte';
	import { initialize_kernel } from '$lib/wasm/webray';
	import { ID, WebrayEditor, initialize_editor } from '../lib/editor';
	import { editorStore } from '../lib/store/editor.store';
	import { KernelState, OutputView } from '../lib/types';
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import WebrayIcon from '../lib/components/ui/WebrayIcon.svelte';

	onMount(() => {
		initialize_editor();

		initialize_kernel();
	});

	const invoke = (action: string) => {
		WebrayEditor.invokeAction(action);
	};
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
	<div class="h4 flex max-h-full flex-row items-center gap-2">
		Press <WebrayIcon icon="i_render" tooltip="" /> to render an image
	</div>
{/if}

{#if $editorStore.output_view === OutputView.FULLSCREEN}
	<button class="top_right" on:click={() => invoke(ID.a_full_screen_exit)}>
		<WebrayIcon icon="i_full_screen_exit" tooltip="Exit fullscreen" />
	</button>
{/if}

<style>
	.top_right {
		position: absolute;
		top: 16px;
		right: 32px;
	}
</style>
