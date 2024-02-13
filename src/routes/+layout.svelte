<script>
	import '../app.pcss';

	import { AppShell, Toast, getToastStore } from '@skeletonlabs/skeleton';
	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	import { initializeStores } from '@skeletonlabs/skeleton';

	import WebrayLeftBar from '$lib/components/layout/WebrayLeftBar.svelte';
	import WebrayRightBar from '$lib/components/layout/WebrayRightBar.svelte';
	import WebrayRenderBar from '$lib/components/layout/WebrayImageBar.svelte';
	import WebrayFooter from '$lib/components/layout/WebrayFooter.svelte';
	import { editorStore } from '../lib/store/editor.store';
	import { OutputView } from '../lib/types';

	initializeStores();

	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	editorStore.set_toaster(getToastStore());
</script>

<Toast />

<AppShell
	slotSidebarLeft="w-1/6 overflow-y-clip"
	slotSidebarRight="w-1/6 overflow-y-clip"
	slotPageFooter="h-1/6"
>
	<svelte:fragment slot="sidebarLeft">
		<WebrayLeftBar />
	</svelte:fragment>

	<svelte:fragment slot="sidebarRight">
		<WebrayRightBar />
	</svelte:fragment>

	<svelte:fragment slot="pageHeader">
		<WebrayRenderBar />
	</svelte:fragment>

	<!-- Router Slot -->
	<span
		class:fullscreen={$editorStore.output_view === OutputView.FULLSCREEN}
		class="flex h-full flex-col items-center justify-center overflow-y-auto bg-gray-950"
	>
		<slot />
	</span>
	<!-- ---- / ---- -->

	<svelte:fragment slot="pageFooter">
		<WebrayFooter />
	</svelte:fragment>
</AppShell>

<style>
	.fullscreen {
		position: absolute;
		left: 0;
		right: 0;
		top: 0;
		bottom: 0;
		z-index: 10;
	}
</style>
