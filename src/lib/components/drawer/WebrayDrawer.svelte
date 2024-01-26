<script lang="ts">
	import { Tab, TabGroup } from '@skeletonlabs/skeleton';
	import { WebrayEditor } from '../../editor';
	
	import WebrayIconButton from '../ui/WebrayIconButton.svelte';
	import WebrayWindow from '../window/WebrayWindow.svelte';

	export let windows: string[];

	const _windows = windows.map((x) => WebrayEditor.getWindow(x));

	let tab: number = 0;
</script>

<TabGroup
	class="h-full"
	active="variant-filled-primary"
	hover="hover:variant-soft-primary"
	regionPanel="region-height !my-2 px-2 flex flex-col gap-4 snap-y snap-mandatory overflow-y-scroll scroll-smooth"
	padding="p-1"
	rounded=""
	border=""
	
>
	{#each _windows as win, i}
		<Tab bind:group={tab} name={win.icon} value={i}>
			<WebrayIconButton icon={win.icon} tooltip={win.tooltip} />
		</Tab>
	{/each}

	<svelte:fragment slot="panel">
		<WebrayWindow win={_windows[tab]}/>
	</svelte:fragment>
</TabGroup>