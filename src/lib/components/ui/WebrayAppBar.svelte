<script lang="ts">
    import { AppBar } from '@skeletonlabs/skeleton';
</script>

<AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
	<svelte:fragment slot="lead">
        File
    </svelte:fragment>
	<span>WebRay</span>
	<svelte:fragment slot="trail">
        GitHub
    </svelte:fragment>
</AppBar>