<script lang="ts">
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
	import { Icons } from '../../editor';

	export let icon;
	export let tooltip: string;

	const _icon = Icons.getIcon(icon);
	const target = `tooltip_${icon}`;

	const tooltipPopup: PopupSettings = {
		event: 'hover',
		placement: 'bottom',
		target
	};
</script>

<span use:popup={tooltipPopup}>
	<svelte:component this={_icon} style="font-size: 1.5rem" />
</span>

<div class="card variant-filled-secondary p-1" data-popup={target}>
	{tooltip}
</div>
