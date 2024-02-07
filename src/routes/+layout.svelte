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

	initializeStores();
	
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	editorStore.set_toaster(getToastStore());
</script>

<Toast />

<AppShell
	slotPageContent="overflow-y-auto flex flex-col items-center justify-center bg-gray-950"
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
	<slot />
	<!-- ---- / ---- -->

	<svelte:fragment slot="pageFooter">
		<WebrayFooter />
	</svelte:fragment>
</AppShell>
