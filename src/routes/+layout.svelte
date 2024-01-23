<script>
	import '../app.pcss';
	import { AppShell } from '@skeletonlabs/skeleton';
</script>

<AppShell slotPageContent="overflow-y-auto" slotSidebarLeft="w-2/12" slotSidebarRight="w-2/12">
	<svelte:fragment slot="header">
		Header
	</svelte:fragment>

	<svelte:fragment slot="sidebarLeft">
		Sidebar Left
	</svelte:fragment>
	
	<svelte:fragment slot="sidebarRight">
		Sidebar Right
	</svelte:fragment>
	<!-- (pageHeader) -->
	<!-- Router Slot -->
	<slot />
	<!-- ---- / ---- -->
	<svelte:fragment slot="pageFooter">
		Page Footer
	</svelte:fragment>
	<!-- (footer) -->
</AppShell>