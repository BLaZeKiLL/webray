<script lang="ts">
	import { Tab, TabGroup } from '@skeletonlabs/skeleton';
	import { WebrayEditor } from '../../editor';
	import WebrayField from '../fields/WebrayField.svelte';
	import WebrayIconButton from '../ui/WebrayIconButton.svelte';

	export let windows: string[];

	const _windows = windows.map((x) => WebrayEditor.getWindow(x));

	let tab: number = 0;
</script>

<TabGroup
	active="variant-filled-primary"
	hover="hover:variant-soft-primary"
	flex="flex-1 lg:flex-none"
	rounded=""
	padding="p-1"
	border=""
>
	{#each _windows as win, i}
		<Tab bind:group={tab} name={win.icon} value={i}>
			<WebrayIconButton icon={win.icon} tooltip={win.tooltip} />
		</Tab>
	{/each}

	<svelte:fragment slot="panel"></svelte:fragment>
</TabGroup>

<div class="flex max-h-full flex-col py-2">
	<div
		class="my-2 flex snap-y snap-mandatory scroll-py-4 flex-col gap-4 overflow-y-scroll scroll-smooth px-2"
	>
		{#each Array.from({ length: 20 }) as _, i}
			<WebrayField />
		{/each}
	</div>
</div>
