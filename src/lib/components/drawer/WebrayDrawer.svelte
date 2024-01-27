<script lang="ts">
	import { Tab, TabGroup } from '@skeletonlabs/skeleton';
	import { WebrayEditor } from '../../editor';

	import WebrayIcon from '../ui/WebrayIcon.svelte';
	import WebrayWindow from '../window/WebrayWindow.svelte';

	export let windows: string[];

	const _windows = windows.map((x) => WebrayEditor.getWindow(x));

	let tab: number = 0;
</script>

<TabGroup
	class="h-full"
	active="variant-ghost-primary"
	hover="hover:variant-soft-primary"
	regionPanel="region-height !my-2"
	padding="p-2"
	rounded=""
>
	{#each _windows as win, i}
		<Tab bind:group={tab} name={win.icon} value={i}>
			<WebrayIcon icon={win.icon} tooltip={win.tooltip} />
		</Tab>
	{/each}

	<svelte:fragment slot="panel">
		<WebrayWindow win={_windows[tab]} />
	</svelte:fragment>
</TabGroup>
