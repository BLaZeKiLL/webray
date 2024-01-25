<script>
	import '../app.pcss';

	import { AppShell } from '@skeletonlabs/skeleton';

	import WebrayLeftBar from '$lib/components/ui/WebrayLeftBar.svelte';
	import WebrayRightBar from '$lib/components/ui/WebrayRightBar.svelte';
	import WebrayAppBar from '$lib/components/ui/WebrayAppBar.svelte';
	import WebrayRenderBar from '$lib/components/ui/WebrayRenderBar.svelte';
	import WebrayFooter from '$lib/components/ui/WebrayFooter.svelte';
</script>

<AppShell
	slotPageContent="overflow-y-auto flex flex-col items-center bg-gray-950"
	slotSidebarLeft="w-1/6 overflow-y-clip"
	slotSidebarRight="w-1/6 overflow-y-clip"
	slotPageFooter="h-1/6"
>
	<svelte:fragment slot="header">
		<WebrayAppBar />
	</svelte:fragment>

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
	<!-- (footer) -->
</AppShell>
