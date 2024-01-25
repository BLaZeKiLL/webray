<script>
	import '../app.pcss';
	import { AppShell } from '@skeletonlabs/skeleton';
	import WebrayLeftBar from '../lib/components/ui/WebrayLeftBar.svelte';
	import WebrayRightBar from '../lib/components/ui/WebrayRightBar.svelte';
	import WebrayAppBar from '../lib/components/ui/WebrayAppBar.svelte';
	import WebrayRenderBar from '../lib/components/ui/WebrayRenderBar.svelte';
	import WebrayFooter from '../lib/components/ui/WebrayFooter.svelte';
</script>

<AppShell
	slotPageContent="overflow-y-auto"
	slotSidebarLeft="w-2/12 overflow-y-clip"
	slotSidebarRight="w-2/12 overflow-y-clip"
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
