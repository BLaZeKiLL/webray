<script lang="ts">
	import { AppBar } from '@skeletonlabs/skeleton';
</script>

<AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
	<svelte:fragment slot="lead">
		<span class="h4">File</span>
	</svelte:fragment>
	<span class="h2">WebRay</span>
	<svelte:fragment slot="trail">
		<span class="h4">GitHub</span>
	</svelte:fragment>
</AppBar>
